// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Run operations concurrently on the current thread by starting them on
//! separate fibers.

use super::{scope, thread};
use crate::prelude::*;
use crate::util::SharedStr;

/// Starts a new fiber which runs a future to completion on the current thread.
///
/// This function panics if the current thread does not support fibers (for
/// example, if it is a task executor from the global thread pool).
#[track_caller]
pub fn start<O, F>(name: impl Into<SharedStr>, future: F)
where
  O: scope::IntoOutput + 'static,
  F: Future<Output = O> + 'static,
{
  let executor = thread::executor().expect("thread does not support fibers");
  let parent = scope::current().expect("thread does not support fibers");
  let id = parent.register_child("fiber", name.into());

  parent.insert_child(id, executor.spawn(parent.run_child(id, future)));
}
