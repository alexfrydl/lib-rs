// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Common utilities related to the currently running process.

pub mod env;

use std::ffi::OsString;
use std::io;
use std::sync::atomic::AtomicI8;
use std::sync::atomic::Ordering::{Acquire, Release};

use crate::fs;
use crate::prelude::*;

/// The full file system path to the current executable and its file name.
static EXE_PATH: Lazy<Result<(String, String), String>> = Lazy::new(|| {
  let mut path: String = std::env::current_exe()
    .map_err(|err| err.to_string())?
    .to_str()
    .ok_or("non-unicode path name")?
    .into();

  let file = fs::path::pop(&mut path).unwrap_or_default();

  Ok((path, file))
});

/// The exit code to use when the process exits normally.
static EXIT_CODE: Lazy<AtomicI8> = Lazy::new(default);

/// Returns the file name of the currently running executable.
pub fn exe_name() -> &'static str {
  &EXE_PATH.as_ref().expect("failed to determine path to current executable").1
}

/// Returns the full file system path to the directory containing the currently
/// running executable.
pub fn exe_path() -> &'static str {
  &EXE_PATH.as_ref().expect("failed to determine path to current executable").0
}

/// Exit the process immediately, without running destructors.
pub fn exit(code: i8) -> ! {
  std::process::exit(code as i32)
}

/// Gets the exit code to use when the process exits normally.
pub(crate) fn get_exit_code() -> i8 {
  EXIT_CODE.load(Acquire)
}

/// Returns the full file system path to the cargo project of the currently
/// running executable.
///
/// If the current executable was not run with a `cargo` command, this function
/// returns `None`.
pub fn project_path() -> Option<&'static str> {
  static PATH: Lazy<Option<&'static str>> =
    Lazy::new(|| match std::env::var_os("CARGO_MANIFEST_DIR")?.into_string() {
      Ok(path) => Some(Box::leak(path.into_boxed_str())),
      Err(path) => {
        warn!("The project path contains non-Unicode characters: `{:?}`.", path);
        None
      }
    });

  *PATH
}

/// Sets the exit code to use when the process exits normally.
///
/// This value is overridden by an explicit call to [`exit()`].
pub fn set_exit_code(code: i8) {
  EXIT_CODE.store(code, Release);
}

/// Returns the full file system path to the current working directory.
pub fn working_path() -> Result<String, WorkingPathError> {
  let path = std::env::current_dir().map_err(|err| match err.kind() {
    io::ErrorKind::NotFound => WorkingPathError::NotFound,
    io::ErrorKind::PermissionDenied => WorkingPathError::PermissionDenied,
    _ => panic!("{}", err),
  })?;

  match path.into_os_string().into_string() {
    Ok(string) => Ok(string),
    Err(path) => Err(WorkingPathError::NotUnicode(path.into())),
  }
}

/// Returns the full file system path to the cargo workspace of the currently
/// running executable.
///
/// If the currently running executable was not started by `cargo run` or a
/// similar command, this function returns `None`.
pub fn workspace_path() -> Option<&'static str> {
  static PATH: Lazy<Option<&'static str>> = Lazy::new(|| {
    // Starting at the project path, look for a directory containing `Cargo.lock`.

    let project_path = project_path()?;
    let mut workspace_path: String = project_path.into();

    loop {
      fs::path::append(&mut workspace_path, "Cargo.lock");

      let found = fs::is_file(&workspace_path).unwrap_or(false);

      fs::path::pop(&mut workspace_path);

      if found {
        break;
      }

      // Try the parent directory next. If there's no parent directory, default to
      // the project path.

      if fs::path::pop(&mut workspace_path).is_none() {
        workspace_path.replace_range(.., project_path);
        break;
      }
    }

    Some(Box::leak(workspace_path.into_boxed_str()))
  });

  *PATH
}

/// One of the possible errors returned by [`working_path()`].
#[derive(Debug, Clone, Error)]
pub enum WorkingPathError {
  /// The working path was not found.
  #[error("not found")]
  NotFound,
  /// The user does not have permission to access the current working directory.
  #[error("permission denied")]
  PermissionDenied,
  /// The working path is not unicode.
  #[error("path contains non-Unicode characters")]
  NotUnicode(OsString),
  /// Some other error occurred.
  #[error("{0}")]
  Other(String),
}
