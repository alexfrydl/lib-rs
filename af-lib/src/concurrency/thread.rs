// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Run operations concurrently by starting them on separate dedicated threads.

use super::{channel, runtime, scope};
use crate::prelude::*;
use crate::util::SharedStr;

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
  let parent = scope::current().expect("cannot start child threads from this context");
  let name = name.into();

  std::thread::Builder::new()
    .name(name.to_string())
    .spawn(move || {
      runtime::block_on(async move {
        let id = parent.register_child("thread", name);
        let future = parent.run_child(id, op);
        let (tx, rx) = channel::<()>().split();

        let child = runtime::spawn_local(async move {
          let _tx = tx;

          future.await
        });

        parent.insert_child(id, child);

        rx.recv().await;
      });
    })
    .expect("failed to spawn thread");
}
