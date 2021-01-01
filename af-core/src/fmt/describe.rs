// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Wraps a value to display its description.
pub struct AsDescription<T>(pub T);

/// A trait similar to `Display` that instead writes a description of the value.
///
/// The output is intended for use in human-readable error messages.
pub trait Describe: Sized {
  /// Writes a description of the value to a given formatter.
  fn fmt(&self, f: &mut Formatter) -> Result;

  /// Returns a description of this value.
  ///
  /// The description is intended for use in human-readable error messages.
  fn describe(&self) -> String {
    AsDescription(self).to_string()
  }
}

// Implement `Display` to show the description.

impl<T: Describe> Display for AsDescription<T> {
  fn fmt(&self, f: &mut Formatter<'_>) -> Result {
    Describe::fmt(&self.0, f)
  }
}

// Implement `Describe` for common types.

impl<T: Describe> Describe for &'_ T {
  fn fmt(&self, f: &mut Formatter) -> Result {
    T::fmt(self, f)
  }
}

impl Describe for char {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "`{}`", self.escape_default())
  }
}
