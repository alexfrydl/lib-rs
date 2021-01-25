// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Utilties for working with numerical values.

mod float;

pub use self::float::FloatExt;
pub use num_traits::identities::{one, zero, One, Zero};
pub use num_traits::AsPrimitive;
use num_traits::{NumAssignOps, NumOps};
use rand::distributions::uniform::SampleUniform;

use crate::prelude::*;

/// A trait for types that implement all the basic operations of a number.
///
/// This trait is implemented for all primitive integer and floating-point
/// types.
pub trait Number:
  PartialOrd + PartialEq + Zero + One + NumOps + NumAssignOps + SampleUniform
{
}

/// Clamps a number so that it is between a minimum and maximum bound.
pub fn clamp<T: Number>(mut value: T, min: T, max: T) -> T {
  clamp_mut(&mut value, min, max);

  value
}

/// Clamps a number in-place so that it is between a minimum and maximum bound.
pub fn clamp_mut<T: Number>(value: &mut T, min: T, max: T) {
  if *value < min {
    *value = min;
  } else if *value > max {
    *value = max;
  }
}

// Implement Number for all types that implement the required traits.

impl<T> Number for T where
  T: PartialOrd + PartialEq + Zero + One + NumOps + NumAssignOps + SampleUniform
{
}
