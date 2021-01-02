use crate::prelude::*;
use crate::thread;
use async_executor::{Executor, Task};
use easy_parallel::Parallel;
use event_listener::Event;
use once_cell::sync::Lazy;

static EXECUTOR: Lazy<Executor> = Lazy::new(default);

pub struct JoinHandle<T> {
  task: Option<Task<T>>,
}

pub fn run<T>(future: impl Future<Output = T>) -> T {
  let ex = &*EXECUTOR;
  let shutdown = Event::new();
  let threads = num_cpus::get();

  let (_, result) = Parallel::new()
    // Run an executor thread per logical CPU core.
    .each(0..threads, |_| thread::block_on(ex.run(shutdown.listen())))
    // Run the main future on the current thread.
    .finish(|| thread::block_on(async {
      let result = future.await;
      shutdown.notify(threads);
      result
    }));

  result
}

pub fn spawn<T: Send + 'static>(future: impl Future<Output = T> + Send + 'static) -> JoinHandle<T> {
  let task = EXECUTOR.spawn(future);

  JoinHandle { task: Some(task) }
}

impl<T> JoinHandle<T> {
  pub async fn join(mut self) -> Result<T, future::PanicError> {
    let task = self.task.take().unwrap();

    future::catch_unwind(panic::AssertUnwindSafe(task)).await
  }
}

impl<T> Drop for JoinHandle<T> {
  fn drop(&mut self) {
    if let Some(task) = self.task.take() {
      task.detach();
    }
  }
}
