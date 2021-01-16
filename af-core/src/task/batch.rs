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
  pub async fn run(mut self) -> BatchResult<E> {
    let mut err = None;

    while let Some(task) = self.tasks.next().await {
      if let Err(failure) = task.output {
        match &err {
          None => {
            err = Some(BatchError { failure, task_index: task.index, task_name: task.name });
          }

          Some(_) if task.name.is_empty() => {
            warn!("Task #{} failed. {}", task.index, failure);
          }

          Some(_) => {
            warn!("Task `{}` failed. {}", task.name, failure);
          }
        }

        match &self.canceler {
          Some(e) => e.cancel(),
          None => break,
        }
      }
    }

    match err {
      None => Ok(()),
      Some(err) => Err(err),
    }
  }

  /// Sets a canceler to use instead of killing tasks when the batch fails.
  pub fn set_canceler(&mut self, canceler: task::Canceler) {
    self.canceler = Some(canceler);
  }
}

/// The result of a [`Batch`].
pub type BatchResult<E> = Result<(), BatchError<E>>;

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
