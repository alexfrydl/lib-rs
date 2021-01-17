// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::channel;
use crate::prelude::*;
use crate::string::SharedString;
use crate::task::{self, Task};
use fnv::FnvHashMap;

/// Runs multiple tasks in parallel and streams their results.
pub struct Parallel<T, E> {
  children: FnvHashMap<usize, Child>,
  next_index: usize,
  rx: channel::Receiver<TaskExit<T, E>>,
  tx: channel::Sender<TaskExit<T, E>>,
}

/// A completed [`Parallel`] task.
pub struct CompletedTask<T, E> {
  /// The index of the task.
  pub index: usize,
  /// The name of the task, if any.
  pub name: SharedString,
  /// The output of the task.
  pub output: task::Output<T, E>,
}

/// A child task.
struct Child {
  name: SharedString,
  _task: task::Handle<(), Infallible>,
}

/// An exit message sent from a task monitor.
struct TaskExit<T, E> {
  index: usize,
  output: task::Output<T, E>,
}

impl<T, E> Parallel<T, E>
where
  T: Send + 'static,
  E: Debug + Display + Send + 'static,
{
  /// Creates a new task batch.
  pub fn new() -> Self {
    let (tx, rx) = channel::unbounded();

    Self { children: default(), next_index: 0, rx, tx }
  }

  /// Adds a task to run in parallel.
  pub fn add(&mut self, task: impl Task<T, E>) {
    self.add_as("", task)
  }

  /// Adds a named task to run in parallel.
  pub fn add_as(&mut self, name: impl Into<SharedString>, task: impl Task<T, E>) {
    let index = self.next_index;
    let name = name.into();
    let tx = self.tx.clone();

    let _task = task::start(async move {
      let output = task::output::capture(task).await;
      let _ = tx.send(TaskExit { index, output }).await;

      Ok(())
    });

    self.next_index += 1;
    self.children.insert(index, Child { name: name.into(), _task });
  }

  /// Waits for the result of the next completed task.
  ///
  /// If all tasks have been completed, this function returns `None`. Otherwise,
  /// it returns a tuple containing the index of the task and its result.
  pub async fn next(&mut self) -> Option<CompletedTask<T, E>> {
    if self.children.is_empty() {
      return None;
    }

    let TaskExit { index, output } = self.rx.recv().await.ok()?;
    let child = self.children.remove(&index).expect("Received result from unknown child.");

    Some(CompletedTask { index, name: child.name, output })
  }
}
