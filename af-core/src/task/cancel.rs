use crate::prelude::*;
use crate::task;
use event_listener::{Event, EventListener};
use std::sync::atomic::{self, AtomicBool};

/// A task canceler that triggers a cloneable [`CancelSignal`].
pub struct Canceler {
  inner: Arc<Inner>,
  _inherit: Option<task::Handle<(), ()>>,
}

/// An awaitable cancel signal triggered by a [`Canceler`].
#[derive(Default)]
pub struct CancelSignal {
  inner: Option<Arc<Inner>>,
  listener: Option<EventListener>,
}

/// An error indicating a task was canceled.
#[derive(Debug, Display, Error)]
#[display(fmt = "Canceled.")]
pub struct Canceled;

/// Inner state of an canceler/signal pair.
#[derive(Default)]
struct Inner {
  event: Event,
  flag: AtomicBool,
}

impl Canceler {
  /// Creates a new task canceler.
  pub fn new() -> Self {
    Self { inner: default(), _inherit: None }
  }

  /// Creates a new task canceler that inherits the state of an existing cancel
  /// signal.
  ///
  /// When the given cancel signal is triggered, the canceler will trigger its
  /// own cancel signal.
  pub fn inherit(cancel: CancelSignal) -> Self {
    let linked = Self::new();

    Self {
      inner: linked.inner.clone(),

      _inherit: Some(task::start(async move {
        cancel.await;
        linked.cancel();

        Ok(())
      })),
    }
  }

  /// Triggers all cancel signals.
  pub fn cancel(&self) {
    if !self.inner.flag.swap(true, atomic::Ordering::AcqRel) {
      self.inner.event.notify_relaxed(usize::MAX);
    }
  }

  /// Returns `true` if the cancel signals have been triggered.
  pub fn is_triggered(&self) -> bool {
    self.inner.flag.load(atomic::Ordering::Relaxed)
  }

  /// Returns a [`CancelSignal`] that is triggered by this canceler.
  pub fn signal(&self) -> CancelSignal {
    CancelSignal { inner: Some(self.inner.clone()), listener: None }
  }
}

impl CancelSignal {
  /// Waits for the given future until the cancel signal is triggered.
  ///
  /// If the signal is triggered, this function drops the future and returns a
  /// [`Canceled`] error.
  pub async fn guard<F>(self, future: F) -> Result<F::Output, Canceled>
  where
    F: Future,
  {
    let future = async { Ok(future.await) };

    let signal = async {
      self.await;

      Err(Canceled)
    };

    futures_lite::future::or(future, signal).await
  }

  /// Returns `true` if the cancel signal has been triggered.
  pub fn is_triggered(&self) -> bool {
    match &self.inner {
      Some(inner) => inner.flag.load(atomic::Ordering::Acquire),
      None => false,
    }
  }
}

// Implement Clone for CancelSignal to clone without the listener.

impl Clone for CancelSignal {
  fn clone(&self) -> Self {
    Self { inner: self.inner.clone(), listener: None }
  }
}

// Implement Future for CancelSignal to wait for the cancel signal to trigger.

impl Future for CancelSignal {
  type Output = ();

  fn poll(self: Pin<&mut Self>, cx: &mut future::Context) -> future::Poll<Self::Output> {
    if self.inner.is_none() {
      return future::Poll::Pending;
    }

    let _self = unsafe { self.get_unchecked_mut() };

    if _self.listener.is_none() {
      if _self.is_triggered() {
        return future::Poll::Ready(());
      }

      _self.listener = Some(_self.inner.as_ref().unwrap().event.listen());
    }

    match unsafe { Pin::new_unchecked(_self.listener.as_mut().unwrap()).poll(cx) } {
      future::Poll::Ready(()) => {
        _self.listener = None;

        future::Poll::Ready(())
      }

      _ => future::Poll::Pending,
    }
  }
}
