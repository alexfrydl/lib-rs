// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use af_core::test::prelude::*;
use af_core::util::defer;

/// Test the `util` module.
pub fn test(cx: &mut test::Context) {
  test!(cx, "::defer()", {
    let cell = Cell::new(false);

    {
      let _guard = defer(|| cell.set(true));

      fail::when!(cell.get(), "Ran immediately.");
    }

    fail::when!(!cell.get(), "Did not run.");
  });
}
