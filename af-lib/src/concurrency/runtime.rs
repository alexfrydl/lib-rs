// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Concurrency runtime plumbing not intended for end users.

use super::scope;
use crate::prelude::*;
use crate::util::log;

/// Runs an async operation as a scope and then exits the process.
pub fn run<O, F>(module_path: &'static str, op: F) -> !
where
  O: scope::IntoOutput + 'static,
  F: Future<Output = O> + 'static,
{
  let result = scope::run_sync(op);

  if let Err(err) = &result {
    error!(target: module_path, "Main thread {}", err);
  }

  async_io::block_on(log::flush());

  let code = process::get_exit_code();

  if code == 0 && result.is_err() {
    process::exit(-1);
  }

  process::exit(code);
}
