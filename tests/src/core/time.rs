// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use af_core::task;
use af_core::test::prelude::*;

/// Test the `time` module.
pub fn test(cx: &mut test::Context) {
  cx.scope("::Date", test_date);
  cx.scope("::Duration", test_duration);
  cx.scope("::Time", test_time);
  cx.scope("::Zone", test_zone);
}

/// Tests `Date`.
fn test_date(cx: &mut test::Context) {
  test!(cx, "::next()", {
    let date = Date::from_ymd(2021, 1, 24);

    let next = date.next();
    let (year, month, day) = next.ymd();

    fail::when!(year != 2021);
    fail::when!(month != 1);
    fail::when!(day != 25);
  });

  test!(cx, "::prev()", {
    let date = Date::from_ymd(2021, 2, 1);

    let next = date.prev();
    let (year, month, day) = next.ymd();

    fail::when!(year != 2021);
    fail::when!(month != 1);
    fail::when!(day != 31);
  });

  test!(cx, "::to_time()", {
    let initial = Date::from_ymd(2021, 1, 24);

    let time = initial.to_time(time::UTC);
    let date = time.date();
    let (hour, minute, second) = time.hms();

    fail::when!(date != initial);
    fail::when!(hour != 0);
    fail::when!(minute != 0);
    fail::when!(second != 0);
  });
}

/// Tests `Duration`.
fn test_duration(cx: &mut test::Context) {
  test!(cx, "cannot be negative", {
    let secs = Duration::secs(-1.0).as_secs();

    fail::when!(secs != 0.0);
  });

  test!(cx, "::as_ms()", repeat = 100, {
    let expected = random::range(500..1_500) as f64;
    let actual = Duration::secs(expected / 1_000.0).as_ms().round_to_places(2);

    fail::when!(actual != expected);
  });

  test!(cx, "::as_hz()", repeat = 100, {
    let expected = random::range(1..10_000) as f64 / 10.0;
    let actual = Duration::secs(1.0 / expected).as_hz().round_to_places(2);

    fail::when!(actual != expected);
  });

  test!(cx, "::as_secs()", repeat = 100, {
    let expected = random::range(500_000..1_500_000) as f64;
    let actual = Duration::secs(expected).as_secs().round_to_places(1);

    fail::when!(actual != expected);
  });

  test!(cx, "::as_mins()", repeat = 100, {
    let expected = random::range(500..1500) as f64;
    let actual = Duration::secs(expected * 60.0).as_mins().round_to_places(1);

    fail::when!(actual != expected);
  });

  test!(cx, "::as_hours()", repeat = 100, {
    let expected = random::range(500..1500) as f64;
    let actual = Duration::secs(expected * 60.0 * 60.0).as_hours().round_to_places(1);

    fail::when!(actual != expected)
  });

  test!(cx, "::as_days()", repeat = 100, {
    let expected = random::range(500..1500) as f64;
    let actual = Duration::secs(expected * 60.0 * 60.0 * 24.0).as_days().round_to_places(1);

    fail::when!(actual != expected)
  });

  test!(cx, "::as_weeks()", repeat = 100, {
    let expected = random::range(500..1500) as f64;
    let actual = Duration::secs(expected * 7.0 * 24.0 * 60.0 * 60.0).as_weeks().round_to_places(1);

    fail::when!(actual != expected)
  });

  test!(cx, "::ms()", repeat = 100, {
    let expected = random::range(500..1500) as f64;
    let actual = Duration::ms(expected * 1_000.0).as_secs().round_to_places(2);

    fail::when!(actual != expected);
  });

  test!(cx, "::hz()", repeat = 100, {
    let expected = random::range(1..100) as f64 / 10.0;
    let actual = Duration::hz(1.0 / expected).as_secs().round_to_places(2);

    fail::when!(actual != expected);
  });

  test!(cx, "::secs()", repeat = 100, {
    let expected = random::range(50_000_000..150_000_000) as f64;
    let actual = Duration::secs(expected).as_secs().round_to_places(1);

    fail::when!(actual != expected);
  });

  test!(cx, "::mins()", repeat = 100, {
    let expected = random::range(50_000_000..150_000_000) as f64;
    let actual = Duration::mins(expected / 60.0).as_secs().round_to_places(1);

    fail::when!(actual != expected);
  });

  test!(cx, "::hours()", repeat = 100, {
    let expected = random::range(50_000_000..150_000_000) as f64;
    let actual = Duration::hours(expected / 60.0 / 60.0).as_secs().round_to_places(1);

    fail::when!(actual != expected);
  });

  test!(cx, "::days()", repeat = 100, {
    let expected = random::range(50_000_000..150_000_000) as f64;
    let actual = Duration::days(expected / 60.0 / 60.0 / 24.0).as_secs().round_to_places(1);

    fail::when!(actual != expected);
  });

  test!(cx, "::weeks()", repeat = 100, {
    let expected = random::range(50_000_000..150_000_000) as f64;
    let actual = Duration::weeks(expected / 60.0 / 60.0 / 24.0 / 7.0).as_secs().round_to_places(1);

    fail::when!(actual != expected);
  });

  test!(cx, "::is_finite()", {
    let infinite = Duration::secs(f64::INFINITY);
    let finite = Duration::secs(u32::MAX);

    fail::when!(infinite.is_finite());
    fail::when!(!finite.is_finite());
  });

  test!(cx, "::is_infinite()", {
    let infinite = Duration::secs(f64::INFINITY);
    let finite = Duration::secs(u32::MAX);

    fail::when!(!infinite.is_infinite());
    fail::when!(finite.is_infinite());
  });

  test!(cx, "::to_std()", {
    let ms = random::<u32>() as u128;
    let duration = Duration::ms(ms);

    let std_ms = duration.to_std().as_millis();

    fail::when!(std_ms != ms);
  });

  test!(cx, " + Self", {
    let a_secs = random::range(0.1..100.0);
    let b_secs = random::range(0.1..50.0);

    let sum = Duration::secs(a_secs) + Duration::secs(b_secs);

    fail::when!(sum.as_secs() != a_secs + b_secs);
  });

  test!(cx, " - Self", {
    let a_secs = random::range(50.0..100.0);
    let b_secs = random::range(0.1..50.0);

    let diff = Duration::secs(a_secs) - Duration::secs(b_secs);

    fail::when!(diff.as_secs() != a_secs - b_secs);
  });

  test!(cx, " - Self cannot be negative", {
    let diff = Duration::secs(1.0) - Duration::secs(2.0);

    fail::when!(diff != Duration::ZERO)
  });

  test!(cx, " * f64", {
    let secs = random::range(0.1..100.0);
    let multi = random::range(0.1..10.0);

    let product = Duration::secs(secs) * multi;

    fail::when!(product.as_secs() != secs * multi);
  });

  test!(cx, " * f64 cannot be negative", {
    let neg = Duration::secs(1) * -2.0;

    fail::when!(neg != Duration::ZERO);
  });

  test!(cx, " / f64", {
    let secs = random::range(0.1..100.0);
    let div = random::range(0.1..10.0);

    let quotient = Duration::secs(secs) / div;

    fail::when!(quotient.as_secs() != secs / div);
  });

  test!(cx, " / f64 cannot be negative", {
    let neg = Duration::secs(1) / -2.0;

    fail::when!(neg != Duration::ZERO);
  });
}

