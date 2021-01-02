// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// Joins multiple paths together.
#[macro_export]
macro_rules! path_join {
  ($first:expr, $($path:expr),+) => {{
    let mut path = String::from($first);

    $(af_core::fs::path::push(&mut path, $path);)*

    path
  }};

  ($path:expr) => {
    $path
  };
}

/// Joins multiple paths together and then normalizes the result.
#[macro_export]
macro_rules! path_normalize {
  ($($args:tt)*) => {
    af_core::fs::path::normalized(af_core::fs::path::join!($($args:tt)*))
  };
}

/// Joins multiple paths together and then resolves the result.
#[macro_export]
macro_rules! path_resolve {
  ($($args:tt)*) => {
    af_core::fs::path::resolved(af_core::fs::path::join!($($args:tt)*))
  };
}
