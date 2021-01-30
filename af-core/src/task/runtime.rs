// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! The task runtime.

use crate::lazy::SyncOnceCell;
use crate::prelude::*;
use crate::task;
use crate::thread;
use signal_hook::consts::TERM_SIGNALS;
use signal_hook::iterator::Signals;
use std::process::exit;
use tokio::runtime::{Handle, Runtime};

/// The handle to the global runtime.
pub(crate) static HANDLE: SyncOnceCell<Handle> = SyncOnceCell::new();

/// Returns a handle to the global runtime.
pub(crate) fn handle() -> &'static Handle {
  HANDLE
    .get()
    .expect("The runtime must be started with `af_core::runtime::run` before tasks can be spawned.")
}

/// Starts a task on the runtime, waits for it to exit, then exits the process.
///
/// If the task fails, this function logs the error and exits the process with
/// a non-zero exit code.
pub fn run<T, E>(future: impl Future<Output = Result<T, E>> + Send + 'static) -> !
where
  T: Send + 'static,
  E: Display + Send + 'static,
{
  // Force the detection of the local time zone now because it requires blocking
  // operations to do so.

  time::Zone::local();

  // Start the tokio runtime.

  let runtime = Runtime::new().expect("Failed to start tokio runtime");

  if HANDLE.set(runtime.handle().clone()).is_err() {
    panic!("The runtime is already running.")
  }

  // Start the main task and wait for it to exit.

  let task = task::start(future);

  match runtime.block_on(task.join()) {
    Ok(Err(err)) => {
      error!("The main task failed. {}", err);

      thread::sleep(Duration::hz(60));
      exit(1)
    }

    Err(task::JoinError::Panic(panic)) => {
      if let Some(value) = panic.value_str() {
        error!("The main task panicked with `{}`.", value);
      } else {
        error!("The main task panicked.")
      }

      thread::sleep(Duration::hz(60));
      exit(-1)
    }

    _ => exit(0),
  }
}

/// Starts a task from the given function, waits for it to stop, then exits the
/// process.
///
/// If the task fails, this function logs the error and exits the process with
/// a non-zero exit code.
///
/// The provided function is passed a [`task::CancelSignal`] that is triggered
/// when the process receives a termination signal (SIGINT, SIGTERM, or
/// SIGQUIT).
pub fn run_with<T, E, F>(func: impl FnOnce(task::CancelSignal) -> F + Send + 'static) -> !
where
  T: Send + 'static,
  E: Display + Send + 'static,
  F: Future<Output = Result<T, E>> + Send + 'static,
{
  let canceler = task::Canceler::new();
  let cancel = canceler.signal();

  let mut signals = Signals::new(TERM_SIGNALS).expect("Failed to register signal handler");

  thread::start("af_core::run canceler", move || {
    let mut iter = signals.into_iter();

    iter.next();

    warn!("The process received a termination signal. Canceling the main task…");

    canceler.cancel();
  });

  run(async { func(cancel).await })
}
