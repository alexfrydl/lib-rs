// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::{Time, Zone};
use crate::prelude::*;
use chrono::{Datelike, TimeZone};

#[derive(Clone, Copy, Eq, From, Into, Ord, PartialEq, PartialOrd)]
pub struct Date(chrono::NaiveDate);

impl Date {
  /// Creates a date from a given year, month, and day number.
  pub fn from_ymd(year: isize, month: usize, day: usize) -> Self {
    Self(chrono::NaiveDate::from_ymd(year as i32, month as u32, day as u32))
  }

  /// Returns the day of the month starting from `1`.
  pub fn day(&self) -> usize {
    self.0.day() as usize
  }

  /// Formats the date according to the given format string.
  pub fn format<'a>(&self, fmt: &'a str) -> impl Display + 'a {
    self.0.format(fmt)
  }

  /// Returns the month of the year starting from `1`.
  pub fn month(&self) -> usize {
    self.0.month() as usize
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
    self.to_time(Zone::local())
  }

  /// Converts the date to a time in the given time zone.
  pub fn to_time(&self, zone: Zone) -> Time {
    Time(zone.as_tz().from_local_date(&self.0).and_hms_opt(0, 0, 0).unwrap())
  }

  /// Convert the date to a time in UTC.
  pub fn to_utc_time(&self) -> Time {
    self.to_time(super::UTC)
  }

  /// Returns the year number.
  pub fn year(&self) -> isize {
    self.0.year() as isize
  }

  /// Returns the year, month of the year, and day of the month.
  ///
  /// Equivalent to `(date.year(), date.month(), date.day())`.
  pub fn ymd(&self) -> (isize, usize, usize) {
    (self.year(), self.month(), self.day())
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
