// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Run operations concurrently by starting them on separate dedicated threads.

use super::{channel, scope};
use crate::prelude::*;
use crate::util::SharedStr;

/// An executor which runs futures on a single thread.
type Executor = Rc<async_executor::LocalExecutor<'static>>;

thread_local! {
  /// A thread-specific executor.
  static EXECUTOR: RefCell<Option<Executor>> = default();
}

/// Returns a reference to the current thread's executor, if one exists.
pub(super) fn executor() -> Option<Executor> {
  EXECUTOR.with(|ex| ex.borrow().clone())
}

/// Creates an executor for the current thread and returns a reference to it.
fn init_executor() -> Executor {
  let executor = Executor::default();
  EXECUTOR.with(|ex| *ex.borrow_mut() = Some(executor.clone()));
  executor
}

/// Starts a named thread which runs a future to completion.
#[track_caller]
pub fn start<O>(future: impl Future<Output = O> + Send + 'static)
where
  O: scope::IntoOutput + 'static,
{
  start_as("", future)
}

/// Starts a named thread which runs a future to completion.
#[track_caller]
pub fn start_as<O>(name: impl Into<SharedStr>, future: impl Future<Output = O> + Send + 'static)
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
      let future = parent.run_child(id, future);
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

/// Runs a future to completion on the current thread, as though it were started
/// with [`start()`].
pub(super) fn run<O, F>(future: F) -> Result<(), scope::Error>
where
  O: scope::IntoOutput + 'static,
  F: Future<Output = O> + 'static,
{
  let executor = init_executor();

  async_io::block_on(executor.run(scope::run(future)))
}
