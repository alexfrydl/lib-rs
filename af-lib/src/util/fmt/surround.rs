// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

/// Wraps a value so that it displays with the given prefix and suffix strings.
pub fn surround<'a, T>(prefix: &'a str, value: T, suffix: &'a str) -> Surrounded<'a, T> {
  Surrounded(prefix, value, suffix)
}

/// A wrapper returned from [`surround()`] that displays a value surrounded by
/// a custom prefix and suffix.
pub struct Surrounded<'a, T>(&'a str, T, &'a str);

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
