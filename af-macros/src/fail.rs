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

/// Returns a `fail::Error` if a condition is `true`.
///
/// Some simple patterns have default error messages.
#[macro_export]
macro_rules! fail_when {
  (let Err($err:ident) = $value:ident, $($args:tt)+) => {
    let $value = match $value {
      Ok($value) => $value,
      Err($err) => fail!($($args)+),
    };
  };

  (let $pattern:pat = $expr:expr, $($args:tt)+) => {
    if let $pattern = $expr {
      fail!($($args)+);
    }
  };

  ($left:tt == $right:expr) => {
    if $left == $right {
      fail!("`{}` equals `{}`.", stringify!($left), stringify!($right));
    }
  };

  ($left:tt != $right:expr) => {
    if $left != $right {
      fail!("`{}` does not equal `{}`.", stringify!($left), stringify!($right));
    }
  };

  ($left:tt > $right:expr) => {
    if $left > $right {
      fail!("`{}` is greater than `{}`.", stringify!($left), stringify!($right));
    }
  };

  ($left:tt < $right:expr) => {
    if $left < $right {
      fail!("`{}` is less than `{}`.", stringify!($left), stringify!($right));
    }
  };

  ($left:tt >= $right:expr) => {
    match $left.partial_cmp(&$right) {
      Some(std::cmp::Ordering::Greater) => {
        fail!("`{}` is greater than `{}`.", stringify!($left), stringify!($right));
      }

      Some(std::cmp::Ordering::Equal) => {
        fail!("`{}` is equal to `{}`.", stringify!($left), stringify!($right));
      }

      _ => {}
    }
  };

  ($left:tt <= $right:expr) => {
    match $left.partial_cmp(&$right) {
      Some(std::cmp::Ordering::Less) => {
        fail!("`{}` is less than `{}`.", stringify!($left), stringify!($right));
      }

      Some(std::cmp::Ordering::Equal) => {
        fail!("`{}` is equal to `{}`.", stringify!($left), stringify!($right));
      }

      _ => {}
    }
  };

  ($value:tt.is_some()) => {
    if $value.is_some() {
      fail!("`{}` is `Some(_)`.", stringify!($value));
    }
  };

  ($value:ident.is_none()) => {
    let $value = match $value {
      Some($value) => $value,
      None => fail!("`{}` is `None`.", stringify!($value)),
    };
  };

  ($value:tt.is_none()) => {
    if $value.is_none() {
      fail!("`{}` is `None`.", stringify!($value));
    }
  };

  ($value:tt.is_ok()) => {
    if $value.is_ok() {
      fail!("`{}` is not an error.", stringify!($value));
    }
  };

  ($value:ident.is_err()) => {
    let $value = match $value {
      Ok($value) => $value,
      Err(err) => return Err(fail::from(err)),
    };
  };

  ($value:tt.is_err()) => {
    if $value.is_err() {
      Err(fail::from($value))
    }
  };

  ($value:tt.is_empty()) => {
    if $value.is_empty() {
      fail!("`{}` is empty.", stringify!($value));
    }
  };

  (!$value:tt.is_empty()) => {
    if !$value.is_empty() {
      fail!("`{}` is not empty.", stringify!($value));
    }
  };

  ($condition:expr, $($args:tt)+) => {
    if $condition {
      fail!($($args)*);
    }
  };
}
