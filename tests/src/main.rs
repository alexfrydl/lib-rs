// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use af_core::prelude::*;
use af_core::{task, test};

#[af_core::test::main]
fn main(cx: &mut test::Context) {
  cx.context("Sleep", |cx| {
    for i in 0..100 {
      let duration = Duration::secs(i as f64 * 0.1);
      let name = format!("for {}", duration);

      cx.test(name, async move {
        task::sleep(duration / 2.0).await;

        match random::range(1..=20) {
          1 => fail!("Critical miss."),
          _ => {}
        }

        Ok(())
      });
    }
  });
}
