// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Utilities for working with futures and async logic.

pub use std::future::Future;
pub use std::task::{Context, Poll};

mod noop_waker;

use crate::prelude::*;
use futures_lite::FutureExt;

/// Waits for a future to be ready or panic.
///
/// If the future panics, this function returns an `Err` with the panic value.
pub async fn catch_unwind<F>(f: F) -> Result<F::Output, Box<dyn Any + Send>>
where
  F: Future + panic::UnwindSafe,
{
  f.catch_unwind().await
}

/// Waits forever.
pub async fn forever<T>() -> T {
  futures_lite::future::pending().await
}

/// Polls a future and returns its result if it is ready.
pub fn poll<F: Future + Unpin>(f: &mut F) -> Option<F::Output> {
  match Pin::new(f).poll(&mut noop_waker::context()) {
    Poll::Ready(value) => Some(value),
    _ => None,
  }
}

/// Polls the future once then drops it, returning the output if the future was
/// ready.
pub fn try_resolve<T>(f: impl Future<Output = T>) -> Option<T> {
  pin!(f);
  poll(&mut f)
}
