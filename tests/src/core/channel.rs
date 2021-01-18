// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use af_core::channel;
use af_core::test::prelude::*;

pub fn test(cx: &mut test::Context) {
  context!(::Receiver, {
    context!(::is_closed, {
      test!("returns true if all Senders are dropped", {
        let (tx, rx) = channel::unbounded::<()>();

        fail::when!(rx.is_closed(), "Channel closed immediately.");
        drop(tx);
        fail::when!(!rx.is_closed(), "Channel did not close.");
      });
    });
  });

  context!(::Sender, {
    context!(::is_closed, {
      test!("returns true if all Receivers are dropped", {
        let (tx, rx) = channel::unbounded::<()>();

        fail::when!(tx.is_closed(), "Channel closed immediately.");
        drop(rx);
        fail::when!(!tx.is_closed(), "Channel did not close.");
      });
    });
  });
}
