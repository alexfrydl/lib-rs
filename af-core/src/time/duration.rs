// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use parse_duration::parse::Error as ParseError;

use crate::prelude::*;
use parse_duration::parse;

/// A duration of time.
///
/// The duration is stored as a 64-bit floating point number of seconds and
/// cannot be negative.
#[derive(Clone, Copy, Default, PartialEq, PartialOrd)]
pub struct Duration {
  secs: f64,
}

impl Duration {
  /// Returns a duration of the given number of weeks.
  pub fn weeks(weeks: u64) -> Self {
    Self::secs_f64(weeks as f64 * 7.0 * 24.0 * 60.0 * 60.0)
  }

  /// Returns a duration of the given number of days.
  pub fn days(days: u64) -> Self {
    Self::secs_f64(days as f64 * 24.0 * 60.0 * 60.0)
  }

  /// Returns a duration of the given number of hours.
  pub fn hours(hours: u64) -> Self {
    Self::secs_f64(hours as f64 * 60.0 * 60.0)
  }

  /// Returns a duration of the given number of minutes.
  pub fn mins(mins: u64) -> Self {
    Self::secs_f64(mins as f64 * 60.0)
  }

  /// Returns a duration of the given number of seconds.
  pub fn secs(secs: u64) -> Self {
    Self::secs_f64(secs as f64)
  }

  /// Returns a duration of the given number of seconds.
  pub fn secs_f64(secs: f64) -> Self {
    assert!(!secs.is_nan(), "Duration cannot be NaN.");

    Self { secs: secs.max(0.0) }
  }

  /// Returns a duration of the given number of Hz.
  pub fn hz(hz: u64) -> Self {
    Self::secs_f64(1.0 / hz as f64)
  }

  /// Returns a duration of the given number of milliseconds.
  pub fn ms(ms: u64) -> Duration {
    Self::secs_f64(ms as f64 * 1000.0)
  }

  /// Return the duration as a number of days.
  pub fn as_weeks(self) -> f64 {
    self.as_secs() / 7.0 / 24.0 / 60.0 / 60.0
  }

  /// Return the duration as a number of days.
  pub fn as_days(self) -> f64 {
    self.as_secs() / 24.0 / 60.0 / 60.0
  }

  /// Return the duration as a number of hours.
  pub fn as_hours(self) -> f64 {
    self.as_secs() / 60.0 / 60.0
  }

  /// Return the duration as a number of minutes.
  pub fn as_mins(self) -> f64 {
    self.as_secs() / 60.0
  }

  /// Return the duration as a number of seconds.
  pub const fn as_secs(self) -> f64 {
    self.secs
  }

  /// Return the duration as a number of Hz.
  pub fn as_hz(self) -> f64 {
    1.0 / self.secs
  }

  /// Return the duration as a number of milliseconds.
  pub fn as_ms(self) -> f64 {
    self.secs * 1000.0
  }

  /// Converts this duration to a `std::time::Duration`.
  pub fn to_std(self) -> std::time::Duration {
    std::time::Duration::from_secs_f64(self.secs)
  }
}

// Implement conversion traits.

impl FromStr for Duration {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    parse(s).map(From::from)
  }
}

impl From<std::time::Duration> for Duration {
  fn from(value: std::time::Duration) -> Self {
    Self::secs_f64(value.as_secs_f64())
  }
}

impl From<chrono::Duration> for Duration {
  fn from(value: chrono::Duration) -> Self {
    value.to_std().expect("Failed to convert chrono::Duration to std::time::Duration").into()
  }
}

impl From<Duration> for std::time::Duration {
  fn from(value: Duration) -> Self {
    value.to_std()
  }
}

impl From<Duration> for chrono::Duration {
  fn from(value: Duration) -> Self {
    Self::from_std(value.to_std())
      .expect("Failed to convert std::time::Duration to chrono::Duration")
  }
}

// Implement operators.

impl Eq for Duration {}

impl Ord for Duration {
  fn cmp(&self, other: &Self) -> cmp::Ordering {
    self.secs.partial_cmp(&other.secs).unwrap()
  }
}

impl<T> Mul<T> for Duration
where
  T: crate::math::AsPrimitive<f64>,
{
  type Output = Self;

  fn mul(self, rhs: T) -> Self::Output {
    Self::secs_f64(self.secs * rhs.as_())
  }
}

impl<T> Div<T> for Duration
where
  T: crate::math::AsPrimitive<f64>,
{
  type Output = Self;

  fn div(self, rhs: T) -> Self::Output {
    Self::secs_f64(self.secs / rhs.as_())
  }
}
