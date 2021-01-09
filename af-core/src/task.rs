// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Task-based concurrency.

mod cancel;

pub use self::cancel::{CancelSignal, Canceler};

use crate::prelude::*;

/// The output of a [`Task`].
type Output<T> = Result<T, PanicError>;

/// An asynchronous task.
#[must_use = "A task is killed when its Handle is dropped."]
pub struct Handle<T> {
  task: async_executor::Task<Output<T>>,
}

/// An error representing a panic from a [`Future`].
#[derive(Error, From)]
pub struct PanicError {
  /// The value the future panicked with.
  pub value: Box<dyn Any + Send>,
}

/// Waits for the given duration to elapse.
pub async fn sleep(duration: Duration) {
  async_io::Timer::after(duration.into()).await;
}

/// Starts a new task.
pub fn start<F>(future: F) -> Handle<F::Output>
where
  F: Future + Send + 'static,
  F::Output: Send + 'static,
{
  let future = async {
    future::catch_unwind(panic::AssertUnwindSafe(future))
      .await
      .map_err(|value| PanicError { value })
  };

  Handle { task: async_global_executor::spawn(future) }
}

/// Yields once to other running tasks.
pub async fn yield_now() {
  futures_lite::future::yield_now().await;
}

impl<T> Handle<T> {
  /// Kills the task by dropping its associated future, then waits for the task
  /// to exit.
  ///
  /// If the task already exited normally, this function returns its output.
  pub async fn kill(self) -> Option<T> {
    self.task.cancel().await?.ok()
  }
}

// Implement Future for Handle to poll the underlying task.

impl<T> Future for Handle<T> {
  type Output = Output<T>;

  fn poll(self: Pin<&mut Self>, cx: &mut future::Context<'_>) -> future::Poll<Self::Output> {
    let task = unsafe { Pin::map_unchecked_mut(self, |s| &mut s.task) };

    task.poll(cx)
  }
}

impl Debug for PanicError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if let Some(string) = self.value.downcast_ref::<String>() {
      write!(f, "PanicError({:?})", string)
    } else if let Some(string) = self.value.downcast_ref::<&'static str>() {
      write!(f, "PanicError({:?})", string)
    } else {
      write!(f, "PanicError")
    }
  }
}

impl Display for PanicError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if let Some(string) = self.value.downcast_ref::<String>() {
      write!(f, "Task panicked with `{}`.", string)
    } else if let Some(string) = self.value.downcast_ref::<&'static str>() {
      write!(f, "Task panicked with `{}`.", string)
    } else {
      write!(f, "Task panicked.")
    }
  }
}
