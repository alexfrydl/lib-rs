use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::AcqRel;

use rustc_hash::FxHashMap;

use crate::concurrency::{channel, Channel};
use crate::prelude::*;
use crate::string::SharedStr;

/// The name of a kind of scope; for example, "thread" or "task".
pub type Kind = &'static str;

/// A child of a scope.
pub type Child = async_task::Task<()>;

/// The result of running a scope to completion.
type Result<T = (), E = Error> = std::result::Result<T, E>;

/// A running scope.
pub struct Scope {
  next_child_id: AtomicUsize,
  ops: channel::Sender<Op>,
}

/// Identifying info about a scope
pub struct Info {
  pub id: usize,
  pub kind: &'static str,
  pub name: SharedStr,
}

/// An error returned from a scope.
pub enum Error {
  Error(String),
  Panic(Panic),
  FromChild { child: Info, error: Box<Error> },
}

/// One of the possible operations the scope controller can run.
enum Op {
  RegisterChild(Info),
  InsertChild { id: usize, child: Child },
  FinishChild { id: usize, result: Result },
  JoinChildren(ArcWeak<event_listener::Event>),
}

thread_local! {
  /// The currently running scope.
  static SCOPE: RefCell<Option<Arc<Scope>>> = default();
}

/// Returns a reference to the currently running scope.
pub fn current() -> Option<Arc<Scope>> {
  SCOPE.with(|cell| cell.borrow().clone())
}

/// Runs a future as a concurrency scope.
pub async fn run<F>(future: F) -> Result
where
  F: Future<Output = failure::Result> + 'static,
{
  let ops = Channel::new();
  let scope = Arc::new(Scope { next_child_id: AtomicUsize::new(1), ops: ops.sender() });
  let mut join_children: Vec<ArcWeak<event_listener::Event>> = default();

  let event_listener = async move {
    let mut children: FxHashMap<usize, (Info, Option<Child>)> = default();

    loop {
      match ops.recv().await {
        Op::RegisterChild(info) => {
          children.insert(info.id, (info, None));
        }

        Op::InsertChild { id, child } => {
          if let Some(entry) = children.get_mut(&id) {
            entry.1 = Some(child);
          }
        }

        Op::FinishChild { id, result } => {
          let entry = children.remove(&id);

          if children.is_empty() {
            for join in join_children.drain(..) {
              if let Some(event) = join.upgrade() {
                event.notify(1);
              }
            }
          }

          if let Some((info, _)) = entry {
            if let Err(err) = result {
              return Err(Error::FromChild { child: info, error: Box::new(err) });
            }
          }
        }

        Op::JoinChildren(event) => {
          join_children.retain(|j| j.strong_count() > 0);
          join_children.push(event);
        }
      }
    }
  };

  let main_future = async move {
    match future::capture_panic(panic::AssertUnwindSafe(future)).await {
      Ok(Ok(_)) => Ok(()),
      Ok(Err(err)) => Err(Error::Error(err.to_string())),
      Err(panic) => Err(Error::Panic(panic)),
    }
  };

  future::with_tls_value(&SCOPE, scope.clone(), future::race(main_future, event_listener)).await
}

impl Scope {
  /// Registers a child of this scope.
  ///
  /// The actual child must later be provided with [`insert_child()`],
  pub fn register_child(&self, kind: Kind, name: impl Into<SharedStr>) -> usize {
    let id = self.next_child_id.fetch_add(1, AcqRel);
    self.ops.send(Op::RegisterChild(Info { id, kind, name: name.into() }));
    id
  }

  /// Inserts a previously registered child.
  pub fn insert_child(&self, id: usize, child: Child) {
    self.ops.send(Op::InsertChild { id, child });
  }

  /// Runs a future as a child scope.
  pub fn run_child<F>(self: &Arc<Self>, id: usize, future: F) -> impl Future<Output = ()>
  where
    F: Future<Output = failure::Result> + 'static,
  {
    let scope = Arc::downgrade(self);

    async move {
      let result = run(future).await;

      if let Some(scope) = scope.upgrade() {
        scope.ops.send(Op::FinishChild { id, result });
      }
    }
  }

  /// Waits for all children to exit.
  pub async fn join_children(&self) {
    let event = Arc::new(event_listener::Event::new());
    let listener = event.listen();

    self.ops.send(Op::JoinChildren(Arc::downgrade(&event)));

    listener.await;
  }
}

// Implement formatting for errors so that they can be used as part of a
// sentence. For example, "The main task {}" to describe why the main task
// failed.

impl Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Error::Error(err) => {
        write!(f, "failed\n{}", fmt::indent("  ", "  ", err))
      }

      Error::Panic(panic) => Display::fmt(panic, f),

      Error::FromChild { child, error } => match child.name.as_str() {
        "" => write!(f, "failed\nbecause {} {} {}", child.kind, child.id, error),
        name => write!(f, "failed\nbecause {} {:?} {}", child.kind, name, error),
      },
    }
  }
}
