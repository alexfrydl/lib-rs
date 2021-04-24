use super::{scope, thread};
use crate::prelude::*;
use crate::string::SharedStr;

#[track_caller]
pub fn start<F>(name: impl Into<SharedStr>, future: F)
where
  F: Future<Output = Result> + 'static,
{
  let executor = thread::executor().expect("thread does not support fibers");
  let parent = scope::current().expect("thread does not support fibers");
  let id = parent.create_child(scope::Kind::Fiber, name.into());

  parent.set_child(id, executor.spawn(parent.run_child(id, future)));
}
