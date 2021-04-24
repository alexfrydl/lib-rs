use super::{scope, thread};
use crate::prelude::*;
use crate::util::SharedStr;

/// Starts a new fiber which runs a future to completion on the current thread.
#[track_caller]
pub fn start<O, F>(name: impl Into<SharedStr>, future: F)
where
  O: scope::IntoOutput + 'static,
  F: Future<Output = O> + 'static,
{
  let executor = thread::executor().expect("thread does not support fibers");
  let parent = scope::current().expect("thread does not support fibers");
  let id = parent.register_child("fiber", name.into());

  parent.insert_child(id, executor.spawn(parent.run_child(id, future)));
}
