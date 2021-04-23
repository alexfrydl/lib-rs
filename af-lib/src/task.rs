// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Task-based concurrency.

use async_task::Runnable;

use crate::future::Poll;
use crate::prelude::*;
use crate::{channel, env, thread};

fn install_panic_hook() {
  use std::sync::Once;

  static STATE: Once = Once::new();

  STATE.call_once(|| {
    let hook = panic::take_hook();

    panic::set_hook(Box::new(move |info| {
      if !IS_EXECUTOR.with(|c| c.get()) {
        hook(info);
      }
    }))
  });
}

impl Executor {
  pub fn new() -> Self {
    install_panic_hook();

    let (queue_tx, queue_rx) = channel();
    let executor = Self { queue_rx, queue_tx };

    for i in 0..env::num_cpus() {
      let runnables = executor.queue_rx.clone();

      thread::spawn(format!("task executor #{}", i), async move {
        IS_EXECUTOR.with(|e| e.set(true));

        while let Some(runnable) = runnables.recv().await {
          runnable.run();
          warn!("Continuing {}…", i);
        }
      });
    }

    executor
  }

  pub fn handle(&self) -> Handle {
    Handle { queue_tx: self.queue_tx.clone() }
  }
}

impl Handle {
  pub fn spawn(&self, future: impl Future + Send + 'static) -> async_task::Task<Result<(), Panic>> {
    let tx = self.queue_tx.clone();

    let future = async move {
      future::catch_unwind(panic::AssertUnwindSafe(future)).await?;
      Ok(())
    };

    let schedule = move |runnable| {
      tx.send(runnable);
    };

    let (runnable, task) = async_task::spawn(future, schedule);

    self.queue_tx.send(runnable);

    task
  }
}

struct Tree<F> {
  main: F,
}
