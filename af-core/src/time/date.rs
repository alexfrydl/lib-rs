// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Zone;
use crate::prelude::*;
use chrono::TimeZone;

#[cfg(feature = "postgres")]
use crate::postgres as pg;

#[derive(Clone, Copy, Eq, From, Into, Ord, PartialEq, PartialOrd)]
pub struct Date(chrono::NaiveDate);

impl Date {
  /// Formats the date according to the given format string.
  pub fn format<'a>(&self, fmt: &'a str) -> impl Display + 'a {
    self.0.format(fmt)
  }

  /// Returns the next day.
  pub fn next(&self) -> Self {
    Self(self.0.succ())
  }

  /// Returns the previous day.
  pub fn prev(&self) -> Self {
    Self(self.0.pred())
  }

  /// Convert the date to a time in the local time zone.
  pub fn to_local_time(&self) -> Time {
    self.to_time(super::LOCAL)
  }

  /// Converts the date to a time in the given time zone.
  pub fn to_time(&self, zone: Zone) -> Time {
    let inner = match &zone {
      Zone::Local => chrono::Local
        .from_local_date(&self.0)
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .with_timezone(&chrono::Utc),

      Zone::Tz(tz) => {
        tz.from_local_date(&self.0).and_hms_opt(0, 0, 0).unwrap().with_timezone(&chrono::Utc)
      }
    };

    Time { inner, zone }
  }

  /// Convert the date to a time in UTC.
  pub fn to_utc_time(&self) -> Time {
    self.to_time(super::UTC)
  }
}

// Implement formatting.

impl Debug for Date {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "\"{}\"", self.format("%F"))
  }
}

impl Display for Date {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.format("%v"))
  }
}

// Implement conversion to and from postgres.

#[cfg(feature = "postgres")]
impl<'a> pg::FromSql<'a> for Date {
  fn from_sql(ty: &pg::Type, raw: &'a [u8]) -> pg::FromSqlResult<Self> {
    chrono::NaiveDate::from_sql(ty, raw).map(Date::from)
  }

  fn accepts(ty: &pg::Type) -> bool {
    <chrono::NaiveDate as pg::FromSql>::accepts(ty)
  }
}

#[cfg(feature = "postgres")]
impl pg::ToSql for Date {
  fn to_sql(&self, ty: &pg::Type, out: &mut pg::BytesMut) -> pg::ToSqlResult
  where
    Self: Sized,
  {
    self.0.to_sql(ty, out)
  }

  fn accepts(ty: &pg::Type) -> bool
  where
    Self: Sized,
  {
    <chrono::NaiveDate as pg::ToSql>::accepts(ty)
  }

  fn to_sql_checked(&self, ty: &pg::Type, out: &mut pg::BytesMut) -> pg::ToSqlResult {
    self.0.to_sql_checked(ty, out)
  }
}
