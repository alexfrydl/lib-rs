// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Thread utilities.

use crate::prelude::*;
use std::thread::*;

/// A handle to a spawned thread.
///
/// When this handle is dropped, the thread is joined.  Use [`detach()`] to
/// prevent this.
#[must_use = "Threads get joined when dropped. Use `.detach()` to run them in the background."]
pub struct Thread<T> {
  handle: JoinHandle<T>,
}

/// Blocks the current thread until the given future completes.
pub fn block_on<T>(future: impl Future<Output = T>) -> T {
  futures_lite::future::block_on(future)
}

/// Blocks the current thread for a given duration.
pub fn sleep(dur: Duration) {
  std::thread::sleep(dur.into());
}

/// Starts a new thread.
pub fn start<T: Send + 'static>(
  name: impl Into<String>,
  func: impl FnOnce() -> T + Send + 'static,
) -> Thread<T> {
  let name = name.into();

  Thread { handle: Builder::new().name(name).spawn(func).expect("Failed to start thread").into() }
}

/// Starts a new thread that runs to completion in the background.
///
/// Equivalent to `start(…).detach()`.
pub fn start_detached<T>(name: impl Into<String>, func: impl FnOnce() -> T + Send + 'static) {
  start(name, move || {
    func();
  })
  .detach()
}

impl<T> Thread<T> {
  /// Blocks the current thread until this thread completes and returns its
  /// output.
  pub fn join(self) -> T {
    self.handle.join().unwrap()
  }

  /// Detaches this handle so that the thread runs to completion in the
  /// background.
  pub fn detach(self) {}
}
