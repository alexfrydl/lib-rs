// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use af_lib::concurrency::{fiber, join};
use af_lib::prelude::*;
use af_lib::time::{timeout, Duration};

/// Main entry point for tests.
#[af_lib::main]
async fn main() -> Result {
  fiber::start(async {
    Duration::seconds(1).elapsed().await;
    info!("One!");
  });

  fiber::start(async {
    Duration::seconds(1).elapsed().await;
    info!("Two!");
  });

  if let Err(err) = timeout(Duration::milliseconds(10), join()).await {
    fail!("Failed to wait for fibers: {}.", err);
  }

  Ok(())
}
