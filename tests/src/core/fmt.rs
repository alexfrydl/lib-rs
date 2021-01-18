// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use af_core::test::prelude::*;

/// Test the `fmt` module.
pub fn test(cx: &mut test::Context) {
  test!(cx, "::count()", {
    let output = format!(
      "{}, {}, {}, {}, {}",
      fmt::count(-2, "thing", "things"),
      fmt::count(-1, "thing", "things"),
      fmt::count(0, "thing", "things"),
      fmt::count(1, "thing", "things"),
      fmt::count(2, "thing", "things"),
    );

    fail::when!(
      output != "-2 things, -1 things, 0 things, 1 thing, 2 things",
      "Incorrect output: {:?}.",
      output,
    );
  });

  test!(cx, "::indent()", {
    let output = format!("{}", fmt::indent("*", "--", "One\ntwo\nthree"));

    fail::when!(output != "*One\n--two\n--three", "Incorrect output: {:?}.", output,);
  });

  test!(cx, "::surround()", {
    let output = format!("{}", fmt::surround("(", "hello", ")"));

    fail::when!(output != "(hello)", "Incorrect output: {:?}.", output,);
  });
}
