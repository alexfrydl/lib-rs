// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod channel;
mod env;
mod fmt;
mod future;
mod math;
mod path;
mod random;
mod util;

use af_core::test::prelude::*;

/// Tests the `af_core` package.
pub fn test(cx: &mut test::Context) {
  cx.scope("::channel", channel::test);
  cx.scope("::env", env::test);
  cx.scope("::fmt", fmt::test);
  cx.scope("::future", future::test);
  cx.scope("::math", math::test);
  cx.scope("::path", path::test);
  cx.scope("::random", random::test);
  cx.scope("::util", util::test);
}
