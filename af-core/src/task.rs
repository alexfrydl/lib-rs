// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Task-based concurrency.

pub mod join;
pub mod try_join;

mod cancel;
mod error;

pub use self::cancel::{CancelSignal, Canceled, Canceler};
pub use self::error::{Error, Panic, Result, ResultResultExt};
pub use self::join::Join;
pub use self::try_join::TryJoin;

use crate::prelude::*;

/// A future that can be used as a task.
pub trait Future<T>: future::Future<Output = T> + Send + Sized + 'static {}

impl<T, F> Future<T> for F where F: future::Future<Output = T> + Send + Sized + 'static {}

/// A [`Future`] that returns a result.
pub trait TryFuture<T, E>: Future<Result<T, E>> {}

impl<T, E, F> TryFuture<T, E> for F where F: Future<Result<T, E>> {}

/// Waits for the given duration to elapse.
pub async fn sleep(duration: Duration) {
  if duration.is_infinite() {
    future::forever().await
  } else {
    async_io::Timer::after(duration.into()).await;
  }
}

/// Starts a new task.
pub fn start<T: Send + 'static>(future: impl Future<T>) -> Task<T> {
  let task = async_global_executor::spawn(async move {
    future::catch_unwind(panic::AssertUnwindSafe(future)).await.map_err(|value| Panic { value })
  });

  Task { task }
}

/// Yields once to other running tasks.
pub async fn yield_now() {
  futures_lite::future::yield_now().await;
}

/// An asynchronous task.
#[must_use = "Tasks are killed when dropped."]
pub struct Task<T> {
  task: async_executor::Task<Result<T>>,
}

impl<T> Task<T> {
  /// Kills the task and waits for its future to be dropped.
  pub async fn kill(self) {
    self.task.cancel().await;
  }

  /// Waits for the task to stop and returns its result.
  pub async fn join(self) -> Result<T> {
    self.task.await
  }
}

impl<T, E> Task<Result<T, E>>
where
  E: From<Panic>,
{
  /// Waits for the fallible task to stop and returns its result.
  pub async fn try_join(self) -> Result<T, E> {
    self.task.await?
  }
}
