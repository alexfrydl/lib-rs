// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Common [`Future`] types and utilities.

pub use futures_lite::future::block_on;
pub use std::future::Future;
pub use std::task::{Context, Poll};

mod noop_waker;

use crate::prelude::*;

/// Waits for a future, capturing panic information if one occurs.
pub async fn capture_panic<F>(future: F) -> Result<F::Output, Panic>
where
  F: Future + panic::UnwindSafe,
{
  #[pin_project]
  struct CapturePanic<F>(#[pin] F);

  impl<F: Future> Future for CapturePanic<F> {
    type Output = Result<F::Output, Panic>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
      let this = self.project();
      let result = panic::capture(panic::AssertUnwindSafe(|| this.0.poll(cx)));

      match result {
        Ok(poll) => poll.map(Ok),
        Err(panic) => Poll::Ready(Err(panic)),
      }
    }
  }

  CapturePanic(future).await
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

/// Waits for one of two futures to be ready and returns its result.
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