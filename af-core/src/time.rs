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
use chrono::TimeZone;

#[cfg(feature = "postgres")]
use crate::postgres as pg;

/// A timestamp with a time zone.
#[derive(Clone, Copy)]
pub struct Time {
  inner: chrono::DateTime<chrono::Utc>,
  zone: Zone,
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

  /// Returns the date component of the time.
  pub fn date(&self) -> Date {
    match self.zone {
      Zone::Local => self.inner.with_timezone(&chrono::Local).date().naive_local(),
      Zone::Tz(tz) => self.inner.with_timezone(&tz).date().naive_local(),
    }
    .into()
  }

  /// Returns the start time of the day represented by this time value.
  pub fn start_of_day(&self) -> Time {
    let inner = match &self.zone {
      Zone::Local => {
        self.inner.with_timezone(&chrono::Local).date().and_hms(0, 0, 0).with_timezone(&chrono::Utc)
      }

      Zone::Tz(tz) => {
        self.inner.with_timezone(tz).date().and_hms(0, 0, 0).with_timezone(&chrono::Utc)
      }
    };

    Time { inner, zone: self.zone }
  }

  /// Format the time for display.
  pub fn format<'a>(&self, fmt: &'a str) -> impl Display + 'a {
    match self.zone {
      Zone::Local => self.inner.with_timezone(&chrono::Local).format(fmt),
      Zone::Tz(tz) => self.inner.with_timezone(&tz).format(fmt),
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
    match &self.zone {
      Zone::Local => self.inner.with_timezone(&chrono::Local).naive_local(),
      Zone::Tz(tz) => self.inner.with_timezone(tz).naive_local(),
    }
  }
}

// Implement operators.

impl PartialEq for Time {
  fn eq(&self, other: &Self) -> bool {
    self.inner == other.inner
  }
}

impl Eq for Time {}

impl PartialOrd for Time {
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    self.inner.partial_cmp(&other.inner)
  }
}

impl Ord for Time {
  fn cmp(&self, other: &Self) -> cmp::Ordering {
    self.inner.cmp(&other.inner)
  }
}

impl Add<Duration> for Time {
  type Output = Self;

  fn add(self, rhs: Duration) -> Self::Output {
    let rhs: chrono::Duration = rhs.into();

    Self { inner: self.inner + rhs, zone: self.zone }
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

// Implement formatting.

impl Debug for Time {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "\"{}\"", self.format("%+"))
  }
}

impl Display for Time {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.format("%c"))
  }
}

// Implement conversion to and from postgres.

#[cfg(feature = "postgres")]
impl<'a> pg::FromSql<'a> for Time {
  fn from_sql(ty: &pg::Type, raw: &'a [u8]) -> pg::FromSqlResult<Self> {
    Ok(Self { inner: pg::FromSql::from_sql(ty, raw)?, zone: Zone::Local })
  }

  fn accepts(ty: &pg::Type) -> bool {
    ty.oid() == pg::Type::TIMESTAMPTZ.oid()
  }
}

#[cfg(feature = "postgres")]
impl pg::ToSql for Time {
  fn to_sql(&self, ty: &pg::Type, out: &mut pg::BytesMut) -> pg::ToSqlResult
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

  fn to_sql_checked(&self, ty: &pg::Type, out: &mut pg::BytesMut) -> pg::ToSqlResult {
    if ty.oid() == pg::Type::TIMESTAMP.oid() {
      self.to_naive().to_sql_checked(ty, out)
    } else {
      self.inner.to_sql_checked(ty, out)
    }
  }
}
