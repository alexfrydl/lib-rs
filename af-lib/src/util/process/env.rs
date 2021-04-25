//! Provides access to environment variables and well-known paths.

use std::ffi::OsString;

use crate::prelude::*;

/// Returns the value of an environment variable.
pub fn get(name: &str) -> Result<String, GetError> {
  std::env::var(name).map_err(|err| match err {
    std::env::VarError::NotPresent => GetError::NotPresent,
    std::env::VarError::NotUnicode(value) => GetError::NotUnicode(value),
  })
}

/// Returns the value of an environment variable as an `OsString` if it is
/// present.
pub fn get_os(name: &str) -> Option<OsString> {
  std::env::var_os(name)
}

/// One of the possible errors returned when reading an environment variable.
#[derive(Debug, Error)]
pub enum GetError {
  /// Environment variable not present.
  #[error("not present")]
  NotPresent,
  /// Environment variable contains non-Unicode characters.
  #[error("contains non-Unicode characters")]
  NotUnicode(OsString),
}
