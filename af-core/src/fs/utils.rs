// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::prelude::*;
use crate::sync::Semaphore;

/// A shared semaphore to limit the number of concurrent file system operations.
static SEMAPHORE: Lazy<Semaphore> = Lazy::new(|| Semaphore::new(8));

/// Finds and returns all paths matching the given glob pattern.
///
/// This function ignores matching paths with non-Unicode characters.
pub async fn glob(pattern: impl Into<String>) -> Result<Vec<String>> {
  const MATCH_OPTS: glob::MatchOptions = glob::MatchOptions {
    // Case-insensitive on Windows.
    case_sensitive: !cfg!(target_os = "windows"),
    // Don't match `/` with wildcards `*` and `?`.
    require_literal_separator: true,
    // Require explicit `.` to match dotfiles.
    require_literal_leading_dot: true,
  };

  let pattern = pattern.into();
  let _permit = SEMAPHORE.acquire().await;

  future::unblock! {
    // Find all matching files.

    let paths = glob::glob_with(&pattern, MATCH_OPTS)?;

    // Collect paths as strings, ignoring any with non-Unicode characters.

    let mut output: Vec<String> = default();

    for path in paths {
      match path {
        Ok(path) =>

      if let Some(path) = path.to_str() {
        output.push(path.into());
      },

      Err(err) => fail!("{} (at `{}`).", err.error(), err.path().display()),
    }
  }

    Ok(output)
  }
}
