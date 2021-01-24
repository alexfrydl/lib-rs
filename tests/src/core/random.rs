// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use af_core::task;
use af_core::test::prelude::*;

/// Test the `random` module.
pub fn test(cx: &mut test::Context) {
  test!(cx, "::chance()", {
    let chance = sample_chance(|| random::chance(0.42)).await;

    fail::when!(
      chance < 0.40 || chance > 0.44,
      "Expected a chance of 42%, was {}%.",
      (chance * 100.0).round_to_places(2),
    );
  });

  test!(cx, "::fill_bytes()", {
    let empty = vec![0u8; 256];

    let mut a = empty.clone();
    let mut b = empty.clone();

    random::fill_bytes(&mut a);
    random::fill_bytes(&mut b);

    fail::when!(a == b);
    fail::when!(a == empty);
    fail::when!(b == empty);
  });

  test!(cx, "::range()", {
    let (min, max) = sample_range(|| random::range(1..=10)).await;

    fail::when!(min != 1);
    fail::when!(max != 10);
  });

  test!(cx, "::ratio()", {
    let chance = sample_chance(|| random::ratio(7, 8)).await;

    fail::when!(
      chance < 0.85 || chance > 0.9,
      "Expected a chance of 87.5%, was {}%.",
      (chance * 100.0).round_to_places(2),
    );
  });

  test!(cx, "::shuffle()", {
    let initial = (0..=255u8).collect_vec();

    let mut a = initial.clone();
    let mut b = initial.clone();

    random::shuffle(&mut a);
    random::shuffle(&mut b);

    fail::when!(a == b);
    fail::when!(a == initial);
    fail::when!(b == initial);

    a.sort_unstable();
    b.sort_unstable();

    fail::when!(a != initial, "`a` has different elements.");
    fail::when!(b != initial, "`b` has different elements.");
  });

  cx.scope("::Rng", |cx| {
    test!(cx, "::fill_bytes()", {
      let empty = vec![0u8; 256];

      let mut a = empty.clone();
      let mut b = empty.clone();

      let mut rng = random::Rng::new();

      rng.fill_bytes(&mut a);
      rng.fill_bytes(&mut b);

      fail::when!(a == b);
      fail::when!(a == empty);
      fail::when!(b == empty);
    });

    test!(cx, "::gen_chance()", {
      let mut rng = random::Rng::new();
      let chance = sample_chance(|| rng.gen_chance(0.71)).await;

      fail::when!(
        chance < 0.69 || chance > 0.73,
        "Expected a chance of 71%, was {}%.",
        (chance * 100.0).round_to_places(2),
      );
    });

    test!(cx, "::gen_range()", {
      let mut rng = random::Rng::new();
      let (min, max) = sample_range(|| rng.gen_range(0..8)).await;

      fail::when!(min != 0);
      fail::when!(max != 7);
    });

    test!(cx, "::gen_ratio()", {
      let mut rng = random::Rng::new();
      let chance = sample_chance(|| rng.gen_ratio(4, 9)).await;

      fail::when!(
        chance < 0.434 || chance > 0.464,
        "Expected a chance of 87.5%, was {}%.",
        (chance * 100.0).round_to_places(2),
      );
    });

    test!(cx, "::shuffle()", {
      let initial = (0..=255u8).collect_vec();

      let mut a = initial.clone();
      let mut b = initial.clone();

      let mut rng = random::Rng::new();

      rng.shuffle(&mut a);
      rng.shuffle(&mut b);

      fail::when!(a == b);
      fail::when!(a == initial);
      fail::when!(b == initial);

      a.sort_unstable();
      b.sort_unstable();

      fail::when!(a != initial, "`a` has different elements.");
      fail::when!(b != initial, "`b` has different elements.");
    });
  });
}

/// Samples the chance of a function returning `true`.
async fn sample_chance(mut func: impl FnMut() -> bool) -> f64 {
  let mut count = 0;

  for _ in 0..128 {
    for _ in 0..128 {
      if func() {
        count += 1;
      }
    }

    task::yield_now().await;
  }

  count as f64 / 16_384.0
}

/// Samples the minimum and maximum of a range.
async fn sample_range<T>(mut func: impl FnMut() -> T) -> (T, T)
where
  T: Copy + Number + Ord,
{
  let mut min = func();
  let mut max = func();

  if min > max {
    mem::swap(&mut min, &mut max);
  }

  for _ in 0..128 {
    for _ in 0..128 {
      let val = func();

      min = cmp::min(min, val);
      max = cmp::max(max, val);
    }

    task::yield_now().await;
  }

  (min, max)
}
