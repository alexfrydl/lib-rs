mod channel;

use af_core::prelude::*;
use af_core::test;

pub fn test(cx: &mut test::Context) {
  cx.context("::channel", channel::test);
}
