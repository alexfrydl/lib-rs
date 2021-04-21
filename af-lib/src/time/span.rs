// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::prelude::*;

/// An inclusive span between a start and end time.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Span {
  start: Time,
  end: Time,
}

impl Span {
  /// Creates a new span between two given times.
  ///
  /// The earlier time becomes the start time, and the later time becomes the
  /// end time.
  pub fn new(a: Time, b: Time) -> Self {
    Self::from(a) + b
  }

  /// Returns `true` if a given time is contained with the span.
  pub fn contains(&self, time: Time) -> bool {
    time >= self.start && time <= self.end
  }

  /// Returns a [`Duration`] equal to the time span in length.
  pub fn duration(&self) -> Duration {
    self.end - self.start
  }

  /// Returns the end time of the span.
  pub fn end(&self) -> Time {
    self.end
  }

  /// Returns `true` if the span overlaps another given span.
  pub fn overlaps(&self, rhs: Span) -> bool {
    self.start <= rhs.end && rhs.start <= self.end
  }

  /// Returns the start time of the span.
  pub fn start(&self) -> Time {
    self.start
  }
}

impl Add<Self> for Span {
  type Output = Self;

  fn add(mut self, rhs: Self) -> Self::Output {
    self += rhs;
    self
  }
}

impl AddAssign<Self> for Span {
  fn add_assign(&mut self, rhs: Self) {
    self.start = cmp::min(self.start, rhs.start);
    self.end = cmp::max(self.end, rhs.end);
  }
}

impl Add<Time> for Span {
  type Output = Self;

  fn add(mut self, rhs: Time) -> Self::Output {
    self += rhs;
    self
  }
}

impl AddAssign<Time> for Span {
  fn add_assign(&mut self, rhs: Time) {
    self.start = cmp::min(self.start, rhs);
    self.end = cmp::max(self.end, rhs);
  }
}

impl Display for Span {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    Display::fmt(&self.start, f)?;
    write!(f, " — ")?;
    Display::fmt(&self.end, f)
  }
}

impl From<Time> for Span {
  fn from(time: Time) -> Self {
    Self { start: time, end: time }
  }
}

impl From<Range<Time>> for Span {
  fn from(range: Range<Time>) -> Self {
    Self::new(range.start, range.end)
  }
}

impl From<RangeInclusive<Time>> for Span {
  fn from(range: RangeInclusive<Time>) -> Self {
    let (start, end) = range.into_inner();

    Self::new(start, end)
  }
}
