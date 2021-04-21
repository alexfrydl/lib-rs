// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::prelude::*;

/// An extension trait for `Result<Result<T, E>, F>`.
pub trait ResultResultExt<T, E, F> {
  /// Flattens this nested result by converting the errors to the same type.
  fn flatten_err<G>(self) -> Result<T, G>
  where
    G: From<E>,
    G: From<F>;
}

impl<T, E, F> ResultResultExt<T, E, F> for Result<Result<T, E>, F> {
  fn flatten_err<G>(self) -> Result<T, G>
  where
    G: From<E>,
    G: From<F>,
  {
    match self {
      Ok(Ok(value)) => Ok(value),
      Ok(Err(err)) => Err(err.into()),
      Err(err) => Err(err.into()),
    }
  }
}
