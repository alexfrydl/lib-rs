// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use af_lib::test::prelude::*;

/// Main entry point for tests.
#[test::main]
async fn main(t: test::Context) {
  t.test("root level test", async {
    assert!(false);
  });

  test_things(&t);
}

fn test_things(t: &test::Context) {
  let t = t.context("Thing");

  t.test("should fail", async {
    assert!(false);
  });

  t.test("::tester()", async {
    assert_eq!("not", "equal");
  });
}
