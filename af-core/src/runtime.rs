// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Contains the main entry point code for running af-core applications.

pub mod logger;
pub mod task;

pub use self::task::Task;
pub use af_proc_macros::runtime_main as main;

use crate::prelude::*;
use crate::sync::AtomicBool;
use crate::thread;
use async_executor::Executor;
use easy_parallel::Parallel;
use event_listener::Event;
use std::process::exit;

/// Runs the af runtime until the given future completes, then exits the
/// process.
pub fn run(future: impl Future<Output = Result> + Send + 'static) -> ! {
  // Ensure that only one runtime is running per process.

  static IS_RUNNING: Lazy<AtomicBool> = Lazy::new(default);

  if IS_RUNNING.swap(true) {
    panic!("The af-core runtime is already running.");
  }

  // Run a thread pool executor and a local executor that handles the main
  // thread.

  let ex = executor();
  let shutdown = Event::new();
  let threads = num_cpus::get();

  #[cfg(not(feature = "tokio-compat"))]
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

  #[cfg(feature = "tokio-compat")]
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
    let _ = writeln!(console::Term::stderr(), "{:#}", err);

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
