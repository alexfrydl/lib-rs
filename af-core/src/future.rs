// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Utilities for working with futures and async logic.

pub use std::future::Future;
pub use std::task::{Context, Poll};

use crate::prelude::*;

/// An error returned from [`catch_unwind()`] when the future panics.
#[derive(Debug, Display, Error)]
#[display(fmt = "Panicked.")]
pub struct PanicError {
  /// The value the future panicked with.
  pub value: Box<dyn Any + Send>,
}

/// Waits for a future to complete, returning an `Err` if the future panics.
pub async fn catch_unwind<F>(future: F) -> Result<F::Output, PanicError>
where
  F: Future + panic::UnwindSafe,
{
  futures_lite::FutureExt::catch_unwind(future).await.map_err(|value| PanicError { value })
}

/// Yields once to other running futures or tasks.
pub async fn yield_now() {
  futures_lite::future::yield_now().await;
}
