// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Run operations concurrently by starting them on separate dedicated threads.

use super::{channel, scope};
use crate::prelude::*;
use crate::util::SharedStr;

/// An executor which runs async operations on a single thread.
type Executor = Rc<async_executor::LocalExecutor<'static>>;

thread_local! {
  /// A thread-specific executor.
  static EXECUTOR: RefCell<Option<Executor>> = default();
}

/// Returns a reference to the current thread's executor, if one exists.
pub(crate) fn executor() -> Option<Executor> {
  EXECUTOR.with(|ex| ex.borrow().clone())
}

/// Creates an executor for the current thread and returns a reference to it.
pub(crate) fn init_executor() -> Executor {
  let executor = Executor::default();
  EXECUTOR.with(|ex| *ex.borrow_mut() = Some(executor.clone()));
  executor
}

/// Starts a thread which runs an async operation to completion.
#[track_caller]
pub fn start<O>(op: impl Future<Output = O> + Send + 'static)
where
  O: scope::IntoOutput + 'static,
{
  start_as("", op)
}

/// Starts a named thread which runs an async operation to completion.
#[track_caller]
pub fn start_as<O>(name: impl Into<SharedStr>, op: impl Future<Output = O> + Send + 'static)
where
  O: scope::IntoOutput + 'static,
{
  let parent =
    scope::current().expect("the current thread does not support starting child threads");
  let name = name.into();

  std::thread::Builder::new()
    .name(name.to_string())
    .spawn(move || {
      let executor = init_executor();
      let id = parent.register_child("thread", name);
      let future = parent.run_child(id, op);
      let (tx, rx) = channel::<()>().split();

      let child = executor.spawn(async move {
        let _tx = tx;

        future.await
      });

      parent.insert_child(id, child);

      async_io::block_on(executor.run(rx.recv()));
    })
    .expect("failed to spawn thread");
}
