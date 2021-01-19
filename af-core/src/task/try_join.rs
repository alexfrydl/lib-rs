// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

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
  E: Send + 'static,
{
  /// Creates an empty join.
  pub fn new() -> Self {
    Self { join: task::Join::new() }
  }

  /// Waits for the result of the next completed task.
  ///
  /// If all tasks have been completed, this function returns `None`.
  pub async fn next(&mut self) -> Option<TaskResult<T, E>> {
    let result = self.join.next().await?;

    Some(result.into())
  }
}

/// The result of a [`TryJoin`] task.
#[derive(Debug)]
pub struct TaskResult<T, E> {
  /// The index of the task.
  pub index: Index,
  /// The name of the task, if any.
  pub name: SharedString,
  /// The result of the task.
  pub result: Result<T, task::Error<E>>,
}

impl<T, E> From<task::join::TaskResult<Result<T, E>>> for TaskResult<T, E> {
  fn from(result: task::join::TaskResult<Result<T, E>>) -> Self {
    Self { index: result.index, name: result.name, result: result.result.flatten_err() }
  }
}

impl<T, E> Display for TaskResult<T, E>
where
  E: Display,
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.name.as_str() {
      "" => write!(f, "Task #{} ", self.index)?,
      name => write!(f, "Task `{}`", name)?,
    }

    match &self.result {
      Ok(_) => write!(f, "succeeded."),
      Err(task::Error::Err(err)) => write!(f, "failed. {}", err),
      Err(task::Error::Panic(panic)) => match panic.value_str() {
        Some(value) => write!(f, "panicked with `{}`.", value),
        None => write!(f, "panicked."),
      },
    }
  }
}
