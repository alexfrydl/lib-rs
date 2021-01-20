// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#[cfg(feature = "core")]
mod core;

#[cfg(feature = "sentry")]
mod sentry;

use af_core::test::prelude::*;
use structopt::*;

#[derive(StructOpt)]
pub struct Options {
  /// The Sentry DSN to use for tests.
  #[cfg(feature = "sentry")]
  #[structopt(long, env = "SENTRY_DSN")]
  dsn: String,
}

#[af_core::main]
async fn main() {
  #[allow(unused_variables)]
  let options = Options::from_args();

  let result = {
    #[cfg(feature = "sentry")]
    let _guard = af_sentry::init(options.dsn);

    af_core::test::runner::run(test).await
  };

  if result.is_err() {
    std::process::exit(1);
  }
}

/// Entry point of the test suite.
fn test(cx: &mut test::Context) {
  #[cfg(feature = "core")]
  cx.scope("af_core", core::test);

  #[cfg(feature = "sentry")]
  cx.scope("sentry", sentry::test);
}
