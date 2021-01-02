// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Formatting utilities.

mod indent;

pub use self::indent::*;

#[doc(no_inline)]
pub use std::fmt::*;

use crate::fs::path;

/// Formats a string as a file sytem path.
#[derive(Debug)]
pub struct AsPath<'a>(pub &'a str);

/// Formats a value surrounded by a prefix and suffix string.
pub struct Surrounded<'a, T>(pub &'a str, pub T, pub &'a str);

impl Display for AsPath<'_> {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", path::as_std(self.0).display())
  }
}

impl<T: Debug> Debug for Surrounded<'_, T> {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", self.0)?;
    self.1.fmt(f)?;
    write!(f, "{}", self.2)
  }
}

impl<T: Display> Display for Surrounded<'_, T> {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", self.0)?;
    self.1.fmt(f)?;
    write!(f, "{}", self.2)
  }
}
