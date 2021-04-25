// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Miscellaneous utilities.

pub mod defer;
pub mod derive;
pub mod failure;
pub mod fmt;
pub mod future;
pub mod iter;
pub mod lazy;
pub mod panic;
pub mod random;
mod shared_str;
mod uuid;

pub use futures_lite::pin;
pub use pin_project::pin_project;

#[doc(inline)]
pub use self::defer::defer;
#[doc(inline)]
pub use self::failure::{failure, Failure};
pub use self::shared_str::SharedStr;
pub use self::uuid::Uuid;

/// Returns the “default value” for a type.
pub fn default<T: Default>() -> T {
  T::default()
}
