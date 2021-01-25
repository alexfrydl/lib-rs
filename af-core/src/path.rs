// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Functions for working with Unicode file system paths.

pub use af_macros::{path_join as join, path_normalize as normalize, path_resolve as resolve};

#[doc(inline)]
pub use std::path::{is_separator, MAIN_SEPARATOR as SEPARATOR};

use crate::{env, prelude::*};
use std::cell::RefCell;
use std::path::Path;

thread_local! {
  /// A thread-local buffer for operations that need temporary storage.
  static THREAD_BUFFER: RefCell<String> = default();
}

/// Appends a relative path to a base path.
///
/// If `relative` is an absolute path, the base path is replaced completely.
pub fn append(base: &mut String, relative: &str) {
  if is_absolute(relative) {
    base.replace_range(.., relative);
    return;
  }

  match base.chars().rev().next() {
    None => base.replace_range(.., relative),
    Some(c) if is_separator(c) => base.push_str(relative),
    Some(_) => {
      base.reserve(relative.len() + 1);
      base.push(SEPARATOR);
      base.push_str(relative);
    }
  }
}

/// Returns `true` if the given path is absolute.
pub fn is_absolute(path: &str) -> bool {
  as_std(path).is_absolute()
}

/// Joins a base path and a relative path.
///
/// If `relative` is an absolute path, it is returned unmodified.
pub fn join<'a, 'b>(base: impl PathLike<'b>, relative: impl PathLike<'a>) -> Cow<'a, str> {
  let relative = relative.to_cow();

  if is_absolute(&relative) {
    return relative;
  }

  let mut output = base.to_owned();

  append(&mut output, &relative);

  output.into()
}

/// Returns the last component of the path.
///
/// If `path` is a root or empty path, this function returns `None`.
pub fn last(path: &str) -> Option<&str> {
  as_std(path).file_name()?.to_str()
}

/// Normalizes a path.
pub fn normalize(path: &mut String) {
  THREAD_BUFFER.with(|buffer| {
    let mut buffer = buffer.borrow_mut();

    buffer.clear();

    normalize_into(path, &mut buffer);

    path.replace_range(.., &buffer);
  })
}

/// Returns a normalized version of the given path.
pub fn normalized<'a>(path: impl PathLike<'a>) -> Cow<'a, str> {
  let path = path.to_cow();

  THREAD_BUFFER.with(|buffer| {
    let mut buffer = buffer.borrow_mut();

    buffer.clear();
    normalize_into(&path, &mut buffer);

    match path.as_ref() == *buffer {
      true => path,
      false => buffer.clone().into(),
    }
  })
}

/// Returns the parent of the given path.
///
/// If `path` is a root or empty path, this function returns `None`.
pub fn parent(path: &str) -> Option<&str> {
  as_std(path).parent()?.to_str()
}

/// Removes the last component from the path and returns it.
///
/// If the path is a root or empty path, this function does nothing and returns
/// `None`.
pub fn pop(path: &mut String) -> Option<String> {
  let split_at = parent(&path)?.len();
  let lead_seps = path[split_at..].chars().take_while(|c| is_separator(*c)).count();
  let trail_seps = path.chars().rev().take_while(|c| is_separator(*c)).count();
  let mut last = path.split_off(split_at + lead_seps);

  path.truncate(split_at);
  last.truncate(last.len() - trail_seps);

  Some(last)
}

/// Resolves the given path into an absolute, normalized path.
pub fn resolve(path: &mut String) -> Result<(), env::WorkingPathError> {
  if !is_absolute(&path) {
    let mut buf = env::working_path()?;

    mem::swap(path, &mut buf);
    append(path, &buf);
  }

  normalize(path);

  Ok(())
}

/// Returns an absolute, normalized version of the given path.
pub fn resolved<'a>(path: impl PathLike<'a>) -> Result<Cow<'a, str>, env::WorkingPathError> {
  let mut path = path.to_cow();

  if is_absolute(&path) {
    return Ok(normalized(path));
  }

  resolve(path.to_mut())?;

  Ok(path)
}

/// Returns `true` if the first path starts with the second path.
pub fn starts_with(path: &str, prefix: &str) -> bool {
  as_std(path).starts_with(prefix)
}

/// Returns the given path with a trailing separator if it does not already
/// have one.
pub fn with_trailing_sep<'a>(path: impl PathLike<'a>) -> Cow<'a, str> {
  let mut path = path.to_cow();

  match path.chars().rev().next() {
    Some(c) if is_separator(c) => path,

    _ => {
      path.to_mut().push(SEPARATOR);
      path
    }
  }
}

/// Converts a value into a `&Path`.
pub fn as_std(path: &str) -> &Path {
  path.as_ref()
}

/// Normalizes the given path into the output string.
///
/// The output is expected to already be empty.
fn normalize_into(path: &str, output: &mut String) {
  for component in as_std(path).components() {
    match component {
      std::path::Component::CurDir => continue,
      std::path::Component::Normal(component) => append(output, component.to_str().unwrap()),
      std::path::Component::Prefix(prefix) => output.push_str(prefix.as_os_str().to_str().unwrap()),
      std::path::Component::RootDir => output.push(SEPARATOR),

      std::path::Component::ParentDir => {
        pop(output);
      }
    }
  }
}

/// A trait for values that can be used in path operations.
pub trait PathLike<'a>: Sized {
  /// Converts this value into a `Cow<str>`.
  fn to_cow(self) -> Cow<'a, str>;

  /// Converts this value into a `String`.
  fn to_owned(self) -> String {
    self.to_cow().into()
  }
}

// Implement `PathLike` for common string types.

impl<'a> PathLike<'a> for &'a str {
  fn to_cow(self) -> Cow<'a, str> {
    self.into()
  }
}

impl<'a> PathLike<'a> for &'a &'_ mut str {
  fn to_cow(self) -> Cow<'a, str> {
    (&**self).into()
  }
}

impl<'a> PathLike<'a> for String {
  fn to_cow(self) -> Cow<'a, str> {
    self.into()
  }
}

impl<'a> PathLike<'a> for &'a String {
  fn to_cow(self) -> Cow<'a, str> {
    self.into()
  }
}

impl<'a> PathLike<'a> for &'a &'_ mut String {
  fn to_cow(self) -> Cow<'a, str> {
    (&**self).into()
  }
}

impl<'a> PathLike<'a> for Cow<'a, str> {
  fn to_cow(self) -> Cow<'a, str> {
    self
  }
}

impl<'a> PathLike<'a> for &'a Cow<'_, str> {
  fn to_cow(self) -> Cow<'a, str> {
    self.as_ref().into()
  }
}

impl<'a> PathLike<'a> for &'a &'_ mut Cow<'_, str> {
  fn to_cow(self) -> Cow<'a, str> {
    self.as_ref().into()
  }
}

impl<'a> PathLike<'a> for &'a std::path::PathBuf {
  fn to_cow(self) -> Cow<'a, str> {
    self.to_string_lossy().to_cow()
  }
}
