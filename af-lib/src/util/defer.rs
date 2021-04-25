// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Deferred closures for running code when a block ends or a panic occurs.

/// Defers a closure so that it is run when the returned [`Deferred`] is
/// dropped.
pub fn defer<F>(closure: F) -> Deferred<F>
where
  F: FnOnce(),
{
  Deferred(Some(closure))
}

/// A deferred closure that will be run when dropped.
#[must_use = "A deferred closure is run when dropped. Use a `let` binding to defer it until the end of the block."]
pub struct Deferred<F>(Option<F>)
where
  F: FnOnce();

impl<F> Deferred<F>
where
  F: FnOnce(),
{
  /// Cancels the deferred closure, dropping it instead.
  pub fn cancel(mut self) {
    self.0.take();
  }

  /// Runs the deferred closure now.
  ///
  /// This is equivalent to (but more readable than) calling `drop()`.
  pub fn run_now(self) {}
}

// Implement Drop to run the deferred closure.

impl<F> Drop for Deferred<F>
where
  F: FnOnce(),
{
  fn drop(&mut self) {
    if let Some(closure) = self.0.take() {
      closure();
    }
  }
}
