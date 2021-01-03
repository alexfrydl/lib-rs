#![allow(unused_variables)]

use crate::prelude::*;

pub struct TaskHandle<T>(PhantomData<T>);

pub fn run<T>(future: impl Future<Output = T>) -> T {
  unimplemented()
}

pub async fn sleep(duration: Duration) {
  unimplemented()
}

pub fn spawn<T: Send + 'static>(future: impl Future<Output = T> + Send + 'static) -> TaskHandle<T> {
  unimplemented()
}

pub async fn unblock<T: Send + 'static>(func: impl FnOnce() -> T + Send + 'static) -> T {
  unimplemented()
}

impl<T> TaskHandle<T> {
  pub async fn join(self) -> Result<T, future::PanicError> {
    unimplemented()
  }

  pub async fn stop(self) -> Option<T> {
    unimplemented()
  }
}

fn unimplemented() -> ! {
  panic!(
    "The af-core runtime is not enabled. You must enable either the {:?} or {:?} features.",
    "runtime", "runtime-tokio"
  )
}
