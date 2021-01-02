// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Asynchronous tasks.

use crate::prelude::*;
use crate::runtime::backend;

/// A handle that can be used to wait for a task to complete and receive its
/// result.
pub struct Handle<T>(backend::JoinHandle<T>);

/// Waits for the given duration to elapse.
pub async fn sleep(duration: Duration) {
  backend::sleep(duration).await;
}

/// Spawns a task onto the thread pool.
pub fn spawn<T: Send + 'static>(future: impl Future<Output = T> + Send + 'static) -> Handle<T> {
  Handle(backend::spawn(future))
}

impl<T> Handle<T> {
  /// Waits for the associated task to finish and returns its result.
  ///
  /// If the task panics, this function returns an error containing the panic
  /// value.
  pub async fn join(self) -> Result<T, future::PanicError> {
    self.0.join().await
  }
}
