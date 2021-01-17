// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use crate::task::parallel::FailedTask;

use crate::prelude::*;
use crate::string::SharedString;
use crate::task::{self, Task};

/// Runs a batch of related tasks that must all complete successfully.
///
/// If any task fails, the rest of the tasks are killed. To instead cancel tasks
/// and wait for them to exit, provide a [`Canceler`] with [`set_canceler()`].
pub struct Batch<E> {
  canceler: Option<task::Canceler>,
  tasks: task::Parallel<(), E>,
}

impl<E> Batch<E>
where
  E: Debug + Display + Send + 'static,
{
  /// Creates a new task batch.
  pub fn new() -> Self {
    Self { canceler: None, tasks: task::Parallel::new() }
  }

  /// Adds a task to the batch.
  pub fn add<T>(&mut self, task: impl Task<T, E>)
  where
    T: Send + 'static,
  {
    self.add_as("", task)
  }

  /// Adds a named task to the batch.
  pub fn add_as<T>(&mut self, name: impl Into<SharedString>, task: impl Task<T, E>)
  where
    T: Send + 'static,
  {
    self.tasks.add_as(name, async { task.await.map(|_| ()) });
  }

  /// Runs the batch until all tasks exit successfully or a task fails.
  pub async fn run(mut self) -> Result<E> {
    match self.tasks.next_failed().await {
      None => Ok(()),

      Some(failed_task) => {
        // If a canceler was provided, cancel the tasks and wait for them to
        // exit while logging errors.

        if let Some(c) = &self.canceler {
          c.cancel();

          while let Some(ft) = self.tasks.next_failed().await {
            match ft.name.as_str() {
              "" => warn!("Task #{} failed. {}", ft.index, ft.failure),
              name => warn!("Task `{}` failed. {}", name, ft.failure),
            }
          }
        }

        Err(failed_task)
      }
    }
  }

  /// Sets a canceler to use instead of killing tasks when the batch fails.
  pub fn set_canceler(&mut self, canceler: task::Canceler) {
    self.canceler = Some(canceler);
  }
}

/// The result of a [`Batch`].
pub type Result<E> = std::result::Result<(), FailedTask<E>>;

/// The error that caused a [`Batch`] to exit.
#[derive(Debug)]
pub struct BatchError<E> {
  pub task_index: usize,
  pub task_name: SharedString,
  pub failure: task::Failure<E>,
}

impl<E> Error for BatchError<E> where E: Debug + Display {}

impl<E> Display for BatchError<E>
where
  E: Display,
{
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if self.task_name.is_empty() {
      write!(f, "Task #{} failed. {}", self.task_index, self.failure)
    } else {
      write!(f, "Task `{}` failed. {}", self.task_name, self.failure)
    }
  }
}
