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
pub fn capture_panic<F>(op: F) -> impl Future<Output = Result<F::Output, Panic>>
where
  F: Future + panic::UnwindSafe,
{
  with_poll_fn(op, |this, cx| {
    let result = panic::capture(panic::AssertUnwindSafe(|| this.poll(cx)));

    match result {
      Ok(poll) => poll.map(Ok),
      Err(panic) => Poll::Ready(Err(panic)),
    }
  })
}

/// Waits forever.
pub async fn never() {
  futures_lite::future::pending().await
}

/// Waits for one of two async operations to complete and returns its output.
///
/// The remaining operation is canceled. If both operations complete at the same
/// time, the output of the first is returned.
pub fn race<T>(a: impl Future<Output = T>, b: impl Future<Output = T>) -> impl Future<Output = T> {
  use futures_lite::FutureExt;

  a.or(b)
}

/// Waits for an async operation to complete by polling it with a custom
/// closure.
pub fn with_poll_fn<O, F>(
  op: F,
  poll: impl FnMut(Pin<&mut F>, &mut Context) -> Poll<O>,
) -> impl Future<Output = O>
where
  F: Future,
{
  #[pin_project]
  struct WithPollFn<O, F, P>
  where
    P: FnMut(Pin<&mut F>, &mut Context) -> Poll<O>,
  {
    #[pin]
    op: F,
    poll: P,
  }

  impl<O, F, P> Future for WithPollFn<O, F, P>
  where
    F: Future,
    P: FnMut(Pin<&mut F>, &mut Context) -> Poll<O>,
  {
    type Output = O;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
      let this = self.project();

      (this.poll)(this.op, cx)
    }
  }

  WithPollFn { op, poll }
}
