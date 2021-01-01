// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Provides the `fail!` macro and a generic cloneable error type.

pub use af_macros::{fail, fail_err as err, fail_with as with};

use crate::prelude::*;
use arrayvec::ArrayString;

/// A generic cloneable error.
#[derive(Clone)]
pub struct Error {
  message: Arc<str>,
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

/// Normalizes a message for [`fail::Error`]. This function is used to support
/// `fail::with!`.
fn normalize_message<'a>(message: &mut String, start_at: usize) {
  // Capitalize the first letter of error messages.

  let first = message[start_at..].chars().next().unwrap_or_default();

  if first.is_alphabetic() {
    let mut upper: ArrayString<[_; 16]> = default();

    for c in first.to_uppercase() {
      upper.push(c);
    }

    message.replace_range(start_at..start_at + first.len_utf8(), &upper);
  }

  // Append a period to the end of error messages.

  let last = message.chars().rev().next().unwrap_or_default();

  if !last.is_whitespace() && last != '.' {
    message.push('.');
  }
}

impl Error {
  #[doc(hidden)]
  /// Joins a message and an error into a new error.
  ///
  /// This function is used to support the [`with!`] macro.
  pub fn join(message: impl Display, err: impl Display) -> Self {
    // First write the message to the buffer and normalize it.

    let mut buffer = String::with_capacity(256);

    write!(&mut buffer, "{:#}", message).unwrap();

    fail::normalize_message(&mut buffer, 0);

    // Then write the error to the buffer and normalize that.

    let start_at = buffer.len() + 1;

    write!(&mut buffer, " {:#}", err).unwrap();

    fail::normalize_message(&mut buffer, start_at);

    // Return an error from the buffer.

    Self { message: buffer.into() }
  }

  /// Creates a new error with the given message.
  pub fn new(message: impl Into<String>) -> Self {
    let mut message = message.into();

    normalize_message(&mut message, 0);

    Self { message: message.into() }
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

// Implement serialization to string.

impl serde::Serialize for Error {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    self.message.serialize(serializer)
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
    Display::fmt(&self.message, f)
  }
}

// Unit tests.

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new() {
    assert_eq!(Error::new("test").to_string(), "Test.");
    assert_eq!(Error::new("X { }").to_string(), "X { }.");
  }
}
