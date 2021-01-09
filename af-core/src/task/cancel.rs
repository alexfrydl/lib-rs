use crate::prelude::*;
use event_listener::{Event, EventListener};
use std::sync::atomic::{self, AtomicBool};

/// The notifier for a [`CancelSignal`].
pub struct Canceler(Arc<Inner>);

/// An awaitable cancellation signal.
pub struct CancelSignal {
  inner: Arc<Inner>,
  listener: Option<EventListener>,
}

#[derive(Default)]
struct Inner {
  event: Event,
  flag: AtomicBool,
}

impl Canceler {
  /// Sets the cancel signal.
  pub fn cancel(&self) {
    if !self.0.flag.swap(true, atomic::Ordering::AcqRel) {
      self.0.event.notify_relaxed(usize::MAX);
    }
  }

  /// Returns `true` if the cancel signal is set.
  pub fn is_canceled(&self) -> bool {
    self.0.flag.load(atomic::Ordering::Relaxed)
  }
}

impl CancelSignal {
  /// Creates a new cancellation signal and a [`Canceler`] to trigger it.
  pub fn new() -> (Canceler, Self) {
    let inner = Arc::new(Inner::default());

    (Canceler(inner.clone()), Self { inner, listener: None })
  }

  /// Returns `true` if the cancel signal is set.
  pub fn is_set(&self) -> bool {
    self.inner.flag.load(atomic::Ordering::Acquire)
  }
}

// Implement Clone for CancelSignal to clone without the listener.

impl Clone for CancelSignal {
  fn clone(&self) -> Self {
    Self { inner: self.inner.clone(), listener: None }
  }
}

// Implement Future for CancelSignal to return when the signal is set.

impl Future for CancelSignal {
  type Output = ();

  fn poll(self: Pin<&mut Self>, cx: &mut future::Context) -> future::Poll<Self::Output> {
    let _self = unsafe { self.get_unchecked_mut() };

    if _self.listener.is_none() {
      if _self.is_set() {
        return future::Poll::Ready(());
      }

      _self.listener = Some(_self.inner.event.listen());
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
