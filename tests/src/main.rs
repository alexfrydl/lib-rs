// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod core;

use af_core::prelude::*;
use af_core::test;

#[af_core::test::main]
fn test(cx: &mut test::Context) {
  cx.context("af_core", core::test);
}
