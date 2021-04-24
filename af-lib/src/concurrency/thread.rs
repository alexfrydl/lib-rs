use super::{channel, scope};
use crate::prelude::*;
use crate::util::SharedStr;

/// An executor which runs futures on a single thread.
type Executor = Rc<async_executor::LocalExecutor<'static>>;

thread_local! {
  /// A thread-specific executor.
  static EXECUTOR: RefCell<Option<Executor>> = default();
}

/// Returns a reference to the current thread's executor, if one exists.
pub(super) fn executor() -> Option<Executor> {
  EXECUTOR.with(|ex| ex.borrow().clone())
}

/// Creates an executor for the current thread and returns a reference to it.
fn init_executor() -> Executor {
  let executor = Executor::default();
  EXECUTOR.with(|ex| *ex.borrow_mut() = Some(executor.clone()));
  executor
}

/// Starts a new thread which runs a future to completion.
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
      let id = parent.register_child("thread", name);
      let future = parent.run_child(id, future);
      let (tx, rx) = channel::<()>();

      let child = executor.spawn(async move {
        let _tx = tx;

        future.await
      });

      parent.insert_child(id, child);

      async_io::block_on(executor.run(rx.recv()));
    })
    .expect("failed to spawn thread");
}

/// Runs a future to completion on the current thread, as though it were started
/// with [`start()`].
pub(super) fn run<F>(future: F) -> Result<(), scope::Error>
where
  F: Future<Output = Result> + 'static,
{
  let executor = init_executor();

  async_io::block_on(executor.run(scope::run(future)))
}
