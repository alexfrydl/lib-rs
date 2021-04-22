// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use af_lib::prelude::*;

/// Main entry point for tests.
fn main() {
  af_lib::log::init!();

  if let Err(err) = test() {
    error!("Test failed.\n{:#}", fmt::indent("  ", "  ", err.in_color()));
  }

  std::thread::sleep(Duration::secs(1).into());
}

fn test() -> Result {
  if let Err(err) = nested() {
    fail!(err, "This will have a trace.");
  }

  Ok(())
}

fn nested() -> Result {
  fail!("Shit broke.");
}
