// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use af_macros::{attempt, attempt_async};
pub use cfg_if::cfg_if;
pub use futures_lite::pin;
pub use once_cell::sync::Lazy;

/// Returns the “default value” for a type.
pub fn default<T: Default>() -> T {
  T::default()
}
