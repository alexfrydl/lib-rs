// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Run operations concurrently on a shared, global thread pool by starting
//! them on separate tasks.

use super::{runtime, scope};
use crate::prelude::*;
use crate::util::SharedStr;

/// Starts a task which runs an async operation to completion on a global,
/// shared thread pool.
#[track_caller]
pub fn start<O>(op: impl Future<Output = O> + Send + 'static)
where
  O: scope::IntoOutput + 'static,
{
  start_as("", op)
}

/// Starts a named task which runs an async operation to completion on a global,
/// shared thread pool.
#[track_caller]
pub fn start_as<O>(name: impl Into<SharedStr>, op: impl Future<Output = O> + Send + 'static)
where
  O: scope::IntoOutput + 'static,
{
  let parent = scope::current().expect("cannot start child tasks from this context");
  let id = parent.register_child("task", name.into());

  parent.insert_child(id, runtime::spawn(parent.run_child(id, op)));
}
