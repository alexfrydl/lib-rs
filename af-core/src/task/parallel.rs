// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::prelude::*;
use crate::string::SharedString;
use crate::{channel, task};
use fnv::FnvHashMap;

/// The index of a [`Parallel`] task.
pub type Index = usize;

/// Runs multiple tasks in parallel and streams their results.
pub struct Parallel<T> {
  children: FnvHashMap<Index, Child>,
  next_index: Index,
  rx: channel::Receiver<Exit<T>>,
  tx: channel::Sender<Exit<T>>,
}

/// A completed [`Parallel`] task.
pub struct CompletedTask<T> {
  /// The index of the task.
  pub index: Index,
  /// The name of the task, if any.
  pub name: SharedString,
  /// The result of the task.
  pub result: task::Result<T>,
}

/// An error representing a failed task.
#[derive(Debug)]
pub struct FailedTask<E> {
  /// The reason the task failed.
  pub failure: task::Failure<E>,
  /// The index of the task.
  pub index: Index,
  /// The name of the task, if any.
  pub name: SharedString,
}

/// A child task.
struct Child {
  name: SharedString,
  _task: task::Handle<()>,
}

/// An exit message sent from a task monitor.
struct Exit<T> {
  index: usize,
  result: task::Result<T>,
}

impl<T> Parallel<T>
where
  T: Send + 'static,
{
  /// Creates a new task batch.
  pub fn new() -> Self {
    let (tx, rx) = channel::unbounded();

    Self { children: default(), next_index: 0, rx, tx }
  }

  /// Adds a task to run in parallel and returns its index.
  pub fn add(&mut self, task: impl task::Future<T>) -> Index {
    self.add_as("", task)
  }

  /// Adds a named task to run in parallel and returns its index.
  pub fn add_as(&mut self, name: impl Into<SharedString>, task: impl task::Future<T>) -> Index {
    let index = self.next_index;
    let name = name.into();
    let tx = self.tx.clone();

    let _task = task::start(async move {
      let output = task::output::capture(task).await;

      tx.send(Exit { index, result: output }).await.ok();
    });

    self.next_index += 1;
    self.children.insert(index, Child { name: name.into(), _task });

    index
  }

  /// Waits for the result of the next completed task.
  ///
  /// If all tasks have been completed, this function returns `None`. Otherwise,
  /// it returns a tuple containing the index of the task and its result.
  pub async fn next(&mut self) -> Option<CompletedTask<T>> {
    if self.children.is_empty() {
      return None;
    }

    let Exit { index, result } = self.rx.recv().await.ok()?;
    let child = self.children.remove(&index).expect("Received result from unknown child.");

    Some(CompletedTask { index, name: child.name, result })
  }
}

impl<E> Error for FailedTask<E> where E: Debug + Display {}

impl<E> Display for FailedTask<E>
where
  E: Display,
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.name.as_str() {
      "" => write!(f, "task::Future #{} failed. {}", self.index, self.failure),
      name => write!(f, "task::Future `{}` failed. {}", name, self.failure),
    }
  }
}
