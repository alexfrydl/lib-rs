// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

#![allow(clippy::float_cmp)]

#[cfg(feature = "core")]
mod core;
#[cfg(feature = "postgres")]
mod postgres;
#[cfg(feature = "sentry")]
mod sentry;

use af_core::test::prelude::*;
use structopt::*;

#[derive(StructOpt)]
pub struct Options {
  #[cfg(feature = "sentry")]
  /// The Sentry DSN to use.
  #[structopt(long, env = "SENTRY_DSN")]
  dsn: String,

  #[cfg(feature = "postgres")]
  /// The Postgres URL to connect to.
  #[structopt(long, env = "POSTGRES_URL")]
  postgres_url: af_postgres::Config,
}

#[af_core::main]
async fn main() {
  #[allow(unused_variables)]
  let options = Options::from_args();

  let result = {
    #[cfg(feature = "sentry")]
    let _guard = af_sentry::init(options.dsn.as_str());

    af_core::test::runner::run(|cx| test(cx, options)).await
  };

  if result.is_err() {
    std::process::exit(1);
  }
}

/// Entry point of the test suite.
fn test(cx: &mut test::Context, options: Options) {
  #[cfg(feature = "core")]
  cx.scope("af_core", core::test);

  #[cfg(feature = "postgres")]
  cx.scope("postgres", |cx| postgres::test(cx, &options));

  #[cfg(feature = "sentry")]
  cx.scope("sentry", sentry::test);
}
