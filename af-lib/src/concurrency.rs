// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Concurrency primitives and utilities.

pub mod channel;
pub mod fiber;
pub mod future;
pub(crate) mod runtime;
pub(crate) mod scope;
pub mod task;
pub mod thread;

pub use self::channel::channel;
pub use self::future::Future;
pub use once_cell::sync::{Lazy, OnceCell};

/// Yields once to pending concurrent operations.
pub async fn cooperative_yield() {
  futures_lite::future::yield_now().await
}

/// Waits for all children of the current concurrency scope to exit.
pub async fn join() {
  scope::current().expect("join() cannot be called from this context").join_children().await
}
