// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Types and utilities for working with times, dates, and durations.

mod date;
pub mod duration;
mod zone;

pub use self::date::Date;
pub use self::duration::Duration;
pub use self::zone::{Zone, LOCAL, UTC};

use crate::prelude::*;
use chrono::{TimeZone, Timelike};

/// A timestamp with a time zone.
#[derive(Clone, Copy)]
pub struct Time {
  inner: chrono::DateTime<chrono::Utc>,
  zone: Zone,
}

macro_rules! in_zone {
  ($self:tt, |$var:ident| $expr:expr) => {
    match $self.zone {
      Zone::Local => {
        let $var = $self.inner.with_timezone(&chrono::Local);
        $expr
      }

      Zone::Tz(tz) => {
        let $var = $self.inner.with_timezone(&tz);
        $expr
      }
    }
  };
}

impl Time {
  /// Returns a value representing the maximum local date and time.
  pub const fn max_value() -> Time {
    Time { inner: chrono::MAX_DATETIME, zone: LOCAL }
  }

  /// Returns a value representing the minimum local date and time.
  pub const fn min_value() -> Time {
    Time { inner: chrono::MIN_DATETIME, zone: LOCAL }
  }

  /// Returns a value representing the current local date and time.
  pub fn now() -> Time {
    Time { inner: chrono::Utc::now(), zone: LOCAL }
  }

  /// Returns a value representing the given Unix timestamp in milliseconds.
  pub fn from_unix_ms(timestamp: i64) -> Self {
    Self { inner: chrono::Utc.timestamp_millis(timestamp), zone: Zone::Local }
  }

  /// Formats the time according to RFC 3339.
  pub fn as_rfc3339<'a>(&'a self) -> impl Display + 'a {
    match self.zone == UTC {
      true => self.format("%FT%T%.fZ"),
      false => self.format("%FT%T%.f%:z"),
    }
  }

  /// Returns the date component of the time.
  pub fn date(&self) -> Date {
    in_zone!(self, |t| t.date().naive_local().into())
  }

  /// Returns the duration elapsed since this time occurred.
  ///
  /// If the time is in the future, this function returns [`Duration::ZERO`].
  pub fn elapsed(&self) -> Duration {
    Self::now() - *self
  }

  /// Format the time for display.
  pub fn format<'a>(&self, fmt: &'a str) -> impl Display + 'a {
    in_zone!(self, |t| t.format(fmt))
  }

  /// Returns the hour, minute, and second numbers.
  ///
  /// Equivalent to `(time.hour(), time.minute(), time.second())`.
  pub fn hms(&self) -> (usize, usize, usize) {
    in_zone!(self, |t| (t.hour() as usize, t.minute() as usize, t.second() as usize))
  }

  /// Returns the hour from 0 to 23.
  pub fn hour(&self) -> usize {
    in_zone!(self, |t| t.hour() as usize)
  }

  /// Returns the minute from 0 to 59.
  pub fn minute(&self) -> usize {
    in_zone!(self, |t| t.minute() as usize)
  }

  /// Returns the second number from 0 to 59.
  pub fn second(&self) -> usize {
    in_zone!(self, |t| t.second() as usize)
  }

  /// Returns the start time of the day represented by this time value.
  pub fn start_of_day(&self) -> Time {
    Time {
      inner: in_zone!(self, |t| t.date().and_hms(0, 0, 0).with_timezone(&chrono::Utc)),
      zone: self.zone,
    }
  }

  /// Converts to the local time zone.
  pub const fn to_local(&self) -> Self {
    self.to_zone(LOCAL)
  }

  /// Converts to UTC.
  pub const fn to_utc(&self) -> Self {
    self.to_zone(UTC)
  }

  /// Converts to the given time zone.
  pub const fn to_zone(&self, zone: Zone) -> Self {
    Self { inner: self.inner, zone }
  }

  /// Converts to a `NaiveDateTime`.
  #[cfg(feature = "postgres")]
  fn to_naive(&self) -> chrono::NaiveDateTime {
    in_zone!(self, |t| t.naive_local())
  }
}

// Implement operators.

impl PartialEq for Time {
  fn eq(&self, other: &Self) -> bool {
    self.inner == other.inner
  }
}

impl Add<Duration> for Time {
  type Output = Self;

  fn add(self, rhs: Duration) -> Self::Output {
    let rhs: chrono::Duration = rhs.into();

    Self { inner: self.inner + rhs, zone: self.zone }
  }
}

impl Debug for Time {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "\"{}\"", self.format("%+"))
  }
}

impl Eq for Time {}

impl Ord for Time {
  fn cmp(&self, other: &Self) -> cmp::Ordering {
    self.inner.cmp(&other.inner)
  }
}

impl PartialOrd for Time {
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    self.inner.partial_cmp(&other.inner)
  }
}

impl Sub<Duration> for Time {
  type Output = Self;

  fn sub(self, rhs: Duration) -> Self::Output {
    let rhs: chrono::Duration = rhs.into();

    Self { inner: self.inner - rhs, zone: self.zone }
  }
}

impl Sub<Time> for Time {
  type Output = Duration;

  fn sub(self, rhs: Time) -> Self::Output {
    (self.inner - rhs.inner).into()
  }
}

impl Display for Time {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.format("%c"))
  }
}

// Implement conversion to and from postgres.

cfg_if! {
  if #[cfg(feature = "postgres")] {
    use postgres_types as pg;

    impl<'a> pg::FromSql<'a> for Time {
      fn from_sql(ty: &pg::Type, raw: &'a [u8]) -> Result<Self, Box<dyn Error + Sync + Send>>{
        Ok(Self { inner: pg::FromSql::from_sql(ty, raw)?, zone: Zone::Local })
      }

      fn accepts(ty: &pg::Type) -> bool {
        ty.oid() == pg::Type::TIMESTAMPTZ.oid()
      }
    }

    impl pg::ToSql for Time {
      fn to_sql(&self, ty: &pg::Type, out: &mut bytes::BytesMut) -> Result<pg::IsNull, Box<dyn Error + Sync + Send>>
      where
        Self: Sized,
      {
        if ty.oid() == pg::Type::TIMESTAMP.oid() {
          self.to_naive().to_sql(ty, out)
        } else {
          self.inner.to_sql(ty, out)
        }
      }

      fn accepts(ty: &pg::Type) -> bool
      where
        Self: Sized,
      {
        ty.oid() == pg::Type::TIMESTAMP.oid() || ty.oid() == pg::Type::TIMESTAMPTZ.oid()
      }

      fn to_sql_checked(&self, ty: &pg::Type, out: &mut bytes::BytesMut) -> Result<pg::IsNull, Box<dyn Error + Sync + Send>> {
        if ty.oid() == pg::Type::TIMESTAMP.oid() {
          self.to_naive().to_sql_checked(ty, out)
        } else {
          self.inner.to_sql_checked(ty, out)
        }
      }
    }
  }
}
