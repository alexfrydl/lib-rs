// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use af_core::channel;
use af_core::test::prelude::*;

pub fn test(cx: &mut test::Context) {
  // Unbounded channels.

  cx.scope("::unbounded", |cx| {
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

    test!(cx, "is closed when all receivers are dropped", {
      let (tx, rx) = channel::unbounded::<()>();

      fail::when!(tx.is_closed(), "Channel closed immediately.");
      drop(rx);
      fail::when!(!tx.is_closed(), "Channel did not close.");
    });

    test!(cx, "is closed when all senders are dropped", {
      let (tx, rx) = channel::unbounded::<()>();

      fail::when!(rx.is_closed(), "Channel closed immediately.");
      drop(tx);
      fail::when!(!rx.is_closed(), "Channel did not close.");
    });
  });

  // Bounded channels.

  cx.scope("::with_capacity", |cx| {
    test!(cx, "can transmit messages synchronously", {
      let (tx, rx) = channel::with_capacity(2);

      tx.try_send(1)?;

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

    test!(cx, "is closed when all receivers are dropped", {
      let (tx, rx) = channel::with_capacity::<()>(8);

      fail::when!(tx.is_closed(), "Channel closed immediately.");
      drop(rx);
      fail::when!(!tx.is_closed(), "Channel did not close.");
    });

    test!(cx, "is closed when all senders are dropped", {
      let (tx, rx) = channel::with_capacity::<()>(8);

      fail::when!(rx.is_closed(), "Channel closed immediately.");
      drop(tx);
      fail::when!(!rx.is_closed(), "Channel did not close.");
    });
  });
}
