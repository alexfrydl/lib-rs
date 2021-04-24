// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::prelude::*;
use crate::util::SharedString;

/// A qualified test path, including scope names.
#[derive(Clone, Default)]
pub struct Path {
  pub(super) components: im::Vector<SharedString>,
}

/// An iterator over the components of a path.
pub type Components<'a> = im::vector::Iter<'a, SharedString>;

/// An iterator over the component string slices of a path.
pub type ComponentStrs<'a> = iter::Map<Components<'a>, fn(&'a SharedString) -> &'a str>;

impl Path {
  /// Returns a reference to the components in the path.
  pub fn components(&self) -> Components {
    self.components.iter()
  }

  /// Returns an iterator over the component string slices.
  pub fn component_strs(&self) -> ComponentStrs {
    self.components.iter().map(SharedString::as_str)
  }
}

impl Debug for Path {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self.components)
  }
}

impl Display for Path {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if self.components.is_empty() {
      return write!(f, "(root)");
    }

    let mut components = self.components.iter();

    if let Some(first) = components.next() {
      write!(f, "{}", first)?;
    }

    for component in components {
      let first_char = match component.chars().next() {
        Some(c) => c,
        None => continue,
      };

      if first_char.is_whitespace() || [':', '(', '<'].contains(&first_char) {
        write!(f, "{}", component)?;
      } else {
        write!(f, " {}", component)?;
      }
    }

    Ok(())
  }
}
