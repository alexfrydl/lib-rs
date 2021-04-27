// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Run operations concurrently on the current thread by starting them on
//! separate fibers.

use super::{runtime, scope};
use crate::prelude::*;
use crate::util::SharedStr;

/// Starts a concurrency scope on a child fiber that runs on the current thread.
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

/// Starts a named concurrency scope on a child fiber that runs on the current
/// thread.
///
/// This function panics if the current thread does not support fibers (for
/// example, if it is a task executor from the global thread pool).
#[track_caller]
pub fn start_as<O>(name: impl Into<SharedStr>, op: impl Future<Output = O> + 'static)
where
  O: scope::IntoOutput + 'static,
{
  assert!(runtime::can_spawn_local(), "the current thread does not support fibers");

  let parent = scope::current().expect("cannot start child fibers from this context");
  let id = parent.register_child("fiber", name.into());
  let child = runtime::spawn_local(parent.run_child(id, op));

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

    assert!(rx.load(Acquire), "did not run");
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
