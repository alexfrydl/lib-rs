// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use af_core::string::SharedString;
use af_core::test::prelude::*;

/// Test the `stringü` module.
pub fn test(cx: &mut test::Context) {
  cx.scope("::SharedString", |cx| {
    test!(cx, "functions as a string", {
      let value = "hello world";
      let shared = SharedString::from(value);

      fail::when!(shared.as_str() != value, "`as_str()` is not {:?}.", value);
      fail::when!(shared.to_string() != value, "`to_string()` is not {:?}", value);
      fail::when!(AsRef::<str>::as_ref(&shared) != value, "`as_ref()` is not {:?}.", value);
    });

    test!(cx, "::from<&'static str>() is cheap", {
      let value = "check";
      let shared = SharedString::from(value);

      fail::when!(!ptr::eq(shared.as_str(), value), "Value was copied.");
    });

    test!(cx, "::clone() is cheap", {
      let shared = SharedString::from("hello world");
      let clone = shared.clone();

      fail::when!(!ptr::eq(clone.as_str(), shared.as_str()), "First clone is different.");

      let shared = SharedString::from(String::from("hello world"));
      let clone = shared.clone();

      fail::when!(!ptr::eq(clone.as_str(), shared.as_str()), "Second clone is different.");
    });
  });
}
