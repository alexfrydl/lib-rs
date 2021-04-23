// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Miscellaneous utilities.

mod defer;
mod uuid;

pub use cfg_if::cfg_if;
pub use futures_lite::pin;
pub use pin_project::pin_project;

pub use self::defer::defer;
pub use self::uuid::Uuid;

/// Returns the “default value” for a type.
pub fn default<T: Default>() -> T {
  T::default()
}
