// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A counting semaphore.

use super::channel;
use crate::prelude::*;

/// A counting semaphore.
///
/// A semaphore manages a fixed number of _permits_. Tasks can acquire a single
/// `Permit` by awaiting the `acquire()` method, then release it by dropping it.
#[derive(Clone)]
pub struct Semaphore {
  acquire: channel::Receiver<()>,
  release: channel::Sender<()>,
}

/// A permit for a `Semaphore`, released when dropped.
pub struct Permit {
  release: Option<channel::Sender<()>>,
}

impl Semaphore {
  /// Creates a new semaphore with a specified number of permits.
  pub fn new(permits: usize) -> Self {
    let (release, acquire) = channel::bounded(permits);

    for _ in 0..permits {
      release.try_send(()).unwrap();
    }

    Self { acquire, release }
  }

  /// Waits for an availableü permit and then acquires it.
  pub async fn acquire(&self) -> Permit {
    self.acquire.recv().await.unwrap();

    Permit { release: Some(self.release.clone()) }
  }
}

impl Permit {
  /// Releases a permit, dropping it immediately.
  pub fn release(self) {}

  /// Wraps a future such that when it completes, the permit is released.
  pub async fn release_after<O>(self, future: impl Future<Output = O>) -> O {
    let output = future.await;
    self.release();
    output
  }
}

// Implement `Drop` to release permits.

impl Drop for Permit {
  fn drop(&mut self) {
    if let Some(release) = self.release.take() {
      let _ = release.try_send(());
    }
  }
}
