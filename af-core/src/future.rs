// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Utilities for working with futures and async logic.

pub use std::future::Future;
pub use std::task::{Context, Poll};

use crate::prelude::*;
use crate::runtime;

/// An error returned from [`cancel_after()`] when the operation is canceled.
#[derive(Clone, Copy, Display, Error)]
#[display(fmt = "Canceled.")]
pub struct CancelError<T> {
  pub value: T,
}

/// An error returned from [`Handle::join()`] when the task panics.
#[derive(Debug, Display, Error, From)]
#[display(fmt = "Panicked.")]
pub struct PanicError {
  /// The value the task panicked with.
  pub value: Box<dyn Any + Send>,
}

/// An error returned from [`timeout()`] when the timeout duration elapses.
#[derive(Clone, Copy, Debug, Display, Error)]
#[display(fmt = "Timed out.")]
pub struct TimeoutError;

/// Runs an async operation until another “signal” operation completes.
///
/// If the signal completes before the operation completes, this function
/// returns a [`CancelError`]. The operation is then dropped.
pub async fn cancel_after<O, A>(
  signal: impl Future<Output = A>,
  op: impl Future<Output = O>,
) -> Result<O, CancelError<A>> {
  futures_lite::future::or(
    async move {
      let output = op.await;
      Ok(output)
    },
    async {
      let value = signal.await;
      Err(CancelError { value })
    },
  )
  .await
}

/// Runs an async operation and returs a [`PanicError`] if it panics.
pub async fn catch_unwind<F>(future: F) -> Result<F::Output, PanicError>
where
  F: Future + panic::UnwindSafe,
{
  futures_lite::FutureExt::catch_unwind(future).await.map_err(|value| PanicError { value })
}

/// Waits for the given duration to elapse.
pub async fn sleep(duration: Duration) {
  runtime::backend::sleep(duration).await;
}

/// Runs an async operation with a given timeout.
///
/// If the timeout duration elapses before the operation completes, this
/// function returns `None`. The operation is then dropped.
pub async fn timeout<T>(
  duration: Duration,
  future: impl Future<Output = T>,
) -> Result<T, TimeoutError> {
  cancel_after(sleep(duration), future).await.map_err(|_| TimeoutError)
}

/// Runs the given function on a background thread and waits for the result.
pub async fn unblock<T: Send + 'static>(func: impl FnOnce() -> T + Send + 'static) -> T {
  runtime::backend::unblock(func).await
}

/// Yields once to other running futures or tasks.
pub async fn yield_now() {
  futures_lite::future::yield_now().await;
}

// Delegate `Debug` to `Display` for errors.

impl<T> Debug for CancelError<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    Display::fmt(self, f)
  }
}
