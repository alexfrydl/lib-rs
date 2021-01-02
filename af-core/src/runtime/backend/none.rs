use crate::prelude::*;

pub struct JoinHandle<T>(PhantomData<T>);

pub fn run<T>(future: impl Future<Output = T>) -> T {
  unimplemented()
}

pub fn spawn<T: Send + 'static>(future: impl Future<Output = T> + Send + 'static) -> JoinHandle<T> {
  unimplemented()
}

impl<T> JoinHandle<T> {
  pub async fn join(self) -> Result<T, future::PanicError> {
    unimplemented()
  }
}

fn unimplemented() -> ! {
  panic!(
    "The af-core runtime is not enabled. You must enable either the {:?} or {:?} features.",
    "runtime", "runtime-tokio"
  )
}
