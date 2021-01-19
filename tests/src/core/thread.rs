// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use af_core::channel;
use af_core::test::prelude::*;
use af_core::thread;

/// Test the `stringü` module.
pub fn test(cx: &mut test::Context) {
  test!(cx, "::start()", timeout = "1 s", {
    let current = std::thread::current().id();
    let (tx, rx) = channel::with_capacity(1);

    thread::start(module_path!(), move || {
      future::try_resolve(tx.send(std::thread::current().id()));
    });

    let id = rx.recv().await?;

    fail::when!(id == current);
  });

  cx.scope("::Handle::join()", |cx| {
    test!(cx, "waits for the thread output", timeout = "1 s", {
      let thread = thread::start("fourteen", || 14);
      let output = thread.join().await?;

      fail::when!(output != 14);
    });

    test!(cx, "returns an error on panic", timeout = "1 s", {
      let thread = thread::start("panicker", || panic!("haha"));
      let output = thread.join().await;

      fail::when!(output.is_ok());
    });

    test!(cx, "does not block tasks", {
      let thread = thread::start("panicker", || {
        std::thread::sleep(std::time::Duration::from_secs(1));
      });

      let before = Time::now();
      let output = future::try_resolve(thread.join());
      let elapsed = before.elapsed();

      fail::when!(output.is_some());
      fail::when!(elapsed > Duration::ms(0.1), "Blocked.");
    });
  });

  test!(cx, "::sleep()", timeout = "1 s", {
    let duration = Duration::ms(50);

    let thread = thread::start("panicker", move || {
      thread::sleep(duration);
    });

    let time = Time::now();

    thread.join().await?;

    fail::when!(time.elapsed().as_ms() < 40.0, "Too fast.");
  });
}
