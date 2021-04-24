pub(crate) use async_io::block_on;

use super::{channel, scope};
use crate::prelude::*;
use crate::string::SharedStr;

pub(crate) type Executor = Rc<async_executor::LocalExecutor<'static>>;

thread_local! {
  static EXECUTOR: RefCell<Option<Executor>> = default();
}

#[track_caller]
pub fn start<F>(name: impl Into<SharedStr>, future: F)
where
  F: Future<Output = Result> + Send + 'static,
{
  let parent = scope::current().expect("thread does not support child threads");
  let name = name.into();

  std::thread::Builder::new()
    .name(name.to_string())
    .spawn(move || {
      let executor = init_executor();
      let id = parent.create_child(scope::Kind::Thread, name);
      let future = parent.run_child(id, future);
      let (tx, rx) = channel::<()>();

      let child = executor.spawn(async move {
        let _tx = tx;

        future.await
      });

      parent.set_child(id, child);

      block_on(executor.run(rx.recv()));
    })
    .expect("failed to spawn thread");
}

pub(crate) fn executor() -> Option<Executor> {
  EXECUTOR.with(|ex| ex.borrow().clone())
}

fn init_executor() -> Executor {
  let executor = Executor::default();
  EXECUTOR.with(|ex| *ex.borrow_mut() = Some(executor.clone()));
  executor
}

pub(crate) fn run<F>(future: F) -> Result<(), scope::Error>
where
  F: Future<Output = Result> + 'static,
{
  let executor = init_executor();

  block_on(executor.run(scope::run(future)))
}
