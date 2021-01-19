// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use af_core::task;
use af_core::test::prelude::*;
use event_listener::Event;

/// Test the `future` module.
pub fn test(cx: &mut test::Context) {
  test!(cx, "::catch_unwind()", timeout = immediate, {
    let output = future::catch_unwind(async move { panic!("Hello.") }).await;

    match output {
      Err(value) => {
        let value: Box<&'static str> = match value.downcast() {
          Ok(value) => value,
          Err(_) => fail!("Value was not a `&'static str`."),
        };

        fail::when!(*value != "Hello.", "Unexpected value: {:?}.", value);
      }

      _ => fail!("Did not panic."),
    }
  });

  test!(cx, "::forever()", timeout = "1 s", {
    let forever = future::forever::<()>();

    pin!(forever);

    // Obviously can't test that it waits forever, but can test that it waits.

    task::sleep(Duration::hz(60)).await;

    fail::when!(future::poll(&mut forever).is_some(), "Completed.");
  });

  cx.scope("::poll()", |cx| {
    test!(cx, "polls the future", timeout = immediate, {
      let event = Event::new();
      let listener = event.listen();
      let notify = async move { event.notify(1) };

      pin!(notify);

      future::poll(&mut notify);

      listener.await;
    });

    test!(cx, "returns output when ready", timeout = "1 s", {
      let ready = async { "hello" };

      pin!(ready);

      let ready = future::poll(&mut ready);

      fail::when!(ready.is_none());
      fail::when!(ready != "hello");
    });
  });

  cx.scope("::race()", |cx| {
    test!(cx, "returns the first ready output", timeout = "1 s", {
      let a = async {
        task::sleep(Duration::hours(1)).await;
        "a"
      };

      let b = async {
        task::yield_now().await;
        "b"
      };

      let output = future::race(a, b).await;

      fail::when!(output != "b");
    });

    test!(cx, "prefers the first future", timeout = "1 s", {
      let a = async { "a" };
      let b = async { "b" };

      let output = future::race(a, b).await;

      fail::when!(output != "a");
    });
  });

  cx.scope("::try_resolve()", |cx| {
    test!(cx, "polls the future", timeout = immediate, {
      let event = Event::new();
      let listener = event.listen();

      future::try_resolve(async move { event.notify(1) });

      listener.await;
    });

    test!(cx, "returns output when ready", timeout = "1 s", {
      let ready = future::try_resolve(async { "hello" });

      fail::when!(ready.is_none());
      fail::when!(ready != "hello");
    });
  });

  cx.scope("::TryFutureExt", |cx| {
    test!(cx, "::map_err()", timeout = immediate, {
      let fut = async {
        fail!("inner");

        #[allow(unreachable_code)]
        Ok(())
      };

      match fut.map_err(|err| fail::err!("outer {}", err)).await {
        Ok(_) => fail!("Succeeded."),
        Err(err) => {
          fail::when!(err.to_string() != "outer inner", "Unexpected output: {:?}.", err.to_string())
        }
      }
    });
  });
}
