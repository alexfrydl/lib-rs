// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Task-based concurrency.

mod batch;
mod cancel;
mod handle;
mod output;
mod parallel;

pub use self::batch::{Batch, BatchError, BatchResult};
pub use self::cancel::{CancelSignal, Canceled, Canceler};
pub use self::handle::Handle;
pub use self::output::{Failure, Output, Panicked};
pub use self::parallel::Parallel;

use crate::prelude::*;

/// A trait marking futures that can be started as tasks.
pub trait Task<T, E>: TryFuture<T, E> + Send + Sized + 'static
where
  T: Send + 'static,
  E: Send + 'static,
{
}

impl<F, T, E> Task<T, E> for F
where
  F: TryFuture<T, E> + Send + 'static,
  T: Send + 'static,
  E: Send + 'static,
{
}

/// Waits for the given duration to elapse.
pub async fn sleep(duration: Duration) {
  if duration.is_infinite() {
    future::forever().await
  } else {
    async_io::Timer::after(duration.into()).await;
  }
}

/// Starts a new task.
pub fn start<T, E>(future: impl Task<T, E>) -> Handle<T, E>
where
  T: Send + 'static,
  E: Send + 'static,
{
  let task = async_global_executor::spawn(output::capture(future));

  task.into()
}

/// Yields once to other running tasks.
pub async fn yield_now() {
  futures_lite::future::yield_now().await;
}
