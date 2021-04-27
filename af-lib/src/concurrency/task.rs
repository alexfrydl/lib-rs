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

/// Starts a concurrency scope on a child task that runs on the global thread
/// pool.
#[track_caller]
pub fn start<O>(op: impl Future<Output = O> + Send + 'static)
where
  O: scope::IntoOutput + 'static,
{
  start_as("", op)
}
/// Starts a named concurrency scope on a child task that runs on the global
/// thread pool.
#[track_caller]
pub fn start_as<O>(name: impl Into<SharedStr>, op: impl Future<Output = O> + Send + 'static)
where
  O: scope::IntoOutput + 'static,
{
  let parent = scope::current().expect("cannot start child tasks from this context");
  let id = parent.register_child("task", name.into());
  let child = runtime::spawn(parent.run_child(id, op));

  parent.insert_child(id, child);
}

// Tests

#[cfg(test)]
mod tests {
  use std::sync::atomic::AtomicBool;
  use std::sync::atomic::Ordering::{Acquire, Release};

  use super::*;
  use crate::concurrency::join;

  #[async_test]
  async fn should_work() {
    let rx = Arc::new(AtomicBool::new(false));
    let tx = rx.clone();

    start(async move {
      tx.store(true, Release);
    });

    join().await;

    assert!(rx.load(Acquire));
  }

  #[async_test]
  #[should_panic]
  async fn should_propagate_errors() {
    start(async {
      fail!("oh no!");
    });

    join().await;
  }

  #[async_test]
  #[should_panic]
  async fn should_propagate_panics() {
    start(async {
      panic!("oh no!");
    });

    join().await;
  }
}
