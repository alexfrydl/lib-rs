// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// Returns an `Err` containing a new `fail::Error` from format args.
#[macro_export]
macro_rules! fail {
  ($($args:tt)*) => {
    return Err(fail::err!($($args)*).into())
  };
}

/// Creates a new `fail::Error` from format args.
#[macro_export]
macro_rules! fail_err {
  ($expr:expr) => {
    fail::Error::new($expr)
  };

  ($($args:tt)*) => {
    fail::Error::new(format!($($args)*))
  };
}

/// Returns a closure for using [`Result::map_err`] to wrap an error in a
/// `fail::Error`.
#[macro_export]
macro_rules! fail_wrap {
  ($($args:tt)*) => {
    |err| fail::err!($($args)*).with_cause(err)
  };
}
