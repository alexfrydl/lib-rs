// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Wait for multiple fallible tasks concurrently.

use crate::prelude::*;
use crate::string::SharedString;
use crate::task;

/// The index of a [`Join`] task.
pub type Index = usize;

/// Concurrently waits for the results of multiple tasks that may return an
/// error.
#[derive(Deref, DerefMut)]
pub struct TryJoin<T, E> {
  #[deref]
  #[deref_mut]
  join: task::Join<Result<T, E>>,
}

impl<T, E> TryJoin<T, E>
where
  T: Send + 'static,
  E: From<task::Panic> + Send + 'static,
{
  /// Creates an empty join.
  pub fn new() -> Self {
    Self { join: task::Join::new() }
  }

  /// Adds a task to the join, returning its index.
  pub fn add(&mut self, task: impl task::Start<Result<T, E>>) -> Index {
    self.join.add(task)
  }

  /// Adds a named task to the join, returning its index.
  pub fn add_as(
    &mut self,
    name: impl Into<SharedString>,
    task: impl task::Start<Result<T, E>>,
  ) -> Index {
    self.join.add_as(name, task)
  }

  /// Waits for the next stopped task.
  ///
  /// If all tasks have stopped, this function returns `None`.
  pub async fn next(&mut self) -> Option<StoppedTask<T, E>> {
    let task = self.join.next().await?;

    Some(StoppedTask {
      index: task.index,
      name: task.name,
      result: task.result.map_err(E::from).and_then(|res| res),
    })
  }

  /// Waits for the next stopped task and returns its information as a
  /// [`Result`].
  ///
  /// If all tasks have stopped, this function returns `None`.
  pub async fn try_next(&mut self) -> Option<Result<FinishedTask<T>, FailedTask<E>>> {
    let task = self.next().await?;

    Some(match task.result {
      Ok(output) => Ok(FinishedTask { index: task.index, name: task.name, output }),
      Err(error) => Err(FailedTask { index: task.index, name: task.name, error }),
    })
  }

  /// Waits for all tasks to stop, dropping their results.
  pub async fn drain(&mut self) {
    self.join.drain().await
  }

  /// Waits for all tasks to stop, dropping their results, until a task fails.
  pub async fn try_drain(&mut self) -> Result<(), FailedTask<E>> {
    while self.try_next().await.transpose()?.is_some() {}

    Ok(())
  }
}

impl<T, E> Default for TryJoin<T, E>
where
  T: Send + 'static,
  E: From<Panic> + Send + 'static,
{
  fn default() -> Self {
    Self::new()
  }
}

/// Information about a stopped task.
#[derive(Debug)]
pub struct StoppedTask<T, E> {
  /// The index of the task.
  pub index: Index,
  /// The name of the task, if any.
  pub name: SharedString,
  /// The result of the task.
  pub result: Result<T, E>,
}

/// Information about a finished task.
#[derive(Debug)]
pub struct FinishedTask<T> {
  /// The index of the task.
  pub index: Index,
  /// The name of the task, if any.
  pub name: SharedString,
  /// The output of the task.
  pub output: T,
}

/// Information about a failed task.
#[derive(Debug)]
pub struct FailedTask<E> {
  /// The index of the task.
  pub index: Index,
  /// The name of the task, if any.
  pub name: SharedString,
  /// The error of the task.
  pub error: E,
}

impl<E> Display for FailedTask<E>
where
  E: Display,
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.name.as_str() {
      "" => write!(f, "Task #{} ", self.index)?,
      name => write!(f, "Task `{}`", name)?,
    }

    write!(f, "failed. {}", self.error)
  }
}

impl<E> Error for FailedTask<E> where E: Debug + Display {}
