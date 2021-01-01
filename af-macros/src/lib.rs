// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod fail;
mod logger;
mod path;
mod sync;

/// Runs a block of code and catches `?` operator returns.
///
/// This is just a cleaner version of creating a closure and immediately calling
/// it. It is intended to serve as a replacement for `try { .. }` until it is
/// stable.
#[macro_export]
macro_rules! attempt {
  ($($tokens:tt)+) => {
    (|| { $($tokens)+ })()
  };
}

/// Runs a block of async code and catches `?` operator returns.
///
/// This is just a cleaner version of creating a closure and immediately calling
/// it. It is intended to serve as a replacement for `try { .. }` until it is
/// stable.
#[macro_export]
macro_rules! attempt_async {
  ($($tokens:tt)+) => {
    (|| async { $($tokens)+ })().await
  };
}
