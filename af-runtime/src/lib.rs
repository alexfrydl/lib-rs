// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod backend;
pub mod logger;
pub mod task;

pub use af_macros::runtime_main as main;

use af_core::prelude::*;
use af_core::sync::AtomicBool;
use once_cell::sync::Lazy;
use std::process::exit;

/// Runs the runtime until the given future completes, then exits the process.
pub fn run(future: impl Future<Output = Result> + Send + 'static) -> ! {
  // Ensure that only one runtime is running per process.

  static IS_RUNNING: Lazy<AtomicBool> = Lazy::new(default);

  if IS_RUNNING.swap(true) {
    panic!("The af-core runtime is already running.");
  }

  if let Err(err) = backend::run(future) {
    eprintln!("{}", err);
    exit(1);
  }

  exit(0)
}
