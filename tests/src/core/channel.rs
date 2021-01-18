// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use af_core::test::prelude::*;
use af_core::{channel, task};

/// Test the `channel` module.
pub fn test(cx: &mut test::Context) {
  cx.scope("::unbounded", test_unbounded);
  cx.scope("::with_capacity", test_with_capacity);
}

/// Test unbounded channels.
fn test_unbounded(cx: &mut test::Context) {
  test!(cx, "can transmit messages synchronously", {
    let (tx, rx) = channel::unbounded();

    tx.try_send(1)?;
    tx.try_send(2)?;

    let one = rx.try_recv()?;
    let two = rx.try_recv()?;

    fail::when!(one.is_none());
    fail::when!(two.is_none());
    fail::when!(one != 1);
    fail::when!(two != 2);
  });

  test!(cx, "can transmit messages asynchronously", {
    let (tx, rx) = channel::unbounded();

    tx.send(1usize).await?;
    tx.send(2).await?;

    let one = rx.recv().await?;
    let two = rx.recv().await?;

    fail::when!(one != 1);
    fail::when!(two != 2);
  });

  test!(cx, "is_closed() returns `true` when closed", {
    let (tx, rx) = channel::unbounded::<()>();

    fail::when!(tx.is_closed(), "Closed immediately.");
    drop(rx);
    fail::when!(!tx.is_closed(), "Dropping the Receiver did not close the channel.");

    let (tx, rx) = channel::unbounded::<()>();

    fail::when!(rx.is_closed(), "Closed immediately.");
    drop(tx);
    fail::when!(!rx.is_closed(), "Dropping the Sender did not close the channel.");
  });

  cx.scope("try_recv()", |cx| {
    test!(cx, "succeeds while open", {
      let (_tx, rx) = channel::unbounded::<()>();

      rx.try_recv()?;
      rx.try_recv()?;
    });

    test!(cx, "fails when closed", {
      let (_, rx) = channel::unbounded::<()>();
      let result = rx.try_recv();

      fail::when!(result.is_ok());
    });

    test!(cx, "returns `None` while empty", {
      let (_tx, rx) = channel::unbounded::<()>();
      let msg = rx.try_recv()?;

      fail::when!(msg.is_some());
    });
  });

  cx.scope("recv()", |cx| {
    test!(cx, "succeeds immediately while open and non-empty", timeout = immediate, {
      let (tx, rx) = channel::unbounded();
      let _ = tx.try_send(());

      rx.recv().await?;
    });

    test!(cx, "fails immediately when closed", timeout = immediate, {
      let (_, rx) = channel::unbounded::<()>();
      let result = rx.recv().await;

      fail::when!(result.is_ok());
    });

    test!(cx, "waits when empty", timeout = "1 s", {
      let (tx, rx) = channel::unbounded::<()>();

      let send = task::start(async move {
        task::sleep(Duration::hz(60)).await;

        tx.try_send(())
      });

      let recv = rx.recv();

      pin!(recv);

      fail::when!(future::poll(&mut recv).is_some(), "Completed immediately.");

      send.await?;
      recv.await?;
    });
  });

  cx.scope("try_send()", |cx| {
    test!(cx, "succeeds while open", {
      let (tx, _rx) = channel::unbounded();

      tx.try_send(())?;
      tx.try_send(())?;
    });

    test!(cx, "fails while closed", {
      let (tx, _) = channel::unbounded();

      match tx.try_send(1) {
        Ok(()) => fail!("Succeeded."),
        Err(err) => match err.reason {
          channel::SendErrorReason::Closed => {}
          other => fail!("Unexpected error reason: `{:?}`", other),
        },
      }
    });
  });

  cx.scope("send()", |cx| {
    test!(cx, "succeeds immediately while open", timeout = immediate, {
      let (tx, _rx) = channel::unbounded();

      tx.send(()).await?;
      tx.send(()).await?;
    });

    test!(cx, "fails immediately while closed", timeout = immediate, {
      let (tx, _) = channel::unbounded();

      match tx.send(()).await {
        Ok(()) => fail!("Succeeded."),
        Err(err) => match err.reason {
          channel::SendErrorReason::Closed => {}
          other => fail!("Unexpected error reason: `{:?}`", other),
        },
      }
    });
  });
}

