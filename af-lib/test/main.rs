// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use af_lib::concurrency::{fiber, join};
use af_lib::prelude::*;

/// Main entry point for tests.
#[af_lib::main]
async fn main() {
  fiber::start(async {
    time::delay(Duration::secs(10)).await;
  });

  fiber::start(async {
    time::delay(Duration::secs(10)).await;
  });

  join().await;
}
