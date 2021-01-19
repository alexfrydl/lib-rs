// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::prelude::*;
use crate::task::Result;

/// An asynchronous task.
#[must_use = "A task is killed when its Handle is dropped."]
pub struct Handle<T> {
  task: async_executor::Task<Result<T>>,
}

impl<T> Handle<T> {
  /// Kills the task and waits for its future to be dropped.
  pub async fn kill(self) {
    self.task.cancel().await;
  }
}

// Implement From for Handle to convert from async_executor tasks.

impl<T> From<async_executor::Task<Result<T>>> for Handle<T> {
  fn from(task: async_executor::Task<Result<T>>) -> Self {
    Self { task }
  }
}

// Implement Future for Handle to poll the underlying task.

impl<T> Future for Handle<T> {
  type Output = Result<T>;

  fn poll(self: Pin<&mut Self>, cx: &mut future::Context<'_>) -> future::Poll<Self::Output> {
    let task = unsafe { Pin::map_unchecked_mut(self, |s| &mut s.task) };

    task.poll(cx)
  }
}
