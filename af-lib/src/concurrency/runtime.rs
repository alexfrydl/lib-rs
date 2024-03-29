// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Concurrency runtime plumbing not intended for end users.

use tokio::runtime::{Handle, Runtime};
use tokio::task::LocalSet;
pub use tokio::time::Sleep;

use super::{future, scope, OnceCell};
use crate::prelude::*;
use crate::time::Duration;
use crate::util::log;

/// A handle to the shared global runtime.
static HANDLE: OnceCell<Handle> = OnceCell::new();

thread_local! {
  /// A thread-specific flag indicating whether a [`LocalSet`] is available.
  static HAS_LOCAL_SET: Cell<bool> = Cell::new(false);
}

/// Blocks the current thread to run an async operation.
pub fn block_on<O>(op: impl Future<Output = O>) -> O {
  with_local_set(|local| handle().block_on(local.run_until(op)))
}

/// Returns `true` if [`spawn_local()`] can be used in the current context.
pub fn can_spawn_local() -> bool {
  HAS_LOCAL_SET.with(|cell| cell.get())
}

/// Runs a closure in the context of a local set.
fn with_local_set<O>(closure: impl FnOnce(LocalSet) -> O) -> O {
  let local = LocalSet::new();
  let original_hls = HAS_LOCAL_SET.with(|cell| cell.replace(true));

  defer! {
    HAS_LOCAL_SET.with(|cell| cell.set(original_hls));
  }

  closure(local)
}

/// Returns a handle to the global runtime.
///
/// If no runtime is started, this will start one. This is needed to make tests
/// work.
fn handle() -> &'static Handle {
  HANDLE.get_or_init(|| {
    let runtime = Runtime::new().expect("failed to start tokio runtime");
    let handle = runtime.handle().clone();

    std::thread::spawn(move || runtime.block_on(future::never()));

    handle
  })
}

/// Runs an async operation as the main scope and then exits the process.
pub fn run<O, F>(module_path: &'static str, op: F) -> !
where
  O: scope::IntoOutput + 'static,
  F: Future<Output = O> + 'static,
{
  // Start a tokio runtime.

  let runtime = Runtime::new().expect("failed to start tokio runtime");

  if HANDLE.set(runtime.handle().clone()).is_err() {
    panic!("a runtime was already started");
  }

  // Run the main function in a concurrency scope with a local set for fibers.

  let result = with_local_set(|local| local.block_on(&runtime, scope::run(op)));

  if let Err(err) = &result {
    error!(target: module_path, "Main thread {}", err);
  }

  // Ensure all log messages have been written.

  runtime.block_on(log::flush());

  // Exit with the appropriate code.

  let code = process::get_exit_code();

  if code == 0 && result.is_err() {
    process::exit(i8::MIN);
  }

  process::exit(code);
}

/// Waits for a duration of time to elapse.
pub fn sleep(duration: Duration) -> Sleep {
  let duration = duration.to_std();

  std::panic::catch_unwind(|| tokio::time::sleep(duration)).unwrap_or_else(|_| {
    let _guard = handle().enter();

    tokio::time::sleep(duration)
  })
}

/// Spawns an async operation onto the shared global thread pool.
pub fn spawn(op: impl Future<Output = ()> + Send + 'static) -> AsyncOp {
  AsyncOp(handle().spawn(op))
}

/// Spawns an async operation onto the current thread.
pub fn spawn_local(op: impl Future<Output = ()> + 'static) -> AsyncOp {
  assert!(can_spawn_local(), "spawn_local() is not allowed in this context");
  AsyncOp(tokio::task::spawn_local(op))
}

/// A spawned async operation.
///
/// The operation is canceled when this structure is dropped.
pub struct AsyncOp(tokio::task::JoinHandle<()>);

impl Drop for AsyncOp {
  fn drop(&mut self) {
    self.0.abort();
  }
}
