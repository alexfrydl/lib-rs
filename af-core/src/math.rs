// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod matrix;
mod vector;

pub use self::matrix::Matrix4;
pub use self::vector::{Vector2, Vector3, Vector4};
pub use num_traits::identities::{one, zero, One, Zero};
pub use num_traits::AsPrimitive;

use crate::prelude::*;

/// A trait for types that implement all the basic operations of a number.
///
/// This trait is implemented for all primitive integer and floating-point
/// types, for example.
pub trait Number:
  PartialOrd + PartialEq + Zero + One + num_traits::NumOps + num_traits::NumAssignOps
{
}

impl<T> Number for T where
  T: PartialOrd + PartialEq + Zero + One + num_traits::NumOps + num_traits::NumAssignOps
{
}

/// Clamps a number so that it is between a minimum and maximum bound.
pub fn clamp<T: Number>(mut value: T, min: T, max: T) -> T {
  clamp_mut(&mut value, min, max);

  value
}

/// Clamps a number in-place so that it is between a minimum and maximum bound.
pub fn clamp_mut<T: Number>(value: &mut T, min: T, max: T) {
  if &*value < &min {
    *value = min;
  } else if &*value > &max {
    *value = max;
  }
}

// Unit tests.

#[cfg(test)]
mod tests {
  use super::*;

  /// Tests that the clamp function works.
  #[test]
  fn test_clamp() {
    assert_eq!(clamp(5, 0, 20), 5);
    assert_eq!(clamp(-5, 0, 20), 0);
    assert_eq!(clamp(25, 0, 20), 20);
  }
}
