// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Asynchronous tasks.

use crate::executor;
use af_core::prelude::*;
use async_executor::Task;

/// A handle that can be used to wait for a task to complete and receive its
/// result.
pub struct Handle<T> {
  task: Option<Task<T>>,
}

/// Spawns a task onto the thread pool.
pub fn spawn<T: Send + 'static>(future: impl Future<Output = T> + Send + 'static) -> Handle<T> {
  let task = executor().spawn(future);

  Handle { task: Some(task) }
}

impl<T> Handle<T> {
  /// Waits for the associated task to finish and returns its result.
  ///
  /// If the task panics, this function returns an error containing the panic
  /// value.
  pub async fn join(mut self) -> Result<T, future::PanicError> {
    let task = self.task.take().unwrap();

    future::catch_unwind(panic::AssertUnwindSafe(task)).await
  }

  /// Cancels the associated task by dropping its future.
  pub fn cancel(mut self) {
    self.task.take();
  }
}

impl<T> Drop for Handle<T> {
  fn drop(&mut self) {
    if let Some(task) = self.task.take() {
      task.detach();
    }
  }
}
