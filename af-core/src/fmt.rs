// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Formatting utilities.

mod describe;

pub use self::describe::{AsDescription, Describe};

#[doc(inline)]
pub use console::style;

#[doc(no_inline)]
pub use std::fmt::*;

/// Formats a string as a file sytem path.
#[derive(Debug)]
pub struct AsPath<'a>(pub &'a str);

/// Formats a value surrounded by a prefix and suffix string.
pub struct Surrounded<'a, T>(pub &'a str, pub T, pub &'a str);

// Implement formatting.

impl Describe for AsPath<'_> {
  fn fmt(&self, f: &mut Formatter) -> Result {
    Display::fmt(&Surrounded("`", self, "`"), f)
  }
}

impl Display for AsPath<'_> {
  fn fmt(&self, f: &mut Formatter) -> Result {
    let mut path = style(self.0);

    if f.alternate() {
      path = path.green();
    }

    write!(f, "{}", path)
  }
}

impl<T: Debug> Debug for Surrounded<'_, T> {
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "{}", self.0)?;
    self.1.fmt(f)?;
    write!(f, "{}", self.2)
  }
}

impl<T: Describe> Describe for Surrounded<'_, T> {
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
