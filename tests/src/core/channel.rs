use af_core::channel;
use af_core::prelude::*;
use af_core::test;

pub fn test(cx: &mut test::Context) {
  cx.context("::Receiver", |cx| {
    cx.test("::is_closed() returns true if all Senders are dropped", async {
      let (tx, rx) = channel::unbounded::<()>();

      fail::when!(rx.is_closed(), "Channel closed immediately.");
      drop(tx);
      fail::when!(!rx.is_closed(), "Channel did not close.");

      Ok(())
    });
  });

  cx.context("::Sender", |cx| {
    cx.test("::is_closed() returns true if all Receivers are dropped", async {
      let (tx, rx) = channel::unbounded::<()>();

      fail::when!(tx.is_closed(), "Channel closed immediately.");
      drop(rx);
      fail::when!(!tx.is_closed(), "Channel did not close.");

      Ok(())
    });
  });
}
