// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use af_proc_macros::future_race as race;

use super::*;

/// A future that waits for one of two futures to complete.
pub struct Race<A, B>(pub A, pub B);

/// Returns a future that waits for one of two futures to complete.
pub fn race<A: Future, B: Future>(a: A, b: B) -> Race<A, B> {
  Race(a, b)
}

// Implement Future for Race.

impl<O, A, B> Future for Race<A, B>
where
  A: Future<Output = O>,
  B: Future<Output = O>,
{
  type Output = O;

  fn poll(self: Pin<&mut Self>, cx: &mut future::Context) -> Poll<Self::Output> {
    unsafe {
      let this = self.get_unchecked_mut();

      if let Poll::Ready(output) = Pin::new_unchecked(&mut this.0).poll(cx) {
        return Poll::Ready(output);
      }

      if let Poll::Ready(output) = Pin::new_unchecked(&mut this.1).poll(cx) {
        return Poll::Ready(output);
      }
    }

    Poll::Pending
  }
}
