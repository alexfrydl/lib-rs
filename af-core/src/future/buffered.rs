// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// A buffered future that separates polling from consuming the output value.
pub enum Buffered<F: Future> {
  Pending(F),
  Ready(F::Output),
  Taken,
}

impl<F: Future> Buffered<F> {
  /// Returns a new buffered future.
  pub fn new(future: F) -> Buffered<F> {
    Buffered::Pending(future)
  }

  /// Returns the output value of the future.
  ///
  /// If the output value is not available, this function will panic.
  pub fn into_output(self) -> F::Output {
    if let Self::Ready(output) = self {
      return output;
    }

    panic!("The output is not available.");
  }

  /// Removes and returns the output value of the future.
  ///
  /// If the output value is not available, this function will panic.
  pub fn take_output(&mut self) -> F::Output {
    let mut taken = Self::Taken;

    mem::swap(self, &mut taken);

    taken.into_output()
  }
}

// Implement polling.

impl<F: Future> Future for Buffered<F> {
  type Output = ();

  fn poll(mut self: Pin<&mut Self>, cx: &mut future::Context) -> Poll<()> {
    unsafe {
      match Pin::as_mut(&mut self).get_unchecked_mut() {
        Self::Pending(f) => match Pin::new_unchecked(f).poll(cx) {
          Poll::Pending => Poll::Pending,

          Poll::Ready(value) => {
            self.set(Self::Ready(value));

            Poll::Ready(())
          }
        },

        _ => Poll::Ready(()),
      }
    }
  }
}
