// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use crate::math::One;
use crate::prelude::*;

/// Wraps a value so that it displays with a correctly pluralized label; for
/// example, `1 image` vs. `3 images`.
pub fn count<'a, T>(count: T, one: &'a str, many: &'a str) -> Counted<'a, T> {
  Counted { count, one, many }
}

/// A wrapper returned from [`count()`] that displays its value with a correctly
/// pluralized label.
pub struct Counted<'a, T> {
  pub count: T,
  pub one: &'a str,
  pub many: &'a str,
}

impl<'a, T> Display for Counted<'a, T>
where
  T: Display + One + PartialEq,
{
  fn fmt(&self, f: &mut Formatter) -> fmt::Result {
    Display::fmt(&self.count, f)?;

    match self.count.is_one() {
      true => write!(f, " {}", self.one),
      false => write!(f, " {}", self.many),
    }
  }
}
