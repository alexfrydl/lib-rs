// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Utilities for working with futures and async logic.

pub use std::future::Future;
pub use std::task::{Context, Poll};

mod noop_waker;

use crate::prelude::*;
use crate::runtime;
use futures_lite::FutureExt;

/// An error returned from [`cancel_after()`] when the operation is canceled.
#[derive(Clone, Copy, Debug, Display, Error)]
#[display(fmt = "Canceled.")]
pub struct CancelError;

/// An error representing a panic from a [`Future`].
#[derive(Debug, Display, Error, From)]
#[display(fmt = "Panicked.")]
pub struct PanicError {
  /// The value the future panicked with.
  pub value: Box<dyn Any + Send>,
}

/// An error returned from [`timeout()`] when the timeout duration elapses.
#[derive(Clone, Copy, Debug, Display, Error)]
#[display(fmt = "Timed out.")]
pub struct TimeoutError;

/// Waits for a future to be ready or cancels it when a signal future is
/// ready.
///
/// When either future is ready, the other is dropped. If the signal future is
/// ready first, this function returns a [`CancelError`].
pub async fn cancel_after<O, A>(
  signal: impl Future<Output = A>,
  f: impl Future<Output = O>,
) -> Result<O, CancelError> {
  async move {
    let output = f.await;
    Ok(output)
  }
  .or(async {
    let value = signal.await;
    Err(CancelError)
  })
  .await
}

/// Waits for a future to be ready or returns a [`PanicError`] if it panics.
pub async fn catch_unwind<F>(f: F) -> Result<F::Output, PanicError>
where
  F: Future + panic::UnwindSafe,
{
  f.catch_unwind().await.map_err(|value| PanicError { value })
}

/// Polls a future and returns its result if it is ready.
pub fn poll<F: Future + Unpin>(f: &mut F) -> Option<F::Output> {
  match Pin::new(f).poll(&mut noop_waker::context()) {
    Poll::Ready(value) => Some(value),
    _ => None,
  }
}

/// Waits for the given duration to elapse.
pub async fn sleep(duration: Duration) {
  runtime::backend::sleep(duration).await;
}

/// Waits for a future to be ready or cancels it after a given timeout.
///
/// If the timeout duration elapses before the future is ready, this
/// function drops the future and returns `None`.
pub async fn timeout<T>(duration: Duration, f: impl Future<Output = T>) -> Result<T, TimeoutError> {
  cancel_after(sleep(duration), f).await.map_err(|_| TimeoutError)
}

/// Polls the future once then drops it, returning the output if the future was
/// ready.
pub fn try_resolve<T>(f: impl Future<Output = T>) -> Option<T> {
  pin!(f);
  poll(&mut f)
}

/// Runs a blocking operation on a background thread and waits for its output.
pub async fn unblock<T: Send + 'static>(op: impl FnOnce() -> T + Send + 'static) -> T {
  runtime::backend::unblock(op).await
}

/// Yields once to other running futures or tasks.
pub async fn yield_now() {
  futures_lite::future::yield_now().await;
}
