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

/// The index of a [`Join`] task.
pub type Index = usize;

/// Concurrently waits for the results of multiple tasks.
pub struct Join<T> {
  children: FnvHashMap<Index, Child>,
  next_index: Index,
  rx: channel::Receiver<Exit<T>>,
  tx: channel::Sender<Exit<T>>,
}

/// A task in a [`Join`].
struct Child {
  name: SharedString,
  _monitor: Task<()>,
}

/// An exit message sent from a task monitor.
struct Exit<T> {
  index: usize,
  result: task::Result<T>,
}

impl<T> Join<T>
where
  T: Send + 'static,
{
  /// Creates an empty join.
  pub fn new() -> Self {
    let (tx, rx) = channel::unbounded();

    Self { children: default(), next_index: 0, rx, tx }
  }

  /// Adds a task to the join, returning its index.
  pub fn add(&mut self, task: Task<T>) -> Index {
    self.add_as("", task)
  }

  /// Adds a named task to the join, returning its index.
  pub fn add_as(&mut self, name: impl Into<SharedString>, task: Task<T>) -> Index {
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

    self.children.insert(index, Child { name: name.into(), _monitor });

    index
  }

  /// Starts a task from a future and adds it to the join, returning its
  /// index.
  pub fn start(&mut self, future: impl task::Future<T>) -> Index {
    self.start_as("", future)
  }

  /// Starts a task from a future and adds it to the join with the given name,
  /// returning its index.
  pub fn start_as(&mut self, name: impl Into<SharedString>, future: impl task::Future<T>) -> Index {
    self.add_as(name, task::start(future))
  }

  /// Waits for the result of the next completed task.
  ///
  /// If all tasks have been completed, this function returns `None`.x.
  pub async fn next(&mut self) -> Option<TaskResult<T>> {
    if self.children.is_empty() {
      return None;
    }

    let Exit { index, result } = self.rx.recv().await.ok()?;
    let child = self.children.remove(&index).expect("Received result from unknown child.");

    Some(TaskResult { index, name: child.name, result })
  }

  /// Waits for the result of the next panicked task, dropping the results of
  /// tasks that complete successfully.
  ///
  /// If all tasks have been completed, this function returns `None`.
  pub async fn next_panic(&mut self) -> Option<TaskPanic> {
    while let Some(task) = self.next().await {
      match task.result {
        Ok(_) => continue,
        Err(panic) => return Some(TaskPanic { index: task.index, name: task.name, panic }),
      }
    }

    None
  }

  /// Waits for and drops the results of all remaining tasks.
  pub async fn drain(&mut self) {
    while let Some(_) = self.next().await {}

    self.next_index = 0;
  }
}

impl<T, E> Join<Result<T, E>>
where
  T: Send + 'static,
  E: Send + 'static,
{
  /// Waits for the result of the next failed task, dropping the results of
  /// tasks that complete successfully.
  ///
  /// If all tasks have been completed, this function returns `None`.
  pub async fn next_err(&mut self) -> Option<TaskError<E>> {
    while let Some(task) = self.next().await {
      match task.result.flatten_err() {
        Ok(_) => continue,
        Err(err) => return Some(TaskError { index: task.index, name: task.name, err }),
      }
    }

    None
  }
}

/// The result of a task.
pub struct TaskResult<T> {
  /// The index of the task.
  pub index: Index,
  /// The name of the task, if any.
  pub name: SharedString,
  /// The result of the task.
  pub result: task::Result<T>,
}

impl<T> Display for TaskResult<T> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.name.as_str() {
      "" => write!(f, "Task #{} ", self.index)?,
      name => write!(f, "Task `{}`", name)?,
    }

    match &self.result {
      Ok(_) => write!(f, "succeeded."),
      Err(panic) => match panic.value_str() {
        Some(value) => write!(f, "panicked with `{}`.", value),
        None => write!(f, "panicked."),
      },
    }
  }
}

/// An error representing a task that panicked.
#[derive(Debug, Error)]
pub struct TaskPanic {
  /// The index of the task.
  pub index: Index,
  /// The name of the task, if any.
  pub name: SharedString,
  /// The panic from the task.
  pub panic: task::Panic,
}

impl Display for TaskPanic {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.name.as_str() {
      "" => write!(f, "Task #{} ", self.index)?,
      name => write!(f, "Task `{}`", name)?,
    }

    match self.panic.value_str() {
      Some(value) => write!(f, "panicked with `{}`.", value),
      None => write!(f, "panicked."),
    }
  }
}

/// An error representing a task that failed.
pub struct TaskError<E> {
  /// The index of the task.
  pub index: Index,
  /// The name of the task, if any.
  pub name: SharedString,
  /// The error of the task.
  pub err: task::Error<E>,
}

impl<E> Display for TaskError<E>
where
  E: Display,
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.name.as_str() {
      "" => write!(f, "Task #{} ", self.index)?,
      name => write!(f, "Task `{}`", name)?,
    }

    write!(f, "failed. {}", self.err)
  }
}
