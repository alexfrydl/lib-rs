// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Run operations concurrently by starting them on separate dedicated threads.

use super::{channel, runtime, scope};
use crate::prelude::*;
use crate::util::SharedStr;

/// Starts a concurrency scope on a child thread.
///
/// The child thread can start [fibers][super::fiber] for thread-local
/// concurrency.
#[track_caller]
pub fn start<O>(op: impl Future<Output = O> + Send + 'static)
where
  O: scope::IntoOutput + 'static,
{
  start_as("", op)
}

/// Starts a named concurrency scope on a child thread.
///
/// The child thread can start [fibers][super::fiber] for thread-local
/// concurrency.
#[track_caller]
pub fn start_as<O>(name: impl Into<SharedStr>, op: impl Future<Output = O> + Send + 'static)
where
  O: scope::IntoOutput + 'static,
{
  let parent = scope::current().expect("cannot start child threads from this context");
  let name = name.into();
  let id = parent.register_child("thread", name.clone());

  std::thread::Builder::new()
    .name(name.to_string())
    .spawn(move || {
      runtime::block_on(async move {
        // We need to spawn an actual AsyncOp, so this main future will instead
        // wait to receive a message when the operation exits.

        let future = parent.run_child(id, op);
        let (tx, rx) = channel().split();

        let child = runtime::spawn_local(async move {
          defer! {
            tx.send(());
          }

          future.await
        });

        parent.insert_child(id, child);

        rx.recv().await;
      });
    })
    .expect("failed to spawn thread");
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
