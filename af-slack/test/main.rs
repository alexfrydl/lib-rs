// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use af_core::test::prelude::*;
use af_slack::{api, chat};
use structopt::*;

#[derive(StructOpt)]
struct Options {
  /// Sets the OAuth token to use.
  #[structopt(long, env = "SLACK_TOKEN")]
  token: String,
}

#[test::main]
fn main(cx: &mut test::Context) {
  let options = Options::from_args();

  test!(cx, "can post a message", {
    let client = api::Client::new(&options.token);

    let id = chat::post(
      &client,
      "#detail",
      chat::Attachment::warn()
        .with_block(":warning: *A warning attachment*")
        .with_block("More info."),
    )
    .await?;

    let id = chat::reply(&client, &id, ":exclamation: Test message.").await?;
    let permalink = chat::permalink_to(&client, &id).await?;

    chat::post(
      &client,
      "#general",
      chat::Attachment::warn()
        .with_block(format!(":warning: There's <{}|a warning> in the other channel.", permalink)),
    )
    .await?;
  });
}
