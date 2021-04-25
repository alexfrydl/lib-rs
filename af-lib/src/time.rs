// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Time-related types and utilities.

mod date;
mod duration;
mod span;
mod zone;

use chrono::{TimeZone, Timelike};
use chrono_tz::Tz;

pub use self::date::Date;
pub use self::duration::{delay, Duration};
pub use self::span::Span;
pub use self::zone::{Zone, UTC};
use crate::prelude::*;

/// A timestamp with a time zone.
#[derive(Clone, Copy)]
pub struct Time(chrono::DateTime<Tz>);

impl Time {
  /// Returns a value representing the maximum local date and time.
  pub fn max_value() -> Time {
    Time(chrono::MAX_DATETIME.with_timezone(Zone::local().as_tz()))
  }

  /// Returns a value representing the minimum local date and time.
  pub fn min_value() -> Time {
    Time(chrono::MIN_DATETIME.with_timezone(Zone::local().as_tz()))
  }

  /// Returns a value representing the current local date and time.
  pub fn now() -> Time {
    Self(chrono::Utc::now().with_timezone(Zone::local().as_tz()))
  }

  /// Returns a value representing the given Unix timestamp in milliseconds.
  pub fn from_unix_ms(timestamp: i64) -> Self {
    Self(Zone::local().as_tz().timestamp_millis(timestamp))
  }

  /// Formats the time according to RFC 3339.
  pub fn as_rfc3339(&self) -> impl Display {
    match self.0.timezone() {
      Tz::UTC => self.format("%FT%T%.fZ"),
      _ => self.format("%FT%T%.f%:z"),
    }
  }

  /// Returns the date component of the time.
  pub fn date(&self) -> Date {
    self.0.date().naive_local().into()
  }

  /// Returns the duration elapsed since this time occurred.
  ///
  /// If the time is in the future, this function returns [`Duration::ZERO`].
  pub fn elapsed(&self) -> Duration {
    Self::now() - *self
  }

  /// Format the time for display.
  pub fn format<'a>(&self, fmt: &'a str) -> impl Display + 'a {
    self.0.format(fmt)
  }

  /// Returns the hour, minute, and second numbers.
  ///
  /// Equivalent to `(time.hour(), time.minute(), time.second())`.
  pub fn hms(&self) -> (usize, usize, usize) {
    (self.hour(), self.minute(), self.second())
  }

  /// Returns the hour from 0 to 23.
  pub fn hour(&self) -> usize {
    self.0.hour() as usize
  }

  /// Returns the minute from 0 to 59.
  pub fn minute(&self) -> usize {
    self.0.minute() as usize
  }

  /// Returns the second number from 0 to 59.
  pub fn second(&self) -> usize {
    self.0.second() as usize
  }

  /// Returns the start time of the day represented by this time value.
  pub fn start_of_day(&self) -> Time {
    Self(self.0.date().and_hms(0, 0, 0))
  }

  /// Converts to the local time zone.
  pub fn to_local(&self) -> Self {
    self.to_zone(Zone::local())
  }

  /// Converts to UTC.
  pub fn to_utc(&self) -> Self {
    self.to_zone(UTC)
  }

  /// Converts to the given time zone.
  pub fn to_zone(&self, zone: Zone) -> Self {
    Self(self.0.with_timezone(zone.as_tz()))
  }
}

// Implement operators.

impl PartialEq for Time {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

impl Add<Duration> for Time {
  type Output = Self;

  fn add(self, rhs: Duration) -> Self::Output {
    let rhs: chrono::Duration = rhs.into();

    Self(self.0 + rhs)
  }
}

impl Debug for Time {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "\"{}\"", self.format("%+"))
  }
}

impl Display for Time {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.as_rfc3339().fmt(f)
  }
}

impl Eq for Time {}

impl Ord for Time {
  fn cmp(&self, other: &Self) -> cmp::Ordering {
    self.0.cmp(&other.0)
  }
}

impl PartialOrd for Time {
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    self.0.partial_cmp(&other.0)
  }
}

impl Sub<Duration> for Time {
  type Output = Self;

  fn sub(self, rhs: Duration) -> Self::Output {
    let rhs: chrono::Duration = rhs.into();

    Self(self.0 - rhs)
  }
}

impl Sub<Time> for Time {
  type Output = Duration;

  fn sub(self, rhs: Time) -> Self::Output {
    (self.0 - rhs.0).into()
  }
}
