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

use crate::prelude::*;
use num_traits::{NumAssignOps, NumOps};
use rand::distributions::uniform::SampleUniform;

/// A trait for types that implement all the basic operations of a number.
///
/// This trait is implemented for all primitive integer and floating-point
/// types.
pub trait Number:
  PartialOrd + PartialEq + Zero + One + NumOps + NumAssignOps + SampleUniform
{
}

// Implement Number for all types that implement the required traits.

impl<T> Number for T where
  T: PartialOrd + PartialEq + Zero + One + NumOps + NumAssignOps + SampleUniform
{
}
