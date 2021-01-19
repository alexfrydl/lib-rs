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
mod handle;

pub use self::cancel::{CancelSignal, Canceled, Canceler};
pub use self::error::{Error, Panic, Result, ResultResultExt};
pub use self::handle::Handle;
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
pub fn start<T: Send + 'static>(future: impl Future<T>) -> Handle<T> {
  let task = async_global_executor::spawn(async move {
    future::catch_unwind(panic::AssertUnwindSafe(future)).await.map_err(|value| Panic { value })
  });

  task.into()
}

/// Yields once to other running tasks.
pub async fn yield_now() {
  futures_lite::future::yield_now().await;
}