/// Tests `Time`.
fn test_time(cx: &mut test::Context) {
  test!(cx, "::as_rfc3339", {
    let time = Time::from_unix_ms(1611510414260).to_utc();
    let output = time.as_rfc3339().to_string();

    fail::when!(output != "2021-01-24T17:46:54.260Z");
  });

  test!(cx, "::from_unix_ms()", {
    let time = Time::from_unix_ms(1611510414000).to_utc();
    let (year, month, day) = time.date().ymd();
    let (hour, minute, second) = time.hms();

    fail::when!(year != 2021);
    fail::when!(month != 1);
    fail::when!(day != 24);
    fail::when!(hour != 17);
    fail::when!(minute != 46);
    fail::when!(second != 54);
  });

  test!(cx, "::elapsed()", timeout = "100 ms", {
    let time = Time::now();

    task::sleep(Duration::ms(10)).await;

    let elapsed_ms = time.elapsed().as_ms();

    fail::when!(elapsed_ms < 9.0);
  });

  test!(cx, "::start_of_day()", {
    let time = Time::from_unix_ms(1611510414000).to_utc();

    let day = time.start_of_day();
    let (hour, minute, second) = day.hms();

    fail::when!(time.date() != day.date(), "Different days.");
    fail::when!(hour > 0);
    fail::when!(minute > 0);
    fail::when!(second > 0);
  });

  test!(cx, " + Duration", {
    let before = Time::from_unix_ms(1611510414000).to_utc();

    let after = before + Duration::mins(1.25);
    let (hour, minute, second) = after.hms();

    fail::when!(before.date() != after.date(), "Different days.");
    fail::when!(hour != 17);
    fail::when!(minute != 48);
    fail::when!(second != 9);
  });

  test!(cx, " - Duration", {
    let before = Time::from_unix_ms(1611510414000).to_utc();

    let after = before - Duration::hours(4.0 / 3.0);
    let (hour, minute, second) = after.hms();

    fail::when!(before.date() != after.date(), "Different days.");
    fail::when!(hour != 16);
    fail::when!(minute != 26);
    fail::when!(second != 54);
  });

  test!(cx, " - Self", {
    let before = Time::from_unix_ms(1611510414000).to_utc();
    let after = Time::from_unix_ms(1611510436873).to_utc();

    let elapsed_ms = (after - before).as_ms();
    let backwards = before - after;

    fail::when!(elapsed_ms != 22873.0);
    fail::when!(backwards > Duration::ZERO);
  });
}

/// Tests `Zone`.
fn test_zone(cx: &mut test::Context) {
  test!(cx, "::from_name()", {
    let zone = time::Zone::from_name("America/New_York")?;

    let time = Time::from_unix_ms(1611510414000).to_zone(zone);
    let (year, month, day) = time.date().ymd();
    let (hour, minute, second) = time.hms();

    fail::when!(zone.name() != "America/New_York");
    fail::when!(year != 2021);
    fail::when!(month != 1);
    fail::when!(day != 24);
    fail::when!(hour != 12);
    fail::when!(minute != 46);
    fail::when!(second != 54);
  });

  test!(cx, "::name()", {
    use time::UTC;

    fail::when!(UTC.name() != "UTC");
  });

  test!(cx, "::to_time()", {
    let initial = Date::from_ymd(2021, 1, 24);

    let time = initial.to_time(time::UTC);
    let date = time.date();
    let (hour, minute, second) = time.hms();

    fail::when!(date != initial);
    fail::when!(hour != 0);
    fail::when!(minute != 0);
    fail::when!(second != 0);
  });
}
