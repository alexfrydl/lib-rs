use super::*;
use crate::sync::{channel, AtomicCell, AtomicCellListener};
use crate::task;

type TaskHandle<T = (), E = fail::Error> = task::Handle<TaskOutput<T, E>>;
type TaskOutput<T = (), E = fail::Error> = Result<T, E>;
type TaskResult<T = (), E = fail::Error> = Result<TaskOutput<T, E>, future::PanicError>;

pub struct Batch<T, E> {
  canceled: Arc<AtomicCell<State>>,
  tasks: Vec<TaskHandle<T, E>>,
  output_rx: channel::Receiver<TaskResult<T, E>>,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum State {
  Running,
  Canceled,
  Succeeded,
}

pub struct CancelSignal(Option<AtomicCellListener<State>>);

impl<T, E> Batch<T, E> {
  pub fn add(&mut self, task: TaskHandle<T, E>) {
    self.tasks.push(task);
  }

  pub async fn cancel(&self) {
    self.canceled.store(true);

    for task in tasks {}
  }

  pub fn cancel_signal(&self) -> CancelSignal {
    CancelSignal(Some(self.canceled.listen()))
  }
}

impl Future for CancelSignal {
  type Output = ();

  fn poll(self: Pin<&mut Self>, cx: &mut future::Context) -> future::Poll<()> {
    let this = unsafe { self.get_unchecked_mut() };

    match &mut this.0 {
      None => future::Poll::Ready(()),

      Some(listener) => match unsafe { Pin::new_unchecked(listener) }.poll(cx) {
        future::Poll::Ready(value) if value == State::Canceled => {
          this.0 = None;

          future::Poll::Ready(())
        }

        _ => future::Poll::Pending,
      },
    }
  }
}
