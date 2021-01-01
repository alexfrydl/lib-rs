// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Functions for inspecting the environment of the current process.

use crate::fs::path;
use crate::prelude::*;
use std::io;
use std::path::{Path, PathBuf};

/// One of the possible errors that can occur when reading an environment
/// variable.
#[derive(Debug, Display, Error)]
pub enum VarError {
  /// Environment variable not present.
  #[display(fmt = "Environment variable not present.")]
  NotPresent,
  /// Environment variable contains non-Unicode characters.
  #[display(fmt = "Environment variable contains non-Unicode characters.")]
  NotUnicode,
}

/// One of the possible errors returned by [`working_path()`].
#[derive(Debug, Display, Error)]
pub enum WorkingPathError {
  /// The working path was not found.
  #[display(fmt = "The current working directory was not found.")]
  NotFound,
  /// The user does not have permission to access the current working directory.
  #[display(fmt = "Permission denied reading the current working directory.")]
  PermissionDenied,
  /// The working path is not unicode.
  #[display(fmt = "The current working directory `{}` is not unicode.", "_0.display()")]
  NotUnicode(PathBuf),
}

/// The full file system path to the current executable.
static EXE_PATH: Lazy<Result<(String, String), String>> = Lazy::new(|| {
  let mut path: String = std::env::current_exe()
    .map_err(|err| format!("IO error. {}.", err))?
    .to_str()
    .ok_or_else(|| "non-unicode path name.")?
    .into();

  let file = path::pop(&mut path).unwrap_or_default();

  Ok((path, file))
});

/// The full file system path to the cargo project for the currently running
/// executable.
static PROJECT_PATH: Lazy<Option<String>> = Lazy::new(|| var("CARGO_MANIFEST_DIR").ok());

/// The full file system path to the cargo workspace of the currently running
/// executable.
static WORKSPACE_PATH: Lazy<Option<String>> = Lazy::new(|| {
  // Starting at the project path, look for a directory containing `Cargo.lock`.

  let project_path = PROJECT_PATH.as_ref()?;
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

  Some(workspace_path)
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

/// Returns true if this executable was started by `cargo run` or similar.
pub fn is_cargo_run() -> bool {
  PROJECT_PATH.is_some()
}

/// Loads environment variables from a `.env` file in the current working
/// directory or one of its parent directories.
#[cfg(feature = "dotenv")]
pub fn load_dotenv() -> bool {
  load_dotenv_from(".env")
}

/// Loads environment variables from a file in the current working directory or
/// one of its parent directories.
#[cfg(feature = "dotenv")]
pub fn load_dotenv_from(file_name: &str) -> bool {
  match dotenv_crate::from_filename(file_name) {
    Ok(path) => {
      debug!("Loaded environment variables from `{}`.", path.display());
      true
    }

    Err(dotenv_crate::Error::Io(err)) if err.kind() == io::ErrorKind::NotFound => false,

    Err(err) => {
      warn!("Failed to load environment variables. {}", err);
      false
    }
  }
}

/// Returns the full file system path to the cargo project of the currently
/// running executable.
///
/// This function panics if the executable was not run with a `cargo` command.
/// Use [`is_cargo_run()`] to check whether this function will panic.
pub fn project_path() -> &'static str {
  PROJECT_PATH.as_ref().expect("project_path is only available with `cargo run` and `cargo test`")
}

/// Returns the full file system path to the cargo workspace of the currently
/// running executable.
///
/// This function panics if the executable was not run with a `cargo` command.
/// Use [`is_cargo_run()`] to check whether this function will panic.
pub fn workspace_path() -> &'static str {
  WORKSPACE_PATH
    .as_ref()
    .map(|w| w.as_str())
    .expect("workspace_path is only available with `cargo run` and `cargo test`")
}

/// Returns the full file system path to the current working directory.
pub fn working_path() -> Result<String, WorkingPathError> {
  let path = std::env::current_dir().map_err(|err| match err.kind() {
    io::ErrorKind::NotFound => WorkingPathError::NotFound,
    io::ErrorKind::PermissionDenied => WorkingPathError::PermissionDenied,
    _ => unreachable!(),
  })?;

  Ok(path.to_str().map(String::from).ok_or_else(|| WorkingPathError::NotUnicode(path))?)
}

/// Returns the value of the given environment variable.
pub fn var(name: &str) -> Result<String, VarError> {
  std::env::var(name).map_err(|err| match err {
    std::env::VarError::NotPresent => VarError::NotPresent,
    std::env::VarError::NotUnicode(_) => VarError::NotUnicode,
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_project_path() {
    assert!(path::is_absolute(project_path()), "path is not absolute");

    assert_eq!(
      project_path(),
      var("CARGO_MANIFEST_DIR").unwrap(),
      "path does not point to project"
    );
  }

  #[test]
  fn test_workspace_path() {
    assert!(path::is_absolute(workspace_path()), "path is not absolute");

    assert_eq!(
      workspace_path(),
      path::parent(project_path()).unwrap(),
      "path does not point to workspace"
    );
  }

  #[test]
  fn test_var() {
    std::env::remove_var("__TEST_VAR");

    assert!(var("__TEST_VAR").is_err());

    std::env::set_var("__TEST_VAR", "value");

    assert_eq!(var("__TEST_VAR").unwrap(), "value");
  }

  #[test]
  fn test_working_path() {
    let working_path = working_path().unwrap();

    assert!(path::is_absolute(&working_path), "path is not absolute");

    assert!(
      path::starts_with(&working_path, workspace_path()),
      "path is outside the workspace directory"
    );
  }
}
