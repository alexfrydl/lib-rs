// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Thread utilities.

pub use std::thread::yield_now;

use crate::channel;
use crate::prelude::*;
use std::thread::{Builder, JoinHandle};

/// An operating system thread.
pub struct Thread<T> {
  inner: JoinHandle<()>,
  rx: channel::Receiver<T>,
}

/// An error returned from [`Thread::join()`] when the thread panics.
#[derive(Debug, Error)]
#[error("Thread panicked.")]
pub struct PanicError {
  /// The value the thread panicked with.
  pub value: Box<dyn Any + Send>,
}

/// Blocks the current thread until a given future is ready.
pub fn block_on<T>(future: impl Future<Output = T>) -> T {
  async_io::block_on(future)
}

/// Blocks the current thread for a given duration.
pub fn sleep(dur: Duration) {
  if dur.is_infinite() {
    std::thread::sleep(std::time::Duration::new(u64::MAX, 0));
  } else {
    std::thread::sleep(dur.into());
  }
}

/// Starts running an operation on a new thread.
pub fn start<T: Send + 'static>(
  name: impl Into<String>,
  func: impl FnOnce() -> T + Send + 'static,
) -> Thread<T> {
  let (tx, rx) = channel::with_capacity(1);

  let func = move || {
    let output = func();
    let _ = tx.try_send(output);
  };

  let inner = Builder::new().name(name.into()).spawn(func).expect("Failed to start thread");

  Thread { inner, rx }
}

impl<T> Thread<T> {
  /// Waits for the thread to stop and returns its result.
  pub async fn join(self) -> Result<T, PanicError> {
    if let Ok(output) = self.rx.recv().await {
      return Ok(output);
    }

    if let Err(value) = self.inner.join() {
      return Err(PanicError { value });
    }

    unreachable!("Thread finished but did not send output.");
  }
}
