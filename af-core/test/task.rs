// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use af_core::channel;
use af_core::task;
use af_core::test::prelude::*;

/// Test the `stringü` module.
pub fn test(cx: &mut test::Context) {
  test!(cx, "::start()", timeout = "1 s", {
    let (tx, rx) = channel::with_capacity(1);
    let _task = task::start(async move { tx.try_send(true) });

    fail::when!(!rx.recv().await?);
  });

  test!(cx, "::sleep()", timeout = "1 s", {
    let time = Time::now();

    task::sleep(Duration::ms(60)).await;

    fail::when!(time.elapsed().as_ms() < 40.0, "Too fast.");
  });

  test!(cx, "::yied_now()", timeout = "1 s", {
    let yield_now = task::yield_now();

    pin!(yield_now);

    fail::when!(future::poll(&mut yield_now).is_some(), "Finished immediately.");
    fail::when!(future::poll(&mut yield_now).is_none(), "Yielded twice.");
  });

  cx.scope("::Task", |cx| {
    cx.scope("::join()", |cx| {
      test!(cx, "waits for the thread output", timeout = "1 s", {
        let task = task::start(async { "pong" });
        let output = task.join().await?;

        fail::when!(output != "pong");
      });

      test!(cx, "returns an error on panic", timeout = "1 s", {
        let task = task::start(async { panic!("haha") });
        let output = task.join().await;

        fail::when!(output.is_ok());
      });
    });
  });

  cx.scope("::Join", test_join);
  cx.scope("::TryJoin", test_try_join);
}

/// Tests the `Join` struct.
fn test_join(cx: &mut test::Context) {
  test!(cx, "waits for task results", {
    let mut join = task::Join::new();

    let a_index = join.add(async { "Hello" });
    let a = join.next().await;

    fail::when!(a.is_none());
    fail::when!(a.index != a_index, "`a.index` is wrong.");
    fail::when!(a.name.as_str() != "", "`a.name` is wrong.");

    let b_index = join.add_as("Bro", async { "World" });
    let b = join.next().await;

    fail::when!(b.is_none());
    fail::when!(b.index != b_index, "`b.index` is wrong.");
    fail::when!(b.name.as_str() != "Bro", "`b.name` is wrong.");

    fail::when!(a_index == b_index);
  });

  test!(cx, "yields results in any order", {
    let (tx, rx) = channel::unbounded::<()>();

    let mut join = task::Join::new();

    join.add_as("one", async move {
      rx.recv().await.ok();
    });

    join.add_as("two", async move {
      drop(tx);
    });

    let two = join.next().await;
    let one = join.next().await;

    fail::when!(one.is_none());
    fail::when!(one.index != 0, "`one.index` is incorrect.");
    fail::when!(one.name.as_str() != "one", "`one.name` is incorrect.");

    fail::when!(two.is_none());
    fail::when!(two.index != 1, "`two.index` is incorrect.");
    fail::when!(two.name.as_str() != "two", "`two.name` is incorrect.");
  });

  test!(cx, "reports panics", {
    let mut join = task::Join::new();

    join.add(async move { panic!("Panic.") });

    let task = join.next().await;

    fail::when!(task.is_none());
    fail::when!(task.result.is_ok(), "Did not panic.");
  });

  test!(cx, "continues on panic", {
    let (tx, rx) = channel::unbounded::<()>();
    let mut join = task::Join::new();

    join.add(async move {
      let _guard = tx;
      panic!("Panic.")
    });

    join.add(async move {
      rx.recv().await.ok();
    });

    let mut count = 0;

    while join.next().await.is_some() {
      count += 1;
    }

    fail::when!(count != 2);
  });

  test!(cx, "::drain()", timeout = "1 s", {
    let (tx, rx) = channel::unbounded::<()>();
    let mut join = task::Join::new();

    join.add({
      let tx = tx.clone();

      async move {
        let _guard = tx;
        task::sleep(Duration::hz(60)).await;
      }
    });

    join.add(async move {
      let _guard = tx;
      task::sleep(Duration::hz(60)).await;
      panic!("nah")
    });

    join.drain().await;

    let result = future::try_resolve(rx.recv());

    fail::when!(result.is_none());
    fail::when!(result.is_ok());
  });
}

