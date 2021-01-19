// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::prelude::*;
use crate::string::SharedString;
use crate::{channel, task};
use fnv::FnvHashMap;

/// The index of a [`Joiner`] task.
pub type Index = usize;

/// Waits for multiple tasks concurrently.
pub struct Joiner<T> {
  children: FnvHashMap<Index, Task>,
  next_index: Index,
  rx: channel::Receiver<Exit<T>>,
  tx: channel::Sender<Exit<T>>,
}

/// The result of a [`Joiner`] task.
pub struct TaskResult<T> {
  /// The index of the task.
  pub index: Index,
  /// The name of the task, if any.
  pub name: SharedString,
  /// The result of the task.
  pub result: task::Result<T>,
}

/// A task in a [`Joiner`].
struct Task {
  name: SharedString,
  _monitor: task::Handle<()>,
}

/// An exit message sent from a task monitor.
struct Exit<T> {
  index: usize,
  result: task::Result<T>,
}

impl<T> Joiner<T>
where
  T: Send + 'static,
{
  /// Creates a new task batch.
  pub fn new() -> Self {
    let (tx, rx) = channel::unbounded();

    Self { children: default(), next_index: 0, rx, tx }
  }

  /// Adds a task to the joiner, returning its index.
  pub fn add(&mut self, task: task::Handle<T>) -> Index {
    self.add_as("", task)
  }

  /// Starts a new task and adds it to the joiner, returning its index.
  pub fn add_new(&mut self, future: impl task::Future<T>) -> Index {
    self.add_as("", task::start(future))
  }

  /// Adds a named task to the joiner, returning its index.
  pub fn add_as(&mut self, name: impl Into<SharedString>, task: task::Handle<T>) -> Index {
    // Get next index.

    let index = self.next_index;

    self.next_index += 1;

    // Start a task to monitor when this task exits and send its result on the
    // channel.

    let tx = self.tx.clone();

    let _monitor = task::start(async move {
      let result = task.await;

      tx.send(Exit { index, result }).await.ok();
    });

    self.children.insert(index, Task { name: name.into(), _monitor });

    index
  }

  /// Waits for the result of the next completed task.
  ///
  /// If all tasks have been completed, this function returns `None`..
  pub async fn next(&mut self) -> Option<TaskResult<T>> {
    if self.children.is_empty() {
      return None;
    }

    let Exit { index, result } = self.rx.recv().await.ok()?;
    let child = self.children.remove(&index).expect("Received result from unknown child.");

    Some(TaskResult { index, name: child.name, result })
  }
}
