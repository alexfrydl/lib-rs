// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! File system utilities.

pub mod path;

use std::io;

use self::path::PathLike;

/// Checks whether a path exists.
pub fn exists<'a>(path: impl PathLike<'a>) -> Result<bool, io::Error> {
  match std::fs::metadata(&*path.to_cow()) {
    Ok(_) => Ok(true),
    Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(false),
    Err(err) => Err(err),
  }
}

/// Checks whether a path refers to a directory.
pub fn is_dir<'a>(path: impl PathLike<'a>) -> Result<bool, io::Error> {
  match std::fs::metadata(&*path.to_cow()) {
    Ok(m) => Ok(m.is_dir()),
    Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(false),
    Err(err) => Err(err),
  }
}

/// Checks whether a path refers to a file.
pub fn is_file<'a>(path: impl PathLike<'a>) -> Result<bool, io::Error> {
  match std::fs::metadata(&*path.to_cow()) {
    Ok(m) => Ok(m.is_file()),
    Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(false),
    Err(err) => Err(err),
  }
}
