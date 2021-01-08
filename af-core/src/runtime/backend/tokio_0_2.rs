use crate::prelude::*;
use event_listener::Event;
use once_cell::sync::OnceCell;
use tokio::runtime::{Handle, Runtime};

static HANDLE: OnceCell<Handle> = OnceCell::new();

pub struct TaskHandle<T> {
  exit: Event,
  inner: tokio::task::JoinHandle<Result<T, future::CancelError>>,
}

pub fn run<T>(future: impl Future<Output = T>) -> T {
  let mut runtime = Runtime::new().expect("Failed to start tokio runtime");

  HANDLE.set(runtime.handle().clone()).unwrap();

  runtime.block_on(future)
}

pub async fn sleep(duration: Duration) {
  handle().enter(|| tokio::time::delay_for(duration.into())).await;
}

pub fn spawn<T: Send + 'static>(future: impl Future<Output = T> + Send + 'static) -> TaskHandle<T> {
  let exit = Event::new();
  let future = future::cancel_after(exit.listen(), future);

  TaskHandle { exit, inner: handle().spawn(future) }
}

pub async fn unblock<T: Send + 'static>(func: impl FnOnce() -> T + Send + 'static) -> T {
  handle().enter(|| tokio::task::spawn_blocking(func)).await.unwrap()
}

fn handle() -> &'static Handle {
  HANDLE.get().expect("The af-core runtime is not running.")
}

impl<T> TaskHandle<T> {
  pub async fn join(self) -> Result<T, future::PanicError> {
    self.inner.await.map(Result::unwrap).map_err(|err| err.into_panic().into())
  }

  pub async fn stop(self) -> Option<T> {
    self.exit.notify_relaxed(1);
    self.join().await.ok()
  }
}
