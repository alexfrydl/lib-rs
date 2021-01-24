// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use parse_duration::parse::Error as ParseError;

use crate::math::AsPrimitive;
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
  /// An infinite duration.
  pub const INFINITE: Self = Self { secs: f64::INFINITY };

  /// A zero-length duration.
  pub const ZERO: Self = Self { secs: 0.0 };

  /// Returns a duration of the given number of weeks.
  pub fn weeks(weeks: impl AsPrimitive<f64>) -> Self {
    Self::secs(weeks.as_() * 7.0 * 24.0 * 60.0 * 60.0)
  }

  /// Returns a duration of the given number of days.
  pub fn days(days: impl AsPrimitive<f64>) -> Self {
    Self::secs(days.as_() * 24.0 * 60.0 * 60.0)
  }

  /// Returns a duration of the given number of hours.
  pub fn hours(hours: impl AsPrimitive<f64>) -> Self {
    Self::secs(hours.as_() * 60.0 * 60.0)
  }

  /// Returns a duration of the given number of minutes.
  pub fn mins(mins: impl AsPrimitive<f64>) -> Self {
    Self::secs(mins.as_() * 60.0)
  }

  /// Returns a duration of the given number of seconds.
  pub fn secs(secs: impl AsPrimitive<f64>) -> Self {
    let secs = secs.as_();

    assert!(!secs.is_nan(), "Duration cannot be NaN.");

    Self { secs: secs.max(0.0) }
  }

  /// Returns a duration of the given number of Hz.
  pub fn hz(hz: impl AsPrimitive<f64>) -> Self {
    Self::secs(1.0 / hz.as_())
  }

  /// Returns a duration of the given number of milliseconds.
  pub fn ms(ms: impl AsPrimitive<f64>) -> Duration {
    Self::secs(ms.as_() / 1000.0)
  }

  /// Return the duration as a number of weeks.
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

  /// Returns `true` if this duration represents a finite amount of time.
  pub fn is_finite(&self) -> bool {
    self.secs.is_finite()
  }

  /// Returns `true` if this duration represents an infinite amount of time.
  pub fn is_infinite(&self) -> bool {
    self.secs.is_infinite()
  }

  /// Converts this duration to a `std::time::Duration`.
  pub fn to_std(self) -> std::time::Duration {
    /// The maximum f64 value with whole number precision.
    const MAX_SAFE_INT: f64 = 9007199254740991f64;

    std::time::Duration::from_secs_f64(self.secs.min(MAX_SAFE_INT))
  }
}

impl Add<Self> for Duration {
  type Output = Self;

  fn add(mut self, rhs: Self) -> Self::Output {
    self += rhs;
    self
  }
}

impl AddAssign<Self> for Duration {
  fn add_assign(&mut self, rhs: Self) {
    self.secs += rhs.secs;
  }
}

impl Debug for Duration {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "\"{}\"", self)
  }
}

impl Display for Duration {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if self.secs.is_infinite() {
      write!(f, "forever")
    } else if self.secs < 2.0 {
      write!(f, "{} ms", self.as_ms().round_to_places(3))
    } else if self.secs < 120.0 {
      write!(f, "{} secs", self.as_secs().round_to_places(3))
    } else if self.secs < 7_200.0 {
      write!(f, "{} mins", self.as_mins().round_to_places(2))
    } else if self.secs < 172_800.0 {
      write!(f, "{} hours", self.as_hours().round_to_places(2))
    } else if self.secs < 604_800.0 {
      write!(f, "{} days", self.as_days().round_to_places(2))
    } else if self.secs < 31_557_600.0 {
      write!(f, "{} weeks", self.as_weeks().round_to_places(1))
    } else {
      write!(f, "{} years", (self.secs / 31_557_600.0).round_to_places(1))
    }
  }
}

impl<T> Div<T> for Duration
where
  T: AsPrimitive<f64>,
{
  type Output = Self;

  fn div(mut self, rhs: T) -> Self::Output {
    self /= rhs;
    self
  }
}

impl<T> DivAssign<T> for Duration
where
  T: AsPrimitive<f64>,
{
  fn div_assign(&mut self, rhs: T) {
    self.secs = f64::max(self.secs / rhs.as_(), 0.0);
  }
}

impl Eq for Duration {}

impl From<std::time::Duration> for Duration {
  fn from(value: std::time::Duration) -> Self {
    Self::secs(value.as_secs_f64())
  }
}

impl From<chrono::Duration> for Duration {
  fn from(value: chrono::Duration) -> Self {
    value.to_std().unwrap_or_default().into()
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

impl FromStr for Duration {
  type Err = ParseError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    parse(s).map(From::from)
  }
}

impl<T> Mul<T> for Duration
where
  T: AsPrimitive<f64>,
{
  type Output = Self;

  fn mul(mut self, rhs: T) -> Self::Output {
    self *= rhs;
    self
  }
}

impl<T> MulAssign<T> for Duration
where
  T: AsPrimitive<f64>,
{
  fn mul_assign(&mut self, rhs: T) {
    self.secs = f64::max(self.secs * rhs.as_(), 0.0);
  }
}

impl Ord for Duration {
  fn cmp(&self, other: &Self) -> cmp::Ordering {
    self.secs.partial_cmp(&other.secs).unwrap()
  }
}

impl Sub<Self> for Duration {
  type Output = Self;

  fn sub(mut self, rhs: Self) -> Self::Output {
    self -= rhs;
    self
  }
}

impl SubAssign<Self> for Duration {
  fn sub_assign(&mut self, rhs: Self) {
    self.secs = f64::max(self.secs - rhs.secs, 0.0);
  }
}
