// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Thread utilities.

use crate::prelude::*;
use crate::sync::channel;
use std::thread::{Builder, JoinHandle};

/// A handle that can be used to wait for a thread to complete and receive its
/// result.
pub struct Handle<T> {
  inner: JoinHandle<bool>,
  rx: channel::Receiver<T>,
}

/// An error returned from [`Handle::join()`] when the thread panics.
#[derive(Debug, Display, Error, From)]
#[display(fmt = "Thread panicked.")]
pub struct PanicError {
  /// The value the thread panicked with.
  pub value: Box<dyn Any + Send>,
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
) -> Handle<T> {
  let (tx, rx) = channel::bounded(1);

  let inner = Builder::new()
    .name(name.into())
    .spawn(move || tx.send(func()))
    .expect("Failed to start thread");

  Handle { inner, rx }
}

impl<T> Handle<T> {
  /// Waits for the thread to exit and returns its output.
  ///
  /// If the thread panicked, this function returns a [`PanicError`] containing
  /// the value it panicked with.
  pub async fn join(self) -> Result<T, PanicError> {
    if let Ok(output) = self.rx.recv().await {
      return Ok(output);
    }

    if let Err(err) = self.inner.join() {
      return Err(err.into());
    }

    unreachable!("Thread exited successfully but did not send output.");
  }
}
