use crate::channel;
use crate::prelude::*;
use crate::string::SharedString;
use crate::task;

/// A batch of concurrent tasks that must all complete successfully.
///
/// If any task fails, the rest of the tasks are canceled.
pub struct Batch<E> {
  canceler: Option<task::Canceler>,
  rx: channel::Receiver<TaskExit<E>>,
  tx: channel::Sender<TaskExit<E>>,
  tasks: Vec<Task>,
}

/// A task in a batch.
struct Task {
  _monitor: task::Handle<(), Infallible>,
  name: SharedString,
}

/// An exit message sent from a task monitor.
struct TaskExit<E> {
  index: usize,
  output: task::Output<(), E>,
}

impl<E> Batch<E>
where
  E: Debug + Display,
{
  /// Creates a new task batch.
  pub fn new() -> Self {
    let (tx, rx) = channel::unbounded();

    Self { canceler: None, rx, tx, tasks: Vec::with_capacity(16) }
  }

  /// Runs the batch until all tasks exit successfully or a task fails.
  pub async fn run(mut self) -> BatchResult<E> {
    drop(self.tx);

    let mut err = None;

    while let Ok(TaskExit { index, output }) = self.rx.recv().await {
      let task = &mut self.tasks[index];

      if let Err(failure) = output {
        match &err {
          None => {
            err = Some(BatchError { failure, task_index: index, task_name: task.name.clone() });
          }

          Some(_) if task.name.is_empty() => {
            warn!("Task #{} failed. {}", index, failure);
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

// Implement the Batch::add method with a DSL for adding extra info.

impl<E> Batch<E>
where
  E: Send + 'static,
{
  /// Adds a task to the batch.
  pub fn add<T>(&mut self, task: task::Handle<T, E>) -> AddTask<E>
  where
    T: Send + 'static,
  {
    let index = self.tasks.len();
    let tx = self.tx.clone();

    let _monitor = task::start(async move {
      let output = task.await.map(|_| ());
      let _ = tx.send(TaskExit { index, output }).await;

      Ok(())
    });

    AddTask { batch: self, task: ManuallyDrop::new(Task { _monitor, name: default() }) }
  }
}

/// A helper for adding information to a [`Batch`] task.
pub struct AddTask<'a, E> {
  batch: &'a mut Batch<E>,
  task: ManuallyDrop<Task>,
}

impl<'a, E> AddTask<'a, E> {
  /// Sets the name of the task in error messages.
  pub fn set_name(&mut self, name: impl Into<SharedString>) {
    self.task.name = name.into();
  }

  /// Sets the name of the task in error messages.
  pub fn with_name(mut self, name: impl Into<SharedString>) -> Self {
    self.set_name(name);
    self
  }
}

impl<'a, E> Drop for AddTask<'a, E> {
  fn drop(&mut self) {
    self.batch.tasks.push(unsafe { ManuallyDrop::take(&mut self.task) });
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
