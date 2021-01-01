// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::prelude::*;
use crate::sync::Event;
use std::sync::atomic as base;
use std::sync::atomic::Ordering;

/// Creates atomic versions of primitive types.
macro_rules! atomic_value {
  ($primitive:ty, $name:ident, $doc:expr) => {
    #[derive(Default)]
    #[doc = $doc]
    pub struct $name {
      value: base::$name,
      event: Event,
    }

    impl $name {
      /// Creates an atomic version of the given value.
      pub fn new(value: $primitive) -> Self {
        Self { value: base::$name::new(value), event: Event::new() }
      }

      /// Loads the current value.
      pub fn load(&self) -> $primitive {
        self.value.load(Ordering::Acquire)
      }

      /// Stores the given value.
      pub fn store(&self, value: $primitive) {
        self.swap(value);
      }

      /// Stores the given value and returns the previous value.
      pub fn swap(&self, value: $primitive) -> $primitive {
        let old = self.value.swap(value, Ordering::AcqRel);

        if old != value {
          self.event.notify(usize::MAX);
        }

        old
      }

      /// Waits until the current value is equal to the given value.
      pub async fn until_eq(&self, value: $primitive) {
        self.until(|v| v == value).await;
      }

      /// Waits until the value matches a predicate.
      pub async fn until(&self, mut predicate: impl FnMut($primitive) -> bool) -> $primitive {
        let mut value = self.value.load(Ordering::Relaxed);

        if predicate(value) {
          return value;
        }

        while !predicate(value) {
          let listener = self.event.listen();

          value = self.load();

          if predicate(value) {
            return value;
          }

          listener.await;
        }

        value
      }
    }

    impl From<$primitive> for $name {
      fn from(value: $primitive) -> Self {
        Self::new(value)
      }
    }
  };
}

atomic_value!(i8, AtomicI8, "An awaitable atomic `i8`.");
atomic_value!(i16, AtomicI16, "An awaitable atomic `i16`.");
atomic_value!(i32, AtomicI32, "An awaitable atomic `i32`.");
atomic_value!(i64, AtomicI64, "An awaitable atomic `i64`.");
atomic_value!(isize, AtomicIsize, "An awaitable atomic `isize`.");
atomic_value!(u8, AtomicU8, "An awaitable atomic `u8`.");
atomic_value!(u16, AtomicU16, "An awaitable atomic `u16`.");
atomic_value!(u32, AtomicU32, "An awaitable atomic `u32`.");
atomic_value!(u64, AtomicU64, "An awaitable atomic `u64`.");
atomic_value!(usize, AtomicUsize, "An awaitable atomic `usize`.");
atomic_value!(bool, AtomicBool, "An awaitable atomic `bool`.");

impl AtomicBool {
  /// Fetches the current value and replaces it with the result of a logical
  /// "and" with the given value.
  pub fn fetch_and(&self, value: bool) -> bool {
    let old = self.value.fetch_and(value, Ordering::AcqRel);

    if old != value {
      self.event.notify(usize::MAX);
    }

    old
  }

  /// Fetches the current value and replaces it with the result of a logical
  /// "or" with the given value.
  pub fn fetch_or(&self, value: bool) -> bool {
    let old = self.value.fetch_or(value, Ordering::AcqRel);

    if value && !old {
      self.event.notify(usize::MAX);
    }

    old
  }
}
