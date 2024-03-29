// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::prelude::*;
use core::ptr::null;
use std::task::{Context, RawWaker, RawWakerVTable, Waker};

/// A shared instance of the no-op waker.
static INSTANCE: Lazy<Waker> = Lazy::new(|| unsafe { Waker::from_raw(create_raw()) });

/// Returns a [`Context`] that uses a no-op waker.
pub fn context() -> Context<'static> {
  Context::from_waker(instance())
}

/// Returns a [`Waker`] singleton that does nothing.
pub fn instance() -> &'static Waker {
  &*INSTANCE
}

/// Creates the raw no-op waker.
fn create_raw() -> RawWaker {
  unsafe fn clone(_data: *const ()) -> RawWaker {
    create_raw()
  }

  unsafe fn noop(_data: *const ()) {}

  RawWaker::new(null(), &RawWakerVTable::new(clone, noop, noop, noop))
}
