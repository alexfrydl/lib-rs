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
#[derive(Clone, Error)]
pub struct Failure {
  cause: Arc<Cause>,
}

/// The cause of a failure.
struct Cause {
  location: Option<Location>,
  message: SharedStr,
  cause: Option<Arc<Cause>>,
}

/// The location of a failure.
struct Location {
  file: Cow<'static, str>,
  line: usize,
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
        location: Some(Location { file: file.into(), line: line.as_() }),
        message: message.into(),
        cause: cause.map(|f| f.cause),
      }),
    }
  }

  /// Returns the name of the file this failure occurred in, if it is known.
  pub fn file(&self) -> Option<&str> {
    self.cause.location.as_ref().map(|loc| loc.file.as_ref())
  }

  /// Returns a temporary value which displays the failure in color.
  pub fn in_color(&self) -> fmt::InColor<&Self> {
    fmt::InColor(self)
  }

  /// Returns the line number this failure occurred on, if it is known.
  pub fn line(&self) -> Option<usize> {
    self.cause.location.as_ref().map(|loc| loc.line)
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

// Implement conversion.

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
    write!(f, "Failure({:?})", self.cause.message)
  }
}

impl Display for Failure {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let mut causes = self.causes().peekable();
    let f = fmt::IndentedFormatter::new(f, "", "  ");

    if causes.peek().and_then(f)

    for (i, cause) in causes.take_while(|c| c.location.is_none()).enumerate() {
      if i > 0 {
        write!(f, "; {}", cause);
      } else {
        write!(f, "{}", cause);
      }
    }

    if !f.alternate() {
      return Ok(());
    }

    while let Some(cause) = causes.next() {
      write!(f, "\n{}", cause)?;

      if cause.location.is_none() {
        while let Some(cause) = causes.next() {
          write!(f, "; {}", cause)?;

          if cause.location.is_some() {
            break;
          }
        }
      }
    }

    Ok(())
  }
}

impl Display for Cause {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if let Some(loc) = &self.location {
      write!(f, "at {} line {} — ", loc.file, loc.line)?;
    }

    write!(f, "{}", fmt::indent("", "  ", &self.message))
  }
}
