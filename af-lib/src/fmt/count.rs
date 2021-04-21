// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;
use crate::math::One;
use crate::prelude::*;

/// Displays a correctly pluralized count of something, for example `3 users`.
pub struct Counted<'a, T> {
  pub count: T,
  pub one: &'a str,
  pub many: &'a str,
}

/// Displays a correctly pluralized count of something, for example `3 users`.
pub fn count<'a, T>(count: T, one: &'a str, many: &'a str) -> Counted<'a, T> {
  Counted { count, one, many }
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
