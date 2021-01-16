use crate::channel;
use crate::prelude::*;
use crate::string::SharedString;
use crate::task::{self, Task};

/// A batch of concurrent tasks that must all complete successfully.
///
/// If any task fails, the rest of the tasks are canceled.
pub struct Batch<E> {
  canceler: Option<task::Canceler>,
  children: Vec<Child>,
  rx: channel::Receiver<TaskExit<E>>,
  tx: channel::Sender<TaskExit<E>>,
}

/// A child task.
struct Child {
  monitor: task::Handle<(), Infallible>,
  name: SharedString,
}

/// An exit message sent from a task monitor.
struct TaskExit<E> {
  index: usize,
  output: task::Output<(), E>,
}

impl<E> Batch<E>
where
  E: Debug + Display + Send + 'static,
{
  /// Creates a new task batch.
  pub fn new() -> Self {
    let (tx, rx) = channel::unbounded();

    Self { canceler: None, children: Vec::with_capacity(16), rx, tx }
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
    let name = name.into();
    let index = self.children.len();
    let tx = self.tx.clone();

    let monitor = task::start(async move {
      let output = task::output::capture(task).await.map(|_| ());
      let _ = tx.send(TaskExit { index, output }).await;

      Ok(())
    });

    self.children.push(Child { monitor, name: name.into() });
  }

  /// Runs the batch until all tasks exit successfully or a task fails.
  pub async fn run(mut self) -> BatchResult<E> {
    drop(self.tx);

    let mut err = None;

    while let Ok(TaskExit { index, output }) = self.rx.recv().await {
      let task = &mut self.children[index];

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

    for child in self.children {
      let _ = child.monitor.await;
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
