use core::ptr::null;
use once_cell::sync::Lazy;
use std::task::{Context, RawWaker, RawWakerVTable, Waker};

/// A shared instance of the no-op waker.
static INSTANCE: Lazy<Waker> = Lazy::new(|| unsafe { Waker::from_raw(create_raw()) });

/// Returns a [`Context`] that uses a no-op waker.
pub fn context() -> Context<'static> {
  Context::from_waker(instance())
}

/// Returns a [`Waker`] singleton that does nothing.
pub fn instance() -> &'static Waker {
  &*INSTANCE
}

/// Creates the raw no-op waker.
fn create_raw() -> RawWaker {
  unsafe fn clone(_data: *const ()) -> RawWaker {
    create_raw()
  }

  unsafe fn noop(_data: *const ()) {}

  RawWaker::new(null(), &RawWakerVTable::new(clone, noop, noop, noop))
}
