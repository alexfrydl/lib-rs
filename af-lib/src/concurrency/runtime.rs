// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::AcqRel;

use super::scope;
use crate::prelude::*;
use crate::util::log;

/// Runs an async operation on the concurrency runtime and then exit the
/// process.
///
/// This function is marked unsafe because it must only be called once on the
///  main thread. Use the `main` proc macro instead.
pub unsafe fn run<O, F>(module_path: &'static str, op: F) -> !
where
  O: scope::IntoOutput + 'static,
  F: Future<Output = O> + 'static,
{
  static ONCE: AtomicBool = AtomicBool::new(false);

  if ONCE.swap(true, AcqRel) {
    panic!("runtime already started");
  }

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
