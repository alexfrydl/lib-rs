// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! task::Future-based concurrency.

mod cancel;
mod handle;
mod joiner;
mod output;

pub use self::cancel::{CancelSignal, Canceled, Canceler};
pub use self::handle::Handle;
pub use self::joiner::Joiner;
pub use self::output::{Failure, Panicked, Result};

use crate::prelude::*;

/// A future that can be used as a task.
pub trait Future<T>: future::Future<Output = T> + Send + Sized + 'static
where
  T: Send + 'static,
{
}

impl<T, U> Future<T> for U
where
  T: Send + Sized + 'static,
  U: future::Future<Output = T> + Send + Sized + 'static,
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
pub fn start<T: Send + 'static>(future: impl Future<T>) -> Handle<T> {
  let task = async_global_executor::spawn(output::capture(future));

  task.into()
}

/// Yields once to other running tasks.
pub async fn yield_now() {
  futures_lite::future::yield_now().await;
}
