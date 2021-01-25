// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use af_core::test::prelude::*;
use af_sentry as sentry;
use structopt::*;

#[derive(StructOpt)]
pub struct Options {
  /// Sets the Sentry DSN to use.
  #[structopt(long, env = "SENTRY_DSN")]
  dsn: String,
}

/// Main entry point.
#[af_core::main]
async fn main() {
  #[allow(unused_variables)]
  let options = Options::from_args();

  let result = {
    let _guard = sentry::init(options.dsn.as_str());

    af_core::test::runner::run(test).await
  };

  if result.is_err() {
    std::process::exit(1);
  }
}

/// Tests the `af_sentry` package.
fn test(cx: &mut test::Context) {
  test!(cx, "is_enabled()", {
    fail::when!(!sentry::is_enabled(), "Not enabled.");
  });

  test!(cx, "simple error", {
    sentry::error!("Simple error", "A simple error with a description.");
  });

  test!(cx, "double error", {
    sentry::error!("Double error", "An error that should show up twice.");
    sentry::error!("Double error", "An error that should show up twice.")
      .with_tag("environment", "second");
  });

  test!(cx, "macro error", {
    sentry::error!("Macro error", "An error made with the {}.", "format macro");
  });

  test!(cx, "with user", {
    sentry::error!("With user", "An error containing user info.").with_user(sentry::User {
      id: Some("person".into()),
      email: Some("person@mail.com".into()),
      ..default()
    });
  });

  test!(cx, "long message", {
    sentry::error!(
      "Long message",
      "An error with a long message that should be truncated: Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Est ante in nibh mauris. In ornare quam viverra orci sagittis eu. Ornare quam viverra orci sagittis eu. Nibh ipsum consequat nisl vel pretium lectus quam id leo. Quam viverra orci sagittis eu volutpat odio facilisis mauris. Elementum tempus egestas sed sed risus pretium quam vulputate dignissim. Ut ornare lectus sit amet est placerat in. Turpis egestas pretium aenean pharetra magna ac placerat vestibulum. Vitae purus faucibus ornare suspendisse sed nisi lacus sed. Semper feugiat nibh sed pulvinar proin gravida. Vestibulum mattis ullamcorper velit sed. Nunc consequat interdum varius sit. Fermentum et sollicitudin ac orci phasellus egestas tellus rutrum tellus. Luctus venenatis lectus magna fringilla. Sit amet purus gravida quis blandit turpis cursus. Hac habitasse platea dictumst quisque sagittis. Faucibus turpis in eu mi bibendum neque egestas congue quisque. Vulputate ut pharetra sit amet aliquam id.",
    );
  });

  test!(cx, "summarized message", {
    sentry::error!(
      "Summarized message",
      r#"An error with a summary followed by a long message:

Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Est ante in nibh mauris. In ornare quam viverra orci sagittis eu. Ornare quam viverra orci sagittis eu. Nibh ipsum consequat nisl vel pretium lectus quam id leo. Quam viverra orci sagittis eu volutpat odio facilisis mauris. Elementum tempus egestas sed sed risus pretium quam vulputate dignissim. Ut ornare lectus sit amet est placerat in. Turpis egestas pretium aenean pharetra magna ac placerat vestibulum. Vitae purus faucibus ornare suspendisse sed nisi lacus sed. Semper feugiat nibh sed pulvinar proin gravida. Vestibulum mattis ullamcorper velit sed. Nunc consequat interdum varius sit. Fermentum et sollicitudin ac orci phasellus egestas tellus rutrum tellus. Luctus venenatis lectus magna fringilla. Sit amet purus gravida quis blandit turpis cursus. Hac habitasse platea dictumst quisque sagittis. Faucibus turpis in eu mi bibendum neque egestas congue quisque. Vulputate ut pharetra sit amet aliquam id.

Ipsum a arcu cursus vitae congue mauris rhoncus aenean vel. Quisque egestas diam in arcu cursus euismod quis viverra. Nisi est sit amet facilisis magna etiam. Libero nunc consequat interdum varius sit. Enim tortor at auctor urna nunc id cursus metus. Convallis convallis tellus id interdum velit laoreet id donec. Blandit massa enim nec dui nunc. In ornare quam viverra orci. Dui accumsan sit amet nulla. Tortor at risus viverra adipiscing at. Elementum nisi quis eleifend quam adipiscing vitae proin. Placerat duis ultricies lacus sed turpis.
      "#,
    );
  });

  test!(cx, "tagged error", {
    let mut error = sentry::Error::new("Tagged error");

    error.set_description("A tagged error created with the extended API.");

    error.set_tag("number", 14);
    error.set_tag("string", "hello world");

    error.report();
  });
}
