//! Contains functionality associated with [`DateTime`].

use chrono::{TimeZone as _, Timelike};
use chrono_tz::Tz;

use super::{Date, Duration, TimeZone};
use crate::prelude::*;

/// A date and time in a specific time zone.
#[derive(Clone, Copy, From)]
pub struct DateTime(chrono::DateTime<Tz>);

impl DateTime {
  /// Returns a value representing the maximum local date and time.
  pub fn max_value() -> DateTime {
    DateTime(chrono::MAX_DATETIME.with_timezone(TimeZone::local().as_tz()))
  }

  /// Returns a value representing the minimum local date and time.
  pub fn min_value() -> DateTime {
    DateTime(chrono::MIN_DATETIME.with_timezone(TimeZone::local().as_tz()))
  }

  /// Returns a value representing the current local date and time.
  pub fn now() -> DateTime {
    Self(chrono::Utc::now().with_timezone(TimeZone::local().as_tz()))
  }

  /// Returns a date and time representing a Unix timestamp in milliseconds.
  pub fn from_unix_ms(timestamp: i64) -> Self {
    Self(TimeZone::local().as_tz().timestamp_millis(timestamp))
  }

  /// Formats the date and time according to RFC 3339.
  pub fn as_rfc3339(&self) -> impl Display {
    match self.0.timezone() {
      Tz::UTC => self.format("%FT%T%.fZ"),
      _ => self.format("%FT%T%.f%:z"),
    }
  }

  /// Returns the date component.
  pub fn date(&self) -> Date {
    self.0.date().naive_local().into()
  }

  /// Returns the duration since this date and time occurred.
  ///
  /// If the date and time is in the past, this function returns a zero
  /// duration.
  pub fn duration_since(&self) -> Duration {
    Self::now() - *self
  }

  /// Returns the duration until this date and time occurs.
  ///
  /// If the date and time is in the past, this function returns a zero
  /// duration.
  pub fn duration_until(&self) -> Duration {
    *self - Self::now()
  }

  /// Format the date and time for display.
  pub fn format<'a>(&self, fmt: &'a str) -> impl Display + 'a {
    self.0.format(fmt)
  }

  /// Returns the hour, minute, and second numbers.
  ///
  /// Equivalent to `(dt.hour(), dt.minute(), dt.second())`.
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

  /// Waits until this date and time occurs.
  pub async fn occurred(&self) {
    loop {
      let remaining = self.duration_until();

      if remaining.is_zero() {
        break;
      }

      remaining.elapsed().await;
    }
  }

  /// Returns the second number from 0 to 59.
  pub fn second(&self) -> usize {
    self.0.second() as usize
  }

  /// Returns a new date and time representing midnight at the beginning of the
  /// same day.
  pub fn start_of_day(&self) -> DateTime {
    Self(self.0.date().and_hms(0, 0, 0))
  }

  /// Converts the date and time to the local time zone.
  pub fn to_local(&self) -> Self {
    self.to_zone(TimeZone::local())
  }

  /// Converts the date and time to UTC.
  pub fn to_utc(&self) -> Self {
    self.to_zone(TimeZone::utc())
  }

  /// Converts the date and time to a time zone.
  pub fn to_zone(&self, zone: TimeZone) -> Self {
    Self(self.0.with_timezone(zone.as_tz()))
  }
}

impl PartialEq for DateTime {
  fn eq(&self, other: &Self) -> bool {
    self.0 == other.0
  }
}

impl Add<Duration> for DateTime {
  type Output = Self;

  fn add(self, rhs: Duration) -> Self::Output {
    let rhs: chrono::Duration = rhs.into();

    Self(self.0 + rhs)
  }
}

impl Debug for DateTime {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "\"{}\"", self.format("%+"))
  }
}

impl Display for DateTime {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    self.as_rfc3339().fmt(f)
  }
}

impl Eq for DateTime {}

impl Ord for DateTime {
  fn cmp(&self, other: &Self) -> cmp::Ordering {
    self.0.cmp(&other.0)
  }
}

impl PartialOrd for DateTime {
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    self.0.partial_cmp(&other.0)
  }
}

impl Sub<Duration> for DateTime {
  type Output = Self;

  fn sub(self, rhs: Duration) -> Self::Output {
    let rhs: chrono::Duration = rhs.into();

    Self(self.0 - rhs)
  }
}

impl Sub<DateTime> for DateTime {
  type Output = Duration;

  fn sub(self, rhs: DateTime) -> Self::Output {
    (self.0 - rhs.0).into()
  }
}
