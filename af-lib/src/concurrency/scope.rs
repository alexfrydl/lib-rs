// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Concurrency scope plumbing not intended for end users.

use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::AcqRel;

use rustc_hash::FxHashMap;

use super::runtime::{self, AsyncOp};
use super::{channel, future};
use crate::prelude::*;
use crate::util::{panic, Panic, SharedStr};

thread_local! {
  /// The currently running scope.
  static SCOPE: RefCell<Option<Arc<Scope>>> = default();
}

/// Returns a reference to the currently running scope.
pub fn current() -> Option<Arc<Scope>> {
  SCOPE.with(|cell| cell.borrow().clone())
}

/// Runs an async operation as a concurrency scope.
pub async fn run<O, F>(op: F) -> Result<(), Error>
where
  O: IntoOutput + 'static,
  F: Future<Output = O> + 'static,
{
  let (commands_tx, command_rx) = channel();
  let scope = Arc::new(Scope { next_child_id: AtomicUsize::new(1), commands: commands_tx });

  // Wrap the main future in a capture_panic() and normalize its output.

  let main_future = async move {
    match future::capture_panic(panic::AssertUnwindSafe(op)).await {
      Err(panic) => Err(Error::Panic(panic)),

      Ok(output) => match output.into_scope_output() {
        Ok(()) => Ok(()),
        Err(err) => Err(Error::Error(err)),
      },
    }
  };

  // Create a “controller” future that implements parent-child relationships.

  let controller = async move {
    // A map of IDs to children.
    let mut children: FxHashMap<usize, (Info, Option<AsyncOp>)> = default();
    // A list of events to signal when all children exit.
    let mut join_children: Vec<ArcWeak<event_listener::Event>> = default();

    // Handle each received command.

    while let Some(command) = command_rx.recv().await {
      match command {
        Command::RegisterChild(info) => {
          children.insert(info.id, (info, None));
        }

        Command::InsertChild { id, child } => {
          if let Some(entry) = children.get_mut(&id) {
            entry.1 = Some(child);
          }
        }

        Command::FinishChild { id, result } => {
          let entry = children.remove(&id);

          // If all children have finished, notify all join listeners.

          if children.is_empty() {
            for join in join_children.drain(..) {
              if let Some(event) = join.upgrade() {
                event.notify_relaxed(usize::MAX);
              }
            }
          }

          // If the child exited with an error, propagate it.

          if let Some((info, _)) = entry {
            if let Err(err) = result {
              return Err(Error::FromChild { child: info, error: Box::new(err) });
            }
          }
        }

        Command::JoinChildren(event) => {
          // Run a “garbage collection” pass on the list.

          join_children.retain(|j| j.strong_count() > 0);

          // If there are no children currently, notify this listener
          // immediately.

          if children.is_empty() {
            if let Some(event) = event.upgrade() {
              event.notify_relaxed(1);
              continue;
            }
          }

          // Otherwise store it.

          join_children.push(event);
        }
      }
    }

    unreachable!();
  };

  // Set up the scope and run the main future and controller future concurrently
  // until the main future exits or a child scope fails.

  let future = future::race(main_future, controller);
  let mut tmp = Some(scope);

  future::with_poll_fn(future, |this, cx| {
    SCOPE.with(|cell| mem::swap(&mut *cell.borrow_mut(), &mut tmp));

    defer! {
      SCOPE.with(|cell| mem::swap(&mut *cell.borrow_mut(), &mut tmp));
    }

    this.poll(cx)
  })
  .await
}

/// Runs an async operation as a scope by blocking the current thread.
pub fn run_sync<O, F>(op: F) -> Result<(), Error>
where
  O: IntoOutput + 'static,
  F: Future<Output = O> + 'static,
{
  runtime::block_on(run(op))
}

/// An error returned from a scope.
#[derive(From)]
pub enum Error {
  #[from]
  Error(String),
  #[from]
  Panic(Panic),
  FromChild {
    child: Info,
    error: Box<Error>,
  },
}

impl Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Error::Error(err) => {
        write!(f, "failed\n{}", fmt::indent("  ", "  ", err))
      }

      Error::Panic(panic) => write!(f, "{:#}", panic),

      Error::FromChild { child, error } => match child.name.as_str() {
        "" => write!(f, "failed\nbecause {} {} {}", child.kind, child.id, error),
        name => write!(f, "failed\nbecause {} {:?} {}", child.kind, name, error),
      },
    }
  }
}

/// Identifying info about a scope
pub struct Info {
  pub id: usize,
  pub kind: &'static str,
  pub name: SharedStr,
}

/// A trait which allows scopes to return `()` or `Result<(), String>`.
pub trait IntoOutput {
  fn into_scope_output(self) -> Result<(), String>;
}

impl IntoOutput for () {
  fn into_scope_output(self) -> Result<(), String> {
    Ok(())
  }
}

impl<E> IntoOutput for Result<(), E>
where
  E: Display,
{
  fn into_scope_output(self) -> Result<(), String> {
    match self {
      Ok(_) => Ok(()),
      Err(err) => Err(err.to_string()),
    }
  }
}

/// The name of a kind of scope; for example, "thread" or "task".
pub type Kind = &'static str;

/// One of the possible operations the scope controller can run.
enum Command {
  RegisterChild(Info),
  InsertChild { id: usize, child: AsyncOp },
  FinishChild { id: usize, result: Result<(), Error> },
  JoinChildren(ArcWeak<event_listener::Event>),
}

/// A running scope.
pub struct Scope {
  next_child_id: AtomicUsize,
  commands: channel::Sender<Command>,
}

impl Scope {
  /// Registers a child of this scope.
  ///
  /// The actual child must later be provided with [`insert_child()`],
  pub fn register_child(&self, kind: Kind, name: impl Into<SharedStr>) -> usize {
    let id = self.next_child_id.fetch_add(1, AcqRel);
    self.commands.send(Command::RegisterChild(Info { id, kind, name: name.into() }));
    id
  }

  /// Inserts a previously registered child.
  pub fn insert_child(&self, id: usize, child: AsyncOp) {
    self.commands.send(Command::InsertChild { id, child });
  }

  /// Runs an async operation as a child scope.
  pub fn run_child<O>(
    self: &Arc<Self>,
    id: usize,
    op: impl Future<Output = O> + 'static,
  ) -> impl Future<Output = ()>
  where
    O: IntoOutput + 'static,
  {
    let scope = Arc::downgrade(self);

    async move {
      let result = run(op).await;

      if let Some(scope) = scope.upgrade() {
        scope.commands.send(Command::FinishChild { id, result });
      }
    }
  }

  /// Waits for all children to exit.
  pub async fn join_children(&self) {
    let event = Arc::new(event_listener::Event::new());
    let listener = event.listen();

    self.commands.send(Command::JoinChildren(Arc::downgrade(&event)));

    listener.await;
  }
}
