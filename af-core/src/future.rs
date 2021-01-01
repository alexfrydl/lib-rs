// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Utilities for working with futures and async logic.

mod buffered;
mod join;
mod race;

pub use self::join::{join, Join};
pub use self::race::{race, Race};

pub use blocking::unblock;
pub use futures_lite::future::FutureExt;
pub use futures_lite::future::{pending, Pending};
pub use futures_lite::future::{poll_fn, PollFn};
pub use std::future::Future;
pub use std::task::{Context, Poll};

use self::buffered::Buffered;
use crate::prelude::*;

/// An error returned from [`catch_unwind()`] when the future panics.
#[derive(Debug, Display, Error)]
#[display(fmt = "Panicked.")]
pub struct PanicError {
  /// The value the future panicked with.
  pub value: Box<dyn Any + Send>,
}

/// Waits for a future to complete, returning an `Err` if the future panics.
pub async fn catch_unwind<F>(future: F) -> Result<F::Output, PanicError>
where
  F: Future + panic::UnwindSafe,
{
  struct CatchUnwind<F>(F);

  impl<F> Future for CatchUnwind<F>
  where
    F: Future + panic::UnwindSafe,
  {
    type Output = Result<F::Output, PanicError>;

    fn poll(self: Pin<&mut Self>, cx: &mut future::Context) -> Poll<Self::Output> {
      let inner = unsafe { Pin::map_unchecked_mut(self, |s| &mut s.0) };
      let result = panic::catch_unwind(panic::AssertUnwindSafe(|| inner.poll(cx)));

      match result {
        Ok(Poll::Pending) => Poll::Pending,
        Ok(Poll::Ready(value)) => Poll::Ready(Ok(value)),
        Err(value) => Poll::Ready(Err(PanicError { value })),
      }
    }
  }

  CatchUnwind(future).await
}

/// Waits for a given duration of time to elapse.
#[cfg(feature = "runtime")]
pub async fn sleep(duration: Duration) {
  async_io::Timer::new(duration.to_std()).await;
}

/// Yields once to other running futures or tasks.
pub async fn yield_now() {
  futures_lite::future::yield_now().await;
}
