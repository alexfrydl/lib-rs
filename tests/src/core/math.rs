// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use af_core::math;
use af_core::test::prelude::*;

/// Test the `math` module.
pub fn test(cx: &mut test::Context) {
  test!(cx, "::clamp()", {
    let a = math::clamp(10, 1, 20);
    let b = math::clamp(-0.5, 1.0, 10.0);
    let c = math::clamp(20, -7, 8);

    fail::when!(a != 10);
    fail::when!(b != 1.0);
    fail::when!(c != 8);
  });

  test!(cx, "::clamp_mut()", {
    let mut x = 10;

    math::clamp_mut(&mut x, 0, 20);
    fail::when!(x != 10);
    math::clamp_mut(&mut x, -20, -10);
    fail::when!(x != -10);
    math::clamp_mut(&mut x, 5, 14);
    fail::when!(x != 5);
  });

  cx.scope("::FloatExt", |cx| {
    cx.scope("<f64>", |cx| {
      test!(cx, "::ceil_to_places()", {
        const VALUE: f64 = 1.35728419;

        check(0, 2.0)?;
        check(1, 1.4)?;
        check(2, 1.36)?;
        check(3, 1.358)?;
        check(4, 1.3573)?;
        check(5, 1.35729)?;
        check(6, 1.357285)?;
        check(7, 1.3572842)?;
        check(8, 1.35728419)?;

        fn check(i: usize, expected: f64) -> Result {
          let actual = VALUE.ceil_to_places(i);

          fail::when!(
            expected != actual,
            "Expected `{}f64.ceil_to_places({})` to be `{}`, was `{}`.",
            VALUE,
            i,
            expected,
            actual
          );

          Ok(())
        }
      });

      test!(cx, "::round_to_places()", {
        const VALUE: f64 = 1.35728419;

        check(0, 1.0)?;
        check(1, 1.4)?;
        check(2, 1.36)?;
        check(3, 1.357)?;
        check(4, 1.3573)?;
        check(5, 1.35728)?;
        check(6, 1.357284)?;
        check(7, 1.3572842)?;
        check(8, 1.35728419)?;

        fn check(i: usize, expected: f64) -> Result {
          let actual = VALUE.round_to_places(i);

          fail::when!(
            expected != actual,
            "Expected `{}f64.round_to_places({})` to be `{}`, was `{}`.",
            VALUE,
            i,
            expected,
            actual
          );

          Ok(())
        }
      });

      test!(cx, "::floor_to_places()", {
        const VALUE: f64 = 1.35728419;

        check(0, 1.0)?;
        check(1, 1.3)?;
        check(2, 1.35)?;
        check(3, 1.357)?;
        check(4, 1.3572)?;
        check(5, 1.35728)?;
        check(6, 1.357284)?;
        check(7, 1.3572841)?;
        check(8, 1.35728419)?;

        fn check(i: usize, expected: f64) -> Result {
          let actual = VALUE.floor_to_places(i);

          fail::when!(
            expected != actual,
            "Expected `{}f64.floor_to_places({})` to be `{}`, was `{}`.",
            VALUE,
            i,
            expected,
            actual
          );

          Ok(())
        }
      });
    });

    cx.scope("<f32>", |cx| {
      test!(cx, "::ceil_to_places()", {
        const VALUE: f32 = 1.3572;

        check(0, 2.0)?;
        check(1, 1.4)?;
        check(2, 1.36)?;
        check(3, 1.358)?;
        check(4, 1.3572)?;

        fn check(i: usize, expected: f32) -> Result {
          let actual = VALUE.ceil_to_places(i);

          fail::when!(
            expected != actual,
            "Expected `{}f32.ceil_to_places({})` to be `{}`, was `{}`.",
            VALUE,
            i,
            expected,
            actual
          );

          Ok(())
        }
      });

      test!(cx, "::round_to_places()", {
        const VALUE: f32 = 1.3572;

        check(0, 1.0)?;
        check(1, 1.4)?;
        check(2, 1.36)?;
        check(3, 1.357)?;
        check(4, 1.3572)?;

        fn check(i: usize, expected: f32) -> Result {
          let actual = VALUE.round_to_places(i);

          fail::when!(
            expected != actual,
            "Expected `{}f32.round_to_places({})` to be `{}`, was `{}`.",
            VALUE,
            i,
            expected,
            actual
          );

          Ok(())
        }
      });

      test!(cx, "::floor_to_places()", {
        const VALUE: f32 = 1.3572;

        check(0, 1.0)?;
        check(1, 1.3)?;
        check(2, 1.35)?;
        check(3, 1.357)?;
        check(4, 1.3572)?;

        fn check(i: usize, expected: f32) -> Result {
          let actual = VALUE.floor_to_places(i);

          fail::when!(
            expected != actual,
            "Expected `{}f32.floor_to_places({})` to be `{}`, was `{}`.",
            VALUE,
            i,
            expected,
            actual
          );

          Ok(())
        }
      });
    });
  });
}
