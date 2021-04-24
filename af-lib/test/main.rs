// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use af_lib::concurrency::{fiber, task, thread};
use af_lib::prelude::*;

/// Main entry point for tests.
#[af_lib::main]
async fn main() {
  fiber::start("a", async {
    time::delay(Duration::ms(10)).await;
    info!("Hello world!");
    Ok(())
  });

  fiber::start("b", async {
    time::delay(Duration::ms(10)).await;
    fail!("oh fuck");
  });
}
