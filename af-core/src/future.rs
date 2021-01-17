// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Utilities for working with futures and async logic.

pub use af_macros::{future_boxed as boxed, future_boxed_local as boxed_local};
pub use std::future::Future;
pub use std::task::{Context, Poll};

mod noop_waker;
mod try_future;

pub use self::try_future::*;

use crate::prelude::*;

/// Waits for a future to be ready or panic.
///
/// If the future panics, this function returns an `Err` with the panic value.
pub async fn catch_unwind<F>(f: F) -> Result<F::Output, Box<dyn Any + Send>>
where
  F: Future + panic::UnwindSafe,
{
  use futures_lite::FutureExt;

  f.catch_unwind().await
}

/// Waits forever.
pub async fn forever<T>() -> T {
  futures_lite::future::pending().await
}

/// Polls a future and returns its result if it is ready.
pub fn poll<F: Future + Unpin>(f: &mut F) -> Option<F::Output> {
  match Pin::new(f).poll(&mut noop_waker::context()) {
    Poll::Ready(value) => Some(value),
    _ => None,
  }
}

/// Waits for one of two futures to complete and returns its result.
///
/// The remaining future is dropped. If both futures are ready at the same time,
/// the first future has priority.
pub async fn race<T>(a: impl Future<Output = T>, b: impl Future<Output = T>) -> T {
  use futures_lite::FutureExt;

  a.or(b).await
}

/// Polls the future once then drops it, returning the output if the future was
/// ready.
pub fn try_resolve<T>(f: impl Future<Output = T>) -> Option<T> {
  pin!(f);
  poll(&mut f)
}

#[pin_project]
pub struct OkWrap<F> {
  #[pin]
  inner: F,
}

impl<F: Future> Future for OkWrap<F> {
  type Output = Result<F::Output, Infallible>;

  fn poll(self: Pin<&mut Self>, cx: &mut future::Context) -> Poll<Self::Output> {
    let this = self.project();

    this.inner.poll(cx).map(Ok)
  }
}

pub trait FutureExt: Future + Sized {
  fn ok(self) -> OkWrap<Self> {
    OkWrap { inner: self }
  }
}

impl<T: Future + Sized> FutureExt for T {}
