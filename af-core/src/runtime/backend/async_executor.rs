pub use blocking::unblock;

use crate::prelude::*;
use crate::thread;
use async_executor::{Executor, Task};
use async_io::Timer;
use easy_parallel::Parallel;
use event_listener::Event;
use once_cell::sync::Lazy;

static EXECUTOR: Lazy<Executor> = Lazy::new(default);

pub struct TaskHandle<T> {
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
      shutdown.notify_relaxed(threads);
      result
    }));

  result
}

pub async fn sleep(duration: Duration) {
  Timer::after(duration.into()).await;
}

pub fn spawn<T: Send + 'static>(future: impl Future<Output = T> + Send + 'static) -> TaskHandle<T> {
  let task = EXECUTOR.spawn(future);

  TaskHandle { task: Some(task) }
}

impl<T> TaskHandle<T> {
  pub async fn join(mut self) -> Result<T, future::PanicError> {
    let task = self.task.take().unwrap();

    future::catch_unwind(panic::AssertUnwindSafe(task)).await
  }

  pub async fn stop(mut self) -> Option<T> {
    self.task.take().unwrap().cancel().await
  }
}

impl<T> Drop for TaskHandle<T> {
  fn drop(&mut self) {
    if let Some(task) = self.task.take() {
      task.detach();
    }
  }
}
