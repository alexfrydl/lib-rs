// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! String formatting utilities.

#[doc(no_inline)]
pub use std::fmt::*;

mod count;
mod indent;
mod surround;

pub use self::count::{count, Counted};
pub use self::indent::{indent, Indented, IndentedFormatter};
pub use self::surround::{surround, Surrounded};

/// A wrapper struct which displays a value in color.
pub struct InColor<T>(pub T);

/// An extension trait adding the `in_color()` display function.
pub trait InColorExt {
  /// Returns a wrapper struct which displays a value in color.
  fn in_color(&self) -> InColor<&Self>;
}

impl<T> InColorExt for T
where
  for<'a> InColor<&'a T>: Display,
{
  fn in_color(&self) -> InColor<&Self> {
    InColor(self)
  }
}
