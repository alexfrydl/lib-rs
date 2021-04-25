// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A general purpose error type.

pub use af_macros::{fail, failure};

use crate::math::AsPrimitive;
use crate::prelude::*;
use crate::util::{Panic, SharedStr};

/// A general-purpose cloneable error type.
#[derive(Clone)]
pub struct Failure {
  cause: Arc<Cause>,
}

/// The cause of a failure.
struct Cause {
  file: Cow<'static, str>,
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
    file: impl Into<Cow<'static, str>>,
    line: impl AsPrimitive<usize>,
    message: impl Into<SharedStr>,
    cause: Option<Failure>,
  ) -> Self {
    Self {
      cause: Arc::new(Cause {
        file: file.into(),
        line: line.as_(),
        message: message.into(),
        cause: cause.map(|c| c.cause),
      }),
    }
  }

  /// Returns the name of the file this failure occurred in.
  pub fn file(&self) -> &str {
    self.cause.file.as_ref()
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

// Implement conversion from panics.

impl From<Panic> for Failure {
  fn from(panic: Panic) -> Self {
    match panic.message {
      Some(message) => Self::new(panic.file, panic.line, format!("panicked: {}", message), None),
      None => Self::new(panic.file, panic.line, "panicked", None),
    }
  }
}

// Implement formatting.

impl Debug for Failure {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut s = f.debug_struct("Failure");

    s.field("file", &self.cause.file);
    s.field("line", &self.cause.line);
    s.field("message", &self.cause.message);

    s.finish()
  }
}

impl Display for Failure {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut causes = self.causes();

    write!(f, "{}", causes.next().unwrap())?;

    for cause in causes {
      write!(f, "\n{}", cause)?;
    }

    Ok(())
  }
}

impl Display for Cause {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "at {} line {}\n{}", self.file, self.line, fmt::indent("  ", "  ", &self.message))
  }
}
