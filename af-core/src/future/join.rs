// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use af_proc_macros::future_join as join;

use super::*;

/// A future that waits for both of two futures to complete.
pub struct Join<A: Future, B: Future>(Buffered<A>, Buffered<B>);

/// Returns a future that waits for both of two futures to complete.
pub fn join<A: Future, B: Future>(a: A, b: B) -> Join<A, B> {
  Join(Buffered::new(a), Buffered::new(b))
}

// Implement Future for Join.

impl<A: Future, B: Future> Future for Join<A, B> {
  type Output = (A::Output, B::Output);

  fn poll(self: Pin<&mut Self>, cx: &mut future::Context) -> Poll<Self::Output> {
    let mut ready = true;

    unsafe {
      let this = self.get_unchecked_mut();

      if let Poll::Pending = Pin::new_unchecked(&mut this.0).poll(cx) {
        ready = false;
      }

      if let Poll::Pending = Pin::new_unchecked(&mut this.1).poll(cx) {
        ready = false;
      }

      if ready {
        return Poll::Ready((this.0.take_output(), this.1.take_output()));
      }
    }

    Poll::Pending
  }
}
