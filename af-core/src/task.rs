// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Task-based concurrency.

pub mod join;
pub mod try_join;

mod cancel;

pub use self::cancel::{CancelSignal, Canceled, Canceler};
pub use self::join::Join;
pub use self::try_join::TryJoin;
pub use crate::task2::runtime;

use crate::prelude::*;

/// Waits for the given duration to elapse.
pub async fn sleep(duration: Duration) {
  if duration.is_infinite() {
    future::forever().await
  } else {
    tokio::time::sleep(duration.into()).await
  }
}

/// Starts a new task.
pub fn start<T>(task: impl Start<T>) -> Task<T>
where
  T: Send + 'static,
{
  task.start()
}

/// Starts a new task.
fn start_impl<T>(future: impl Future<Output = T> + Send + 'static) -> Task<T>
where
  T: Send + 'static,
{
  let task = runtime::handle().spawn(future);

  Task { task }
}

/// Yields once to other running tasks.
pub async fn yield_now() {
  futures_lite::future::yield_now().await;
}

/// An asynchronous task.
#[must_use = "Tasks are killed when dropped."]
pub struct Task<T> {
  task: tokio::task::JoinHandle<T>,
}

impl<T> Task<T> {
  /// Waits for the task to exit and returns its result.
  pub async fn join(mut self) -> Result<T, JoinError> {
    let task = &mut self.task;

    pin!(task);

    task.await.map_err(|err| match err.try_into_panic() {
      Ok(panic) => JoinError::Panic(panic.into()),
      Err(_) => JoinError::Killed,
    })
  }

  /// Kills the task.
  pub fn kill(&self) {
    self.task.abort();
  }
}

impl<T, E> Task<Result<T, E>>
where
  E: From<JoinError>,
{
  /// Waits for the fallible task to stop and returns its result.
  pub async fn try_join(self) -> Result<T, E> {
    self.join().await?
  }
}

impl<T> Drop for Task<T> {
  fn drop(&mut self) {
    self.task.abort();
  }
}

/// A trait for types that can start tasks.
pub trait Start<T> {
  /// Starts a task from this value.
  fn start(self) -> Task<T>;
}

impl<T> Start<T> for Task<T> {
  fn start(self) -> Task<T> {
    self
  }
}

impl<T, F> Start<T> for F
where
  T: Send + 'static,
  F: Future<Output = T> + Send + 'static,
{
  fn start(self) -> Task<T> {
    start_impl(self)
  }
}

/// An error returned from [`Task::join()`].
#[derive(Debug, Error)]
pub enum JoinError {
  #[error(transparent)]
  Panic(Panic),
  #[error("Killed.")]
  Killed,
}
