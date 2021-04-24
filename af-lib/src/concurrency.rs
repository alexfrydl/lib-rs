pub mod channel;
pub mod fiber;
pub mod runtime;
mod scope;
pub mod thread;

pub use self::channel::{channel, Channel};

/// Yields once to pending concurrent operations.
pub async fn cooperative_yield() {
  futures_lite::future::yield_now().await
}

pub mod task {
  use super::{scope, thread};
  use crate::string::SharedStr;
  use crate::{env, prelude::*};

  type Executor = Arc<async_executor::Executor<'static>>;

  static EXECUTOR: Lazy<Executor> = Lazy::new(|| {
    let executor = Executor::default();

    for i in 0..env::num_cpus() + 1 {
      let executor = executor.clone();

      std::thread::Builder::new()
        .name(format!("task executor {}", i + 1))
        .spawn(move || {
          thread::block_on(executor.run(future::forever::<()>()));
        })
        .expect("failed to spawn task executor thread");
    }

    executor
  });

  #[track_caller]
  pub fn start<F>(name: impl Into<SharedStr>, future: F)
  where
    F: Future<Output = Result> + Send + 'static,
  {
    let parent = scope::current().expect("thread does not support tasks");
    let id = parent.create_child(scope::Kind::Task, name.into());

    parent.set_child(id, EXECUTOR.spawn(parent.run_child(id, future)));
  }
}
