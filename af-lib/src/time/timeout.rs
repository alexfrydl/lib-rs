// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Contains functionality associated with [`timeout()`].

use super::Duration;
use crate::concurrency::{future, Future};
use crate::prelude::*;

/// Waits for an async operation to complete with a timeout.
///
/// If the timeout duration elapses before the operation completes, this
/// function returns an error and drops the operation.
pub fn timeout<O>(
  duration: Duration,
  op: impl Future<Output = O>,
) -> impl Future<Output = Result<O, Error>> {
  #[pin_project]
  struct Timeout<F> {
    #[pin]
    future: F,
    #[pin]
    timer: async_io::Timer,
  }

  impl<F> Future for Timeout<F>
  where
    F: Future,
  {
    type Output = Result<F::Output, Error>;

    fn poll(self: Pin<&mut Self>, cx: &mut future::Context) -> future::Poll<Self::Output> {
      let this = self.project();

      if let future::Poll::Ready(value) = this.future.poll(cx) {
        return future::Poll::Ready(Ok(value));
      }

      if this.timer.poll(cx).is_ready() {
        return future::Poll::Ready(Err(Error));
      }

      future::Poll::Pending
    }
  }

  Timeout { future: op, timer: async_io::Timer::after(duration.into()) }
}

/// A timeout error.
#[derive(Error)]
#[error("timed out")]
pub struct Error;

impl Debug for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "timeout::Error")
  }
}
