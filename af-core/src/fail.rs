// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Provides the `fail!` macro and a generic cloneable error type.

pub use af_macros::{fail, fail_err as err, fail_when as when, fail_wrap as wrap};

use crate::prelude::*;
use crate::string::SharedString;

/// A generic cloneable error.
#[derive(Clone)]
pub struct Error {
  message: SharedString,
  trace: im::Vector<SharedString>,
}

/// Represents either success (`Ok`) or failure (`Err`).
///
/// This type doesn't require any type parameters and defaults to
/// `Result<(), fail::Error>`.
pub type Result<T = (), E = Error> = std::result::Result<T, E>;

/// Create a new [`Error`] from a given error.
pub fn from<T: Into<Error>>(err: T) -> Error {
  err.into()
}

impl Error {
  /// Creates a new error with the given message.
  pub fn new(message: impl Into<SharedString>) -> Self {
    Self { message: message.into(), trace: default() }
  }

  /// Sets the cause of this error.
  pub fn set_cause(&mut self, cause: impl Into<Self>) {
    let cause = cause.into();

    self.trace = cause.trace;
    self.trace.push_front(cause.message);
  }

  /// Returns a copy of this error with a new cause.
  pub fn with_cause(mut self, cause: impl Into<Self>) -> Self {
    self.set_cause(cause);
    self
  }
}

// Implement `From` to convert from other types of errors.

impl<T> From<T> for Error
where
  T: std::error::Error,
{
  fn from(err: T) -> Self {
    Self::new(err.to_string())
  }
}

// Implement formatting.

impl Debug for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    Debug::fmt(&self.message, f)
  }
}

impl Display for Error {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    Display::fmt(&self.message, f)?;

    if f.alternate() {
      for err in &self.trace {
        write!(f, "\n * {:#}", err)?;
      }
    }

    Ok(())
  }
}
