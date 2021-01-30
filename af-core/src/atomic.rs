// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Thread-safe value types using atomic operations.

use crate::prelude::*;
use ::atomic::{Atomic as Inner, Ordering::*};

/// A generic wrapper type that allows a value to be shared between tasks.
///
/// This type can only be used with types that are no larger in memory size than
/// a [`usize`].
pub struct Atomic<T>(Inner<T>);

impl<T> Atomic<T>
where
  T: Copy,
{
  /// Creates a new, thread-safe value.
  pub fn new(value: T) -> Self {
    Self(Inner::new(value))
  }

  /// Atomically loads the current value.
  pub fn load(&self) -> T {
    self.0.load(Acquire)
  }

  /// Atomically loads the current value with relaxed memory ordering.
  pub fn load_relaxed(&self) -> T {
    self.0.load(Relaxed)
  }

  /// Atomically stores a new value.
  pub fn store(&self, value: T) {
    self.0.store(value, Release)
  }

  /// Atomically stores a new value and returns the previous value.
  pub fn swap(&self, value: T) -> T {
    self.0.swap(value, AcqRel)
  }
}

/// Implements integer specfic operations.
macro_rules! impl_int {
  ($ty:ty, $($rest:tt)*) => {
    impl_int!($ty);
    impl_int!($($rest)*);
  };

  ($ty:ty) => {
    impl Atomic<$ty> {
      /// Atomically adds a given value to the current value then returns the
      /// previous value.
      pub fn fetch_add(&self, value: $ty) -> $ty {
        self.0.fetch_add(value, AcqRel)
      }

      /// Atomically subtracts a given value from the current value then returns
      /// the previous value.
      pub fn fetch_sub(&self, value: $ty) -> $ty {
        self.0.fetch_sub(value, AcqRel)
      }
    }
  };
}

impl_int!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
