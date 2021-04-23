// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use af_lib::log;
use af_lib::prelude::*;

/// Main entry point for tests.
fn main() {
  log::init!();

  future::block_on(async {
    let result = future::capture_panic(async {
      let test: Option<bool> = None;

      test.expect("Hello world!");
    });

    if let Err(err) = result.await {
      error!("{}", err);
    }
  });

  std::thread::sleep_ms(100);
}
