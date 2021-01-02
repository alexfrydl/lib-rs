pub mod logger;
pub mod task;

pub use af_macros::runtime_main as main;

use af_core::prelude::*;
use af_core::sync::AtomicBool;
use af_core::thread;
use async_executor::Executor;
use easy_parallel::Parallel;
use event_listener::Event;
use once_cell::sync::Lazy;
use std::process::exit;

/// Runs the runtime until the given future completes, then exits the process.
pub fn run(future: impl Future<Output = Result> + Send + 'static) -> ! {
  // Ensure that only one runtime is running per process.

  static IS_RUNNING: Lazy<AtomicBool> = Lazy::new(default);

  if IS_RUNNING.swap(true) {
    panic!("The Indigo runtime is already running.");
  }

  // Run a thread pool executor and a local executor that handles the main
  // thread.

  let ex = executor();
  let shutdown = Event::new();
  let threads = num_cpus::get();

  #[cfg(not(feature = "tokio-0-2"))]
  let (_, result) = {
    Parallel::new()
    // Run an executor thread per logical CPU core.
    .each(0..threads, |_| ex.run(shutdown.listen()))
    // Run the main future on the current thread.
    .finish(|| ex.enter(|| {
      trace!("Started {} executor threads.", threads);

      let result = main(future);
      shutdown.notify(threads);
      result
    }))
  };

  #[cfg(feature = "tokio-0-2")]
  let (_, result) = {
    let mut tokio = tokio::runtime::Builder::new()
      .enable_all()
      .basic_scheduler()
      .build()
      .expect("Failed to start the tokio runtime");

    let tokio_handle = tokio.handle().clone();

    Parallel::new()
    // Add a thread for tokio.
    .add(|| ex.enter(|| tokio.block_on(shutdown.listen())))
    // Run an executor thread per logical CPU core.
    .each(0..threads, |_| tokio_handle.enter(|| ex.run(shutdown.listen())))
    // Run the main future on the current thread.
    .finish(|| tokio_handle.enter(|| ex.enter(|| {
      trace!("Started {} executor threads and 1 tokio-compat thread.", threads);

      let result = main(future);
      shutdown.notify(threads + 1);
      result
    })))
  };

  if let Err(err) = result {
    eprintln!("{}", err);
    exit(1);
  }

  exit(0)
}

/// Returns a reference to the async executor.
pub(crate) fn executor() -> &'static Executor {
  static EXECUTOR: Lazy<Executor> = Lazy::new(default);

  &EXECUTOR
}

/// Runs the main thread.
fn main(future: impl Future<Output = Result> + Send + 'static) -> Result {
  thread::block_on(future)
}
