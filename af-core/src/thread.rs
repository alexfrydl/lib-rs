// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Thread utilities.

use crate::prelude::*;
use std::thread::*;

/// A handle that can be used to wait for a thread to complete and receive its
/// result.
pub struct Handle<T> {
  inner: JoinHandle<T>,
}

/// Blocks the current thread until the given future completes.
pub fn block_on<T>(future: impl Future<Output = T>) -> T {
  futures_lite::future::block_on(future)
}

/// Blocks the current thread for a given duration.
pub fn sleep(dur: Duration) {
  std::thread::sleep(dur.into());
}

/// Spawns a new thread.
pub fn spawn<T: Send + 'static>(
  name: impl Into<String>,
  func: impl FnOnce() -> T + Send + 'static,
) -> Handle<T> {
  let name = name.into();

  Handle { inner: Builder::new().name(name).spawn(func).expect("Failed to start thread").into() }
}

impl<T> Handle<T> {
  /// Blocks the current thread until this thread completes and returns its
  /// output.
  pub fn join(self) -> T {
    self.inner.join().unwrap()
  }
}
