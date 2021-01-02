use crate::prelude::*;
use once_cell::sync::OnceCell;
use tokio::runtime::{Handle, Runtime};

static HANDLE: OnceCell<Handle> = OnceCell::new();

pub struct JoinHandle<T>(tokio::task::JoinHandle<T>);

pub fn run<T>(future: impl Future<Output = T>) -> T {
  let mut runtime = Runtime::new().expect("Failed to start tokio runtime");

  HANDLE.set(runtime.handle().clone()).unwrap();

  runtime.block_on(future)
}

pub fn spawn<T: Send + 'static>(future: impl Future<Output = T> + Send + 'static) -> JoinHandle<T> {
  let handle = HANDLE.get().expect("The af-core runtime is not running.");

  JoinHandle(handle.spawn(future))
}

impl<T> JoinHandle<T> {
  pub async fn join(self) -> Result<T, future::PanicError> {
    self.0.await.map_err(|err| err.into_panic().into())
  }
}
