// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use af_core::test::prelude::*;
use af_core::{env, path};

/// Test the `channel` module.
pub fn test(cx: &mut test::Context) {
  test!(cx, "::project_path()", {
    let path = env::project_path().unwrap();

    fail::when!(!path::is_absolute(path), "Path is not absolute.");
    fail::when!(path != env::var("CARGO_MANIFEST_DIR")?, "Path does not point to manifest dir.");
  });

  test!(cx, "::workspace_path()", {
    let path = env::workspace_path().unwrap();

    fail::when!(!path::is_absolute(path), "Path is not absolute.");

    fail::when!(
      path != path::parent(env::project_path().unwrap()).unwrap(),
      "Path does not point to workspace."
    );
  });

  test!(cx, "::var()", {
    std::env::remove_var("__TEST_VAR");

    fail::when!(env::var("__TEST_VAR").is_ok(), "Read non-existent var.");

    std::env::set_var("__TEST_VAR", "value");

    fail::when!(env::var("__TEST_VAR").unwrap() != "value", "Value was incorrect.");
  });

  test!(cx, "::working_path()", {
    let path = env::working_path().unwrap();

    fail::when!(!path::is_absolute(&path), "Path is not absolute.");

    fail::when!(
      !path::starts_with(&path, env::workspace_path().unwrap()),
      "Path is outside the workspace directory."
    );
  });
}
