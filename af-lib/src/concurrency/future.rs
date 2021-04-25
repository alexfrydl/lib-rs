// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Basic async operations represented by the [`Future`] trait.

pub use std::future::Future;
pub use std::task::{Context, Poll};

pub use futures_lite::ready;

use crate::prelude::*;
use crate::util::{panic, Panic};

/// Waits for an async operation to complete, capturing panic information if one
/// occurs.
pub async fn capture_panic<F>(op: F) -> Result<F::Output, Panic>
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

  CapturePanic(op).await
}

/// Waits forever.
pub async fn never() -> ! {
  futures_lite::future::pending().await
}

/// Waits for one of two async operations to complete and returns its output.
///
/// The remaining operation is canceled. If both operations complete at the same
/// time, the output of the first is returned.
pub async fn race<T>(a: impl Future<Output = T>, b: impl Future<Output = T>) -> T {
  use futures_lite::FutureExt;

  a.or(b).await
}

/// Executes an async operation, setting a thread local value whenever it is
/// polled.
///
/// This function can be used to implement “future local” values using a thread
/// local storage cell.
pub async fn with_tls_value<V, F>(
  key: &'static std::thread::LocalKey<RefCell<V>>,
  value: V,
  op: F,
) -> F::Output
where
  V: 'static,
  F: Future,
{
  #[pin_project]
  struct WithTls<V: 'static, F> {
    key: &'static std::thread::LocalKey<RefCell<V>>,
    value: V,
    #[pin]
    op: F,
  }

  impl<V, F> Future for WithTls<V, F>
  where
    F: Future,
  {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
      let key = self.key;
      let mut this = self.project();
      let local = &mut this.value;

      key.with(|cell| mem::swap(&mut *cell.borrow_mut(), local));

      defer! {
        key.with(|cell| mem::swap(&mut *cell.borrow_mut(), local))
      };

      let output = ready!(this.op.poll(cx));

      Poll::Ready(output)
    }
  }

  WithTls { key, value, op }.await
}
