// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use af_lib::concurrency::{fiber, join};
use af_lib::prelude::*;
use af_lib::time::Duration;

/// Main entry point for tests.
#[af_lib::main]
async fn main() {
  defer! {
    // Runs at the end of the block.
  };

  fiber::start(async {
    Duration::seconds(1).elapsed().await;
    info!("One!");
  });

  fiber::start(async {
    Duration::seconds(1).elapsed().await;
    info!("Two!");
  });

  join().await;
}
