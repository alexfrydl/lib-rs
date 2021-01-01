// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::*;

macro_rules! vector {
  ($name:ident : $($x:ident = $i:expr),+ ; $name_str:expr, $doc_str:expr) => {
    #[derive(Copy, Clone, PartialEq, Eq)]
    #[repr(C)]
    #[doc = $doc_str]
    pub struct $name<T> {
      $(pub $x: T),+
    }

    impl<T: Number> $name<T> {
      pub fn new($($x: T),+) -> Self {
        Self { $($x),+ }
      }
    }

    impl<T: Number> Index<usize> for $name<T> {
      type Output = T;

      fn index(&self, index: usize) -> &T {
        match index {
          $($i => &self.$x,)+
          _ => panic!("Component index `{}` is out of range for `{}`.", index, $name_str),
        }
      }
    }

    impl<T: Number> Mul<Self> for $name<T> {
      type Output = T;

      fn mul(self, rhs: Self) -> T {
        let mut sum = zero();
        $(sum += self.$x * rhs.$x;)+
        sum
      }
    }

    impl From<$name<u16>> for $name<f32> {
      fn from(value: $name<u16>) -> Self {
        Self { $($x: value.$x as f32,)+ }
      }
    }

    impl<T: Debug> Debug for $name<T> {
      fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_tuple("")
          $(.field(&self.$x))+
          .finish()
      }
    }
  };
}

vector!(Vector2: x = 0, y = 1; "Vector2", "A two-dimensional vector.");
vector!(Vector3: x = 0, y = 1, z = 2; "Vector3", "A three-dimensional vector.");
vector!(Vector4: x = 0, y = 1, z = 2, w = 3; "Vector4", "A four-dimensional vector.");
