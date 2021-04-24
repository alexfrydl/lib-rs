use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::AcqRel;

use rustc_hash::FxHashMap;

use crate::concurrency::{channel, Channel};
use crate::prelude::*;
use crate::string::SharedStr;

pub type Child = async_task::Task<()>;
pub type Result<T = (), E = Error> = std::result::Result<T, E>;

pub struct Scope {
  next_child_id: AtomicUsize,
  child_events: channel::Sender<ChildEvent>,
}

pub enum Kind {
  Thread,
  Fiber,
  Task,
}

pub struct Info {
  pub id: usize,
  pub kind: Kind,
  pub name: SharedStr,
}

pub enum Error {
  Error(String),
  Panic(Panic),
  FromChild { child: Info, error: Box<Error> },
}

enum Event {
  Created { kind: Kind, name: SharedStr },
  Started(Child),
  Finished(Result),
}

type ChildEvent = (usize, Event);

thread_local! {
  static SCOPE: RefCell<Option<Arc<Scope>>> = default();
}

pub fn current() -> Option<Arc<Scope>> {
  SCOPE.with(|cell| cell.borrow().clone())
}

pub async fn run<F>(future: F) -> Result
where
  F: Future<Output = failure::Result> + 'static,
{
  let child_events = Channel::new();

  let scope =
    Arc::new(Scope { next_child_id: AtomicUsize::new(1), child_events: child_events.sender() });

  let event_listener = async move {
    let mut children: FxHashMap<usize, (Info, Option<Child>)> = default();

    loop {
      let (child_id, event) = child_events.recv().await;

      match event {
        Event::Created { kind, name } => {
          children.insert(child_id, (Info { kind, id: child_id, name }, None));
        }

        Event::Started(child) => {
          if let Some(entry) = children.get_mut(&child_id) {
            entry.1 = Some(child);
          }
        }

        Event::Finished(result) => {
          if let Some((info, _)) = children.remove(&child_id) {
            if let Err(err) = result {
              return Err(Error::FromChild { child: info, error: Box::new(err) });
            }
          }
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
  pub fn create_child(&self, kind: Kind, name: impl Into<SharedStr>) -> usize {
    let id = self.next_child_id.fetch_add(1, AcqRel);
    self.child_events.send((id, Event::Created { kind, name: name.into() }));
    id
  }

  pub fn set_child(&self, id: usize, child: Child) {
    self.child_events.send((id, Event::Started(child)));
  }

  pub fn finish_child(&self, id: usize, result: Result) {
    self.child_events.send((id, Event::Finished(result)));
  }

  pub fn run_child<F>(self: &Arc<Self>, id: usize, future: F) -> impl Future<Output = ()>
  where
    F: Future<Output = failure::Result> + 'static,
  {
    let scope = Arc::downgrade(self);

    async move {
      let result = run(future).await;

      if let Some(scope) = scope.upgrade() {
        scope.finish_child(id, result);
      }
    }
  }
}

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

impl Display for Kind {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Kind::Thread => write!(f, "thread"),
      Kind::Fiber => write!(f, "fiber"),
      Kind::Task => write!(f, "task"),
    }
  }
}
