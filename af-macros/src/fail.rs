// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// Returns an `Err` containing a new `fail::Error` from format args.
#[macro_export]
macro_rules! fail {
  ($($args:tt)*) => {
    return Err(fail::err!($($args)*))
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

  (let $pattern:pat = $expr:expr) => {
    if let $pattern = $expr {
      fail!("Pattern match failed on line {} of `{}`.", line!(), file!());
    }
  };

  (let $pattern:pat = $expr:expr, $($args:tt)+) => {
    if let $pattern = $expr {
      fail!($($args)+);
    }
  };

  ($left:tt == $right:tt) => {
    if $left == $right {
      fail!(
        "`{}` equals `{}` on line {} of `{}`.",
        stringify!($left),
        stringify!($right),
        line!(),
        file!(),
      );
    }
  };

  ($actual:tt != $expected:expr) => {{
    let actual = $actual;
    let expected = $expected;

    if actual != expected {
      fail!(
        "Expected `{}` to equal `{:?}` on line {} of `{}`, but got `{:?}`.",
        stringify!($actual),
        expected,
        line!(),
        file!(),
        actual
      );
    }
  }};

  ($actual:tt . $method:ident () != $expected:expr) => {{
    let actual = $actual.$method();
    let expected = $expected;

    if actual != expected {
      fail!(
        "Expected `{}.{}()` to equal `{:?}` on line {} of `{}`, but got `{:?}`.",
        stringify!($actual),
        stringify!($method),
        expected,
        line!(),
        file!(),
        actual
      );
    }
  }};

  ($left:tt > $right:tt) => {
    if $left > $right {
      fail!(
        "`{}` is greater than `{}` on line {} of `{}`.",
        stringify!($left),
        stringify!($right),
        line!(),
        file!(),
      );
    }
  };

  ($left:tt < $right:tt) => {
    if $left < $right {
      fail!(
        "`{}` is less than `{}` on line {} of `{}`.",
        stringify!($left),
        stringify!($right),
        line!(),
        file!(),
      );
    }
  };

  ($left:tt >= $right:tt) => {
    match $left.partial_cmp(&$right) {
      Some(std::cmp::Ordering::Greater) => {
        fail!(
          "`{}` is greater than `{}` on line {} of `{}`.",
          stringify!($left),
          stringify!($right),
          line!(),
          file!(),
        );
      }

      Some(std::cmp::Ordering::Equal) => {
        fail!(
          "`{}` is equal to `{}` on line {} of `{}`.",
          stringify!($left),
          stringify!($right),
          line!(),
          file!(),
        );
      }

      _ => {}
    }
  };

  ($left:tt <= $right:tt) => {
    match $left.partial_cmp(&$right) {
      Some(std::cmp::Ordering::Less) => {
        fail!(
          "`{}` is less than `{}` on line {} of `{}`.",
          stringify!($left),
          stringify!($right),
          line!(),
          file!(),
        );
      }

      Some(std::cmp::Ordering::Equal) => {
        fail!(
          "`{}` is equal to `{}` on line {} of `{}`.",
          stringify!($left),
          stringify!($right),
          line!(),
          file!(),
        );
      }

      _ => {}
    }
  };

  ($value:tt.is_some()) => {
    if $value.is_some() {
      fail!("`{}` is `Some` on line {} of `{}`.", stringify!($value), line!(), file!());
    }
  };

  ($value:ident.is_none()) => {
    let $value = match $value {
      Some($value) => $value,
      None => fail!("`{}` is `None` on line {} of `{}`.", stringify!($value), line!(), file!()),
    };
  };

  ($value:tt.is_none()) => {
    if $value.is_none() {
      fail!("`{}` is `None` on line {} of `{}`.", stringify!($value), line!(), file!());
    }
  };

  ($value:ident.is_ok()) => {
    let $value = match $value {
      Ok(_) => fail!("`{}` is `Ok` on line {} of `{}`.", stringify!($value), line!(), file!()),
      Err(err) => err,
    };
  };

  ($value:ident.is_ok(), $($args:tt)+) => {
    let $value = match $value {
      Ok(_) => fail!($($args)+),
      Err(err) => err,
    };
  };

  ($value:tt.is_ok()) => {
    if $value.is_ok() {
      fail!("`{}` is `Ok` on line {} of `{}`.", stringify!($value), line!(), file!());
    }
  };

  ($value:tt.is_empty()) => {
    if $value.is_empty() {
      fail!("`{}` is empty on line {} of `{}`.", stringify!($value), line!(), file!());
    }
  };

  (!$value:tt.is_empty()) => {
    if !$value.is_empty() {
      fail!("`{}` is not empty on line {} of `{}`.", stringify!($value), line!(), file!());
    }
  };

  (!$value:tt) => {
    if !$value {
      fail!("`{}` is `true` on line {} of `{}`.", stringify!($value), line!(), file!());
    }
  };

  ($value:tt) => {
    if $value {
      fail!("`{}` is `true` on line {} of `{}`.", stringify!($value), line!(), file!());
    }
  };

  ($condition:expr, $($args:tt)+) => {
    if $condition {
      fail!($($args)*);
    }
  };

  ($($condition:tt)*) => {
    if $($condition)* {
      fail!("Failure on line {} of `{}`.", line!(), file!());
    }
  };
}
