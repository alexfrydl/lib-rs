// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::process::exit;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::AcqRel;

use super::{scope, thread};
use crate::log;
use crate::prelude::*;

/// Runs the runtime until it exits the process.
///
/// This function is marked unsafe because it may only be called once on the
///  main thread. Use the `main` proc macro instead.
pub unsafe fn run<O, F>(module_path: &'static str, future: F) -> !
where
  O: scope::IntoOutput + 'static,
  F: Future<Output = O> + 'static,
{
  static ONCE: AtomicBool = AtomicBool::new(false);

  if ONCE.swap(true, AcqRel) {
    panic!("runtime already started");
  }

  let result = thread::run(future);

  if let Err(err) = &result {
    error!(target: module_path, "Main thread {}", fmt::indent("", "  ", err));
  }

  async_io::block_on(log::flush());

  match result {
    Ok(_) => exit(0),
    Err(_) => exit(1),
  }
}
