// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::prelude::*;
use crate::task::{self, Task};
use event_listener::Event;
use std::sync::atomic::{self, AtomicBool};

/// Inner state of an canceler/signal pair.
#[derive(Default)]
struct Inner {
  event: Event,
  flag: AtomicBool,
}

/// A task canceler that triggers a cloneable [`CancelSignal`].
pub struct Canceler {
  inner: Arc<Inner>,
  _inherit: Option<Task<()>>,
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
        cancel.listen().await;
        linked.cancel();
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
    CancelSignal { inner: Some(self.inner.clone()) }
  }
}

impl Default for Canceler {
  fn default() -> Self {
    Self::new()
  }
}

/// An awaitable cancel signal triggered by a [`Canceler`].
#[derive(Default)]
pub struct CancelSignal {
  inner: Option<Arc<Inner>>,
}

impl CancelSignal {
  /// Waits for the given future until the cancel signal is triggered.
  ///
  /// If the signal is triggered, this function drops the future and returns a
  /// [`Canceled`] error.
  pub async fn guard<F>(&self, future: F) -> Result<F::Output, Canceled>
  where
    F: Future,
  {
    let future = async { Ok(future.await) };

    let signal = async {
      self.listen().await;

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

  /// Waits for the cancel signal to be triggered.
  pub async fn listen(&self) {
    let inner = match &self.inner {
      Some(inner) => inner,
      None => return future::forever().await,
    };

    while !inner.flag.load(atomic::Ordering::Relaxed) {
      let listener = inner.event.listen();

      if self.is_triggered() {
        return;
      }

      listener.await;
    }
  }
}

impl Clone for CancelSignal {
  fn clone(&self) -> Self {
    Self { inner: self.inner.clone() }
  }
}

/// An error indicating a task was canceled.
#[derive(Debug, Error)]
#[error("Canceled.")]
pub struct Canceled;