/// Test bounded channels.
fn test_with_capacity(cx: &mut test::Context) {
  test!(cx, "can transmit messages synchronously", {
    let (tx, rx) = channel::with_capacity(2);

    tx.try_send(1)?;
    tx.try_send(2)?;

    let one = rx.try_recv()?;
    let two = rx.try_recv()?;

    fail::when!(one.is_none());
    fail::when!(two.is_none());

    fail::when!(one != 1);
    fail::when!(two != 2);
  });

  test!(cx, "can transmit messages asynchronously", {
    let (tx, rx) = channel::with_capacity(2);

    tx.send(1usize).await?;
    tx.send(2).await?;

    let one = rx.recv().await?;
    let two = rx.recv().await?;

    fail::when!(one != 1);
    fail::when!(two != 2);
  });

  test!(cx, "is_closed() returns `true` when closed", {
    let (tx, rx) = channel::with_capacity::<()>(10);

    fail::when!(tx.is_closed(), "Closed immediately.");
    drop(rx);
    fail::when!(!tx.is_closed(), "Dropping the Receiver did not close the channel.");

    let (tx, rx) = channel::with_capacity::<()>(10);

    fail::when!(rx.is_closed(), "Closed immediately.");
    drop(tx);
    fail::when!(!rx.is_closed(), "Dropping the Sender did not close the channel.");
  });

  cx.scope("try_recv()", |cx| {
    test!(cx, "succeeds while open", {
      let (_tx, rx) = channel::with_capacity::<()>(2);

      rx.try_recv()?;
      rx.try_recv()?;
    });

    test!(cx, "fails when closed", {
      let (_, rx) = channel::with_capacity::<()>(2);
      let result = rx.try_recv();

      fail::when!(result.is_ok());
    });

    test!(cx, "returns `None` while empty", {
      let (_tx, rx) = channel::with_capacity::<()>(2);
      let msg = rx.try_recv()?;

      fail::when!(msg.is_some());
    });
  });

  cx.scope("recv()", |cx| {
    test!(cx, "succeeds immediately while open and non-empty", timeout = immediate, {
      let (tx, rx) = channel::with_capacity(10);
      let _ = tx.try_send(());

      rx.recv().await?;
    });

    test!(cx, "fails immediately when closed", timeout = immediate, {
      let (tx, rx) = channel::with_capacity::<()>(8);

      drop(tx);

      let result = rx.recv().await;

      fail::when!(result.is_ok());
    });

    test!(cx, "waits when empty", timeout = "1 s", {
      let (tx, rx) = channel::with_capacity::<()>(8);

      let send = task::start(async move {
        task::sleep(Duration::hz(60)).await;

        tx.try_send(())
      });

      let recv = rx.recv();

      pin!(recv);

      fail::when!(future::poll(&mut recv).is_some(), "Completed immediately.");

      send.await?;
      recv.await?;
    });
  });

  cx.scope("try_send()", |cx| {
    test!(cx, "succeeds while not full", {
      let (tx, _rx) = channel::with_capacity(2);

      tx.try_send(())?;
      tx.try_send(())?;
    });

    test!(cx, "fails while closed", {
      let (tx, _) = channel::with_capacity(13);

      match tx.try_send(()) {
        Ok(()) => fail!("Succeeded."),
        Err(err) => match err.reason {
          channel::SendErrorReason::Closed => {}
          other => fail!("Unexpected error reason: `{:?}`", other),
        },
      }
    });

    test!(cx, "fails while full", {
      let (tx, _rx) = channel::with_capacity(1);

      tx.try_send(())?;

      match tx.try_send(()) {
        Ok(()) => fail!("Succeeded."),
        Err(err) => match err.reason {
          channel::SendErrorReason::Full => {}
          other => fail!("Unexpected error reason: `{:?}`", other),
        },
      }
    });
  });

  cx.scope("send()", |cx| {
    test!(cx, "succeeds immediately while not full", timeout = immediate, {
      let (tx, _rx) = channel::with_capacity(2);

      tx.send(()).await?;
      tx.send(()).await?;
    });

    test!(cx, "fails when closed", timeout = immediate, {
      let (tx, _) = channel::with_capacity(8);

      match tx.send(()).await {
        Ok(()) => fail!("Succeeded."),
        Err(err) => match err.reason {
          channel::SendErrorReason::Closed => {}
          other => fail!("Unexpected error reason: `{:?}`", other),
        },
      }
    });

    test!(cx, "waits while full", timeout = "1 s", {
      let (tx, rx) = channel::with_capacity(1);

      let recv = task::start(async move {
        task::sleep(Duration::hz(60)).await;

        rx.recv().await?;
        rx.recv().await
      });

      tx.send(()).await?;

      let send = tx.send(());

      pin!(send);

      fail::when!(future::poll(&mut send).is_some(), "Sent immediately.");

      send.await?;
      recv.await?;
    });
  });
}
