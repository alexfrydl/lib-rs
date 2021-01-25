// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Functions for inspecting the environment of the current process.

use crate::path;
use crate::prelude::*;
use std::io;
use std::path::{Path, PathBuf};

/// One of the possible errors that can occur when reading an environment
/// variable.
#[derive(Debug, Error)]
pub enum VarError {
  /// Environment variable not present.
  #[error("Environment variable not present.")]
  NotPresent,
  /// Environment variable contains non-Unicode characters.
  #[error("Environment variable contains non-Unicode characters.")]
  NotUnicode,
}

/// One of the possible errors returned by [`working_path()`].
#[derive(Debug, Error)]
pub enum WorkingPathError {
  /// The working path was not found.
  #[error("The current working directory was not found.")]
  NotFound,
  /// The user does not have permission to access the current working directory.
  #[error("Permission denied reading the current working directory.")]
  PermissionDenied,
  /// The working path is not unicode.
  #[error("The current working directory `{}` is not unicode.", .0.display())]
  NotUnicode(PathBuf),
}

/// The full file system path to the current executable.
static EXE_PATH: Lazy<Result<(String, String), String>> = Lazy::new(|| {
  let mut path: String = std::env::current_exe()
    .map_err(|err| format!("IO error. {}.", err))?
    .to_str()
    .ok_or("non-unicode path name.")?
    .into();

  let file = path::pop(&mut path).unwrap_or_default();

  Ok((path, file))
});

/// Returns the file name of the currently running executable.
pub fn exe_name() -> &'static str {
  &EXE_PATH.as_ref().expect("Failed to determine path to current executable").1
}

/// Returns the full file system path to directory containing the currently
/// running executable.
pub fn exe_path() -> &'static str {
  &EXE_PATH.as_ref().expect("Failed to determine path to current executable").0
}

/// Returns the full file system path to the cargo project of the currently
/// running executable.
///
/// This function panics if the executable was not run with a `cargo` command.
/// Use [`is_cargo_run()`] to check whether this function will panic.
pub fn project_path() -> Option<&'static str> {
  static PATH: Lazy<Option<&'static str>> = Lazy::new(|| {
    let value = var("CARGO_MANIFEST_DIR").ok()?;

    Some(Box::leak(value.into_boxed_str()))
  });

  *PATH
}

/// Returns the value of the given environment variable.
pub fn var(name: &str) -> Result<String, VarError> {
  std::env::var(name).map_err(|err| match err {
    std::env::VarError::NotPresent => VarError::NotPresent,
    std::env::VarError::NotUnicode(_) => VarError::NotUnicode,
  })
}

/// Returns the full file system path to the current working directory.
pub fn working_path() -> Result<String, WorkingPathError> {
  let path = std::env::current_dir().map_err(|err| match err.kind() {
    io::ErrorKind::NotFound => WorkingPathError::NotFound,
    io::ErrorKind::PermissionDenied => WorkingPathError::PermissionDenied,
    _ => panic!("{}", err),
  })?;

  Ok(path.to_str().map(String::from).ok_or(WorkingPathError::NotUnicode(path))?)
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
      path::append(&mut workspace_path, "Cargo.lock");

      let found = AsRef::<Path>::as_ref(&workspace_path).exists();

      path::pop(&mut workspace_path);

      if found {
        break;
      }

      // Try the parent directory next. If there's no parent directory, default to
      // the project path.

      if path::pop(&mut workspace_path).is_none() {
        workspace_path.replace_range(.., project_path);
        break;
      }
    }

    Some(Box::leak(workspace_path.into_boxed_str()))
  });

  *PATH
}