/// Tests the `TryJoin` struct.
fn test_try_join(cx: &mut test::Context) {
  test!(cx, "reports panics", {
    let mut join = task::TryJoin::<(), fail::Error>::new();

    join.add(async move { panic!("oh no") });

    let task = join.next().await;

    fail::when!(task.is_none());

    let result = task.result;

    fail::when!(result.is_ok(), "Did not panic.");
    fail::when!(!result.to_string().contains("oh no"), "Did not convert panic properly.");
  });

  test!(cx, "reports errors", {
    let mut join = task::TryJoin::<(), fail::Error>::new();

    join.add(async move { fail!("Screwed up.") });

    let task = join.next().await;

    fail::when!(task.is_none());

    let result = task.result;

    fail::when!(result.is_ok(), "Did not fail.");
    fail::when!(result.to_string().as_str() != "Screwed up.", "Did not convert error properly.");
  });

  test!(cx, "::drain()", timeout = "1 s", {
    let (tx, rx) = channel::unbounded::<()>();
    let mut join = task::TryJoin::new();

    join.add({
      let tx = tx.clone();

      async move {
        let _guard = tx;
        task::sleep(Duration::hz(60)).await;
        Ok(())
      }
    });

    join.add({
      let tx = tx.clone();

      async move {
        let _guard = tx;
        task::sleep(Duration::hz(60)).await;
        panic!("nah")
      }
    });

    join.add(async move {
      let _guard = tx;
      task::sleep(Duration::hz(60)).await;
      fail!("nah")
    });

    join.drain().await;

    let result = future::try_resolve(rx.recv());

    fail::when!(result.is_none());
    fail::when!(result.is_ok());
  });

  cx.scope("::try_next()", |cx| {
    test!(cx, "returns finished results", {
      let mut join = task::TryJoin::<bool, fail::Error>::new();

      join.add(async move { Ok(true) });

      let task = join.try_next().await.transpose()?;

      fail::when!(task.is_none());
      fail::when!(!task.output);
    });

    test!(cx, "reports panicked tasks as errors", {
      let mut join = task::TryJoin::<(), fail::Error>::new();

      join.add(async move { panic!("testing") });

      let task = join.try_next().await;

      fail::when!(task.is_none());
      fail::when!(task.is_ok());
      fail::when!(!task.to_string().contains("testing"), "Did not display panic message.");
    });

    test!(cx, "reports errored tasks as errors", {
      let mut join = task::TryJoin::<(), fail::Error>::new();

      join.add(async move { fail!("nope") });

      let task = join.try_next().await;

      fail::when!(task.is_none());
      fail::when!(task.is_ok());
      fail::when!(!task.to_string().contains("nope"), "Did not convert error properly.",);
    });
  });

  test!(cx, "::try_drain()", timeout = "1 s", {
    let (tx, rx) = channel::unbounded::<()>();
    let mut join = task::TryJoin::new();

    join.add({
      let tx = tx.clone();

      async move {
        let _guard = tx;
        Ok(())
      }
    });

    join.add({
      let tx = tx.clone();

      async move {
        let _guard = tx;
        task::sleep(Duration::hz(60)).await;
        fail!("what")
      }
    });

    join.add(async move {
      let _guard = tx;
      task::sleep(Duration::hz(60)).await;
      panic!("nah")
    });

    let result = join.try_drain().await;

    fail::when!(result.is_ok());

    let result = join.try_drain().await;

    fail::when!(result.is_ok());

    let result = future::try_resolve(rx.recv());

    fail::when!(result.is_none());
    fail::when!(result.is_ok());
  });
}
