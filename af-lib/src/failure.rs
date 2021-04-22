// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A general purpose error type.

pub use af_macros::{fail, failure};

use crate::math::AsPrimitive;
use crate::prelude::*;
use crate::string::SharedStr;
use console::style;

/// A general-purpose cloneable error type.
#[derive(Clone)]
pub struct Failure {
  cause: Arc<Cause>,
}

/// The cause of a failure.
#[derive(Clone)]
struct Cause {
  file: &'static str,
  line: usize,
  message: SharedStr,
  cause: Option<Arc<Cause>>,
}

/// Represents either success (`Ok`) or failure (`Err`).
///
/// This type doesn't require any type parameters and defaults to
/// `Result<(), Failure>`.
pub type Result<T = (), E = Failure> = std::result::Result<T, E>;

impl Failure {
  /// Creates a new failure.
  pub fn new(
    file: &'static str,
    line: impl AsPrimitive<usize>,
    message: impl Into<SharedStr>,
    cause: Option<Failure>,
  ) -> Self {
    Self {
      cause: Arc::new(Cause {
        file,
        line: line.as_(),
        message: message.into(),
        cause: cause.map(|f| f.cause),
      }),
    }
  }

  /// Returns the name of the file this failure occurred in.
  pub fn file(&self) -> &'static str {
    self.cause.file
  }

  /// Returns a temporary value which displays the failure in color.
  pub fn in_color(&self) -> fmt::InColor<&Self> {
    fmt::InColor(self)
  }

  /// Returns the line number this failure occurred on.
  pub fn line(&self) -> usize {
    self.cause.line
  }

  /// Returns the failure message.
  pub fn message(&self) -> &SharedStr {
    &self.cause.message
  }

  /// Returns an iterator over all causes of the failure.
  fn causes(&self) -> impl Iterator<Item = &Cause> {
    struct Iter<'a>(Option<&'a Arc<Cause>>);

    impl<'a> Iterator for Iter<'a> {
      type Item = &'a Cause;

      fn next(&mut self) -> Option<Self::Item> {
        let item = self.0?;
        self.0 = item.cause.as_ref();
        Some(item)
      }
    }

    Iter(Some(&self.cause))
  }
}

// Implement formatting.

impl Debug for Failure {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "\"on line {} of `{}`: {}\"",
      self.cause.line,
      self.cause.file.escape_debug(),
      self.cause.message.escape_debug()
    )
  }
}

impl Display for Failure {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    Display::fmt(&self.cause, f)?;

    if f.alternate() {
      for cause in self.causes().skip(1) {
        write!(f, "\n{:#}", cause)?;
      }
    }

    Ok(())
  }
}

impl<'a> Display for fmt::InColor<&'a Failure> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    Display::fmt(&self.0.cause.in_color(), f)?;

    if f.alternate() {
      for cause in self.0.causes().skip(1) {
        write!(f, "\n{:#}", cause.in_color())?;
      }
    }

    Ok(())
  }
}

impl Display for Cause {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "at {} line {} — {}", self.file, self.line, fmt::indent("", "  ", &self.message))
  }
}

impl<'a> Display for fmt::InColor<&'a Cause> {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(
      f,
      "{} {} line {} — {}",
      style("at").black().bright(),
      style(self.0.file).green(),
      self.0.line,
      fmt::indent("", "  ", &self.0.message),
    )
  }
}
