// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A counting semaphore.

use crate::prelude::*;

/// A counting semaphore with a limited number of “permits” for concurrent
/// operations.
#[derive(Clone)]
pub struct Semaphore(Arc<async_lock::Semaphore>);

/// A permit for a [`Semaphore`], released when dropped.
pub struct Permit(async_lock::SemaphoreGuardArc);

impl Semaphore {
  /// Creates a new semaphore with the specified number of permits.
  pub fn new(permits: usize) -> Self {
    Self(Arc::new(async_lock::Semaphore::new(permits)))
  }

  /// Waits for an available permit.
  pub async fn acquire(&self) -> Permit {
    Permit(self.0.acquire_arc().await)
  }
}

impl Permit {
  /// Releases a permit, dropping it immediately.
  pub fn release(self) {}

  /// Executes a future and then releases the permit.
  pub async fn release_after<O>(self, future: impl Future<Output = O>) -> O {
    future.await
  }
}
