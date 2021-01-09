use crate::prelude::*;
use event_listener::{Event, EventListener};
use futures_lite::Stream;
use std::sync::atomic::{self, AtomicU64};

/// An observable, automatically mutable value.
///
/// The type `T` must implement [`Copy`] and must not be larger than a [`usize`]
/// in memory.
pub struct AtomicCell<T> {
  event: Event,
  value: AtomicU64,
  _data: PhantomData<T>,
}

/// A listener for an [`AtomicCell`] which yields the cell's current value each
/// time it changes.
///
/// If multiple changes occur between calls to [`next()`], only the latest value
/// is yielded.
pub struct Listener<T> {
  state: Option<(EventListener, T)>,
  cell: Arc<AtomicCell<T>>,
}

/// Transmutes a value from a [`u64`].
unsafe fn from_u64<T>(value: u64) -> T {
  mem::transmute_copy(&value)
}

/// Transmutes a value into a [`u64`].
unsafe fn to_u64<T>(value: T) -> u64 {
  let mut out = 0u64;

  *(&mut out as *mut u64 as *mut T) = value;

  out
}

impl<T: Copy> AtomicCell<T> {
  /// Creates a new cell with the given value.
  pub fn new(value: T) -> Self {
    const MAX_SIZE: usize = mem::size_of::<u64>();

    assert!(
      mem::size_of::<T>() <= MAX_SIZE,
      "AtomicCell may only be used on values smaller than {} bytes.",
      MAX_SIZE,
    );

    Self {
      event: Event::new(),
      value: AtomicU64::new(unsafe { to_u64(value) }),
      _data: PhantomData,
    }
  }

  /// Returns a [`Listener`] which yields the current value each time it
  /// changes.
  ///
  /// The first call to [`Listener::next()`] will immediately yield the current
  /// value. Subsequent calls wait for a new value.
  pub fn listen(self: &Arc<Self>) -> Listener<T> {
    Listener { cell: self.clone(), state: None }
  }

  /// Atomically loads the current value.
  pub fn load(&self) -> T {
    let value = self.value.load(atomic::Ordering::Acquire);

    unsafe { from_u64(value) }
  }

  /// Atomically loads the current value with relaxed ordering.
  pub fn load_relaxed(&self) -> T {
    let value = self.value.load(atomic::Ordering::Relaxed);

    unsafe { from_u64(value) }
  }

  /// Atomically stores a new value.
  pub fn store(&self, value: T)
  where
    T: PartialEq,
  {
    // Store the new value by swapping it with the current value.

    let new = unsafe { to_u64(value) };
    let old = self.value.swap(new, atomic::Ordering::AcqRel);

    // If the old value and new value are bitwise identical, don't notify
    // listeners.

    if new == old {
      return;
    }

    // If the old value and new value are equal according to PartialEq, don't
    // notify listeners.
    //
    // For example, an `f64` can be both positive and negative zero. These are
    // not bitwise identical, but are equal according to PartialEq.

    let old_value = unsafe { from_u64(old) };

    if value == old_value {
      return;
    }

    // Otherwise, notify all listeners that the value has updated.

    self.event.notify_relaxed(usize::MAX);
  }
}

impl<T: Copy> Listener<T> {
  /// Waits for and returns the next value of the cell.
  ///
  /// The first time this function is called, it will return immediately with
  /// the current value of the cell.
  pub async fn next(&mut self) -> T
  where
    T: PartialEq,
  {
    struct Next<'a, T>(&'a mut Listener<T>);

    impl<'a, T: Copy + PartialEq> Future for Next<'a, T> {
      type Output = T;

      fn poll(self: Pin<&mut Self>, cx: &mut future::Context) -> future::Poll<T> {
        unsafe { Pin::map_unchecked_mut(self, |s| s.0) }.poll_next(cx)
      }
    }

    Next(self).await
  }

  /// Polls the listener for the next value.
  fn poll_next(self: Pin<&mut Self>, cx: &mut future::Context) -> future::Poll<T>
  where
    T: PartialEq,
  {
    let _self = unsafe { self.get_unchecked_mut() };

    match &mut _self.state {
      None => return future::Poll::Ready(_self.refresh()),

      Some((listener, prev)) => {
        if unsafe { Pin::new_unchecked(listener) }.poll(cx).is_pending() {
          return future::Poll::Pending;
        }

        let prev = *prev;
        let value = _self.refresh();

        if value != prev {
          return future::Poll::Ready(value);
        }
      }
    }

    future::Poll::Pending
  }

  /// Refreshes the state by creating a new listener and then loading the
  /// current value.
  fn refresh(&mut self) -> T {
    let listener = self.cell.event.listen();
    let value = self.cell.load();

    self.state = Some((listener, value));

    value
  }
}

// Implement Future and Stream to both call poll_next() above.

impl<T: Copy + PartialEq> Future for Listener<T> {
  type Output = T;

  fn poll(self: Pin<&mut Self>, cx: &mut future::Context) -> future::Poll<T> {
    self.poll_next(cx)
  }
}

impl<T: Copy + PartialEq> Stream for Listener<T> {
  type Item = T;

  fn poll_next(self: Pin<&mut Self>, cx: &mut future::Context) -> std::task::Poll<Option<T>> {
    self.poll_next(cx).map(Some)
  }
}
