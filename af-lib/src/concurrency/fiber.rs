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

/// Starts a fiber which runs an async operation to completion on the current
/// thread.
///
/// This function panics if the current thread does not support fibers (for
/// example, if it is a task executor from the global thread pool).
#[track_caller]
pub fn start<O>(op: impl Future<Output = O> + 'static)
where
  O: scope::IntoOutput + 'static,
{
  start_as("", op)
}

/// Starts a named fiber which runs an async operation to completion on the
/// current thread.
///
/// This function panics if the current thread does not support fibers (for
/// example, if it is a task executor from the global thread pool).
#[track_caller]
pub fn start_as<O>(name: impl Into<SharedStr>, op: impl Future<Output = O> + 'static)
where
  O: scope::IntoOutput + 'static,
{
  let executor = thread::executor().expect("the current thread does not support fibers");
  let parent = scope::current().expect("the current thread does not support fibers");
  let id = parent.register_child("fiber", name.into());

  parent.insert_child(id, executor.spawn(parent.run_child(id, op)));
}
