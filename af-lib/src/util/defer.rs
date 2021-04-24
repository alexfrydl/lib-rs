// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::prelude::*;

/// A deferred function that will be called when dropped.
#[must_use = "This deferred function runs immediately. Assign it to a guard to run it at the end of the block: `let _guard = defer(…);`"]
pub struct Deferred<F, O>(ManuallyDrop<F>)
where
  F: FnOnce() -> O;

/// Defers a function so that it is called when dropped.
pub fn defer<F, O>(func: F) -> Deferred<F, O>
where
  F: FnOnce() -> O,
{
  Deferred(ManuallyDrop::new(func))
}

impl<F, O> Deferred<F, O>
where
  F: FnOnce() -> O,
{
  pub fn run(self) {}
}

impl<F, O> Drop for Deferred<F, O>
where
  F: FnOnce() -> O,
{
  fn drop(&mut self) {
    let func = unsafe { ManuallyDrop::take(&mut self.0) };

    func();
  }
}
