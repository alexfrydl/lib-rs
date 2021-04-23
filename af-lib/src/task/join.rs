// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Wait for multiple tasks concurrently.

use crate::channel;
use crate::prelude::*;
use crate::string::SharedStr;
use crate::task::{self, Task};
use fnv::FnvHashMap;

/// The index of a [`Join`] task.
pub type Index = usize;

/// Concurrently waits for the results of multiple tasks.
pub struct Join<T> {
  children: FnvHashMap<Index, Child>,
  next_index: Index,
  rx: channel::Receiver<Stopped<T>>,
  tx: channel::Sender<Stopped<T>>,
}

/// A task in a [`Join`].
struct Child {
  name: SharedStr,
  _monitor: Task<()>,
}

/// A message sent from a task monitor.
struct Stopped<T> {
  index: usize,
  result: Result<T, task::JoinError>,
}

impl<T> Join<T>
where
  T: Send + 'static,
{
  /// Creates an empty join.
  pub fn new() -> Self {
    let (tx, rx) = channel();

    Self { children: default(), next_index: 0, rx, tx }
  }

  /// Adds a task to the join, returning its index.
  pub fn add(&mut self, task: impl task::Start<T>) -> Index {
    self.add_as("", task)
  }

  /// Adds a named task to the join, returning its index.
  pub fn add_as(&mut self, name: impl Into<SharedStr>, task: impl task::Start<T>) -> Index {
    // Get next index.

    let index = self.next_index;

    self.next_index += 1;

    // Start the task.

    let task = task.start();

    // Start a task to monitor when this task stops and send its result on the
    // channel.

    let tx = self.tx.clone();

    let _monitor = task::start(async move {
      let result = task.join().await;

      tx.send(Stopped { index, result });
    });

    self.children.insert(index, Child { name: name.into(), _monitor });

    index
  }

  /// Waits for the next stopped task.
  ///
  /// If all tasks have stopped, this function returns `None`.
  pub async fn next(&mut self) -> Option<StoppedTask<Result<T, task::JoinError>>> {
    if self.children.is_empty() {
      return None;
    }

    let Stopped { index, result } = self.rx.recv().await?;
    let child = self.children.remove(&index).expect("Received result from unknown child.");

    Some(StoppedTask { index, name: child.name, result })
  }

  /// Waits for all tasks to stop, dropping their results.
  pub async fn drain(&mut self) {
    while self.next().await.is_some() {}
  }
}

impl<T> Default for Join<T>
where
  T: Send + 'static,
{
  fn default() -> Self {
    Self::new()
  }
}

/// Information about a stopped task.
#[derive(Debug)]
pub struct StoppedTask<T> {
  /// The index of the task.
  pub index: Index,
  /// The name of the task, if any.
  pub name: SharedStr,
  /// The result of the task.
  pub result: T,
}

/// Information about a stopped task.
#[derive(Debug, Error)]
pub struct PanickedTask {
  /// The index of the task.
  pub index: Index,
  /// The name of the task, if any.
  pub name: SharedStr,
  /// The panic from the task.
  pub panic: Panic,
}

impl Display for PanickedTask {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.name.as_str() {
      "" => write!(f, "Task #{} ", self.index)?,
      name => write!(f, "Task `{}`", name)?,
    }

    write!(f, "panicked")?;

    if let Some(value) = self.panic.value_str() {
      write!(f, " with `{}`", value)?;
    }

    write!(f, ".")
  }
}