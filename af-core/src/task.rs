// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Task-based concurrency.

pub use crate::future::{sleep, yield_now, PanicError};

use crate::prelude::*;
use crate::runtime::backend;

/// A handle that can be used to wait for a task to complete and receive its
/// result.
pub struct Handle<T>(backend::JoinHandle<T>);

/// Starts running an async operation on a new task.
pub fn start<T: Send + 'static>(op: impl Future<Output = T> + Send + 'static) -> Handle<T> {
  Handle(backend::spawn(op))
}

impl<T> Handle<T> {
  /// Waits for the associated task to finish and returns its result.
  ///
  /// If the task panics, this function returns an error containing the panic
  /// value.
  pub async fn join(self) -> Result<T, PanicError> {
    self.0.join().await
  }
}
