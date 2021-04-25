// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Run operations concurrently on a shared, global thread pool by starting
//! them on separate tasks.

use super::scope;
use crate::prelude::*;
use crate::util::{future, SharedStr};

/// An executor which can run futures on multiple threads.
type Executor = Arc<async_executor::Executor<'static>>;

/// A global thread pool executor.
static EXECUTOR: Lazy<Executor> = Lazy::new(|| {
  let executor = Executor::default();

  for i in 0..num_cpus::get() + 1 {
    let executor = executor.clone();

    std::thread::Builder::new()
      .name(format!("task executor {}", i + 1))
      .spawn(move || async_io::block_on(executor.run(future::never())))
      .expect("failed to spawn task executor thread");
  }

  executor
});

/// Starts a task which runs a future to completion on a global, shared thread
/// pool.
#[track_caller]
pub fn start<O>(future: impl Future<Output = O> + Send + 'static)
where
  O: scope::IntoOutput + 'static,
{
  start_as("", future)
}

/// Starts a named task which runs a future to completion on a global, shared
/// thread pool.
#[track_caller]
pub fn start_as<O>(name: impl Into<SharedStr>, future: impl Future<Output = O> + Send + 'static)
where
  O: scope::IntoOutput + 'static,
{
  let parent = scope::current().expect("the current thread does not support starting child tasks");
  let id = parent.register_child("task", name.into());

  parent.insert_child(id, EXECUTOR.spawn(parent.run_child(id, future)));
}
