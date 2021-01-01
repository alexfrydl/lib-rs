// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Asynchronous tasks.

use super::executor;
use crate::prelude::*;

/// A handle to a task running a future on the af-core runtime.
///
/// If this handle is dropped, the task is canceled. Use [`detach()`] to
/// prevent this.
#[must_use = "Tasks get canceled when dropped. Use `.detach()` to run them in the background."]
pub struct Task<T> {
  inner: async_executor::Task<T>,
}

impl<T: Send + 'static> Task<T> {
  /// Creates a new asynchronous task.
  pub fn new(future: impl Future<Output = T> + Send + 'static) -> Task<T> {
    Task { inner: executor().spawn(future) }
  }
}

// Implement `Future` to poll the inner task.

impl<T> Future for Task<T> {
  type Output = T;

  fn poll(self: Pin<&mut Self>, cx: &mut future::Context) -> future::Poll<Self::Output> {
    let inner = unsafe { Pin::map_unchecked_mut(self, |s| &mut s.inner) };

    inner.poll(cx)
  }
}
