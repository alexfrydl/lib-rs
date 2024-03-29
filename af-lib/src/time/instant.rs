// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Duration;
use crate::prelude::*;

/// A measurement of monotonically increasing time.
#[derive(Clone, Copy, Eq, From, Hash, Into, Ord, PartialEq, PartialOrd)]
pub struct Instant(std::time::Instant);

impl Instant {
  /// Returns the duration since this instant in time occurred.
  pub fn duration_since(&self) -> Duration {
    *self - Self::now()
  }

  /// Returns the duration until this instant in time occurs.
  pub fn duration_until(&self) -> Duration {
    Self::now() - *self
  }

  /// Waits until this instant in time occurs.
  pub async fn occurred(&self) {
    loop {
      let remaining = self.duration_until();

      if remaining.is_zero() {
        break;
      }

      remaining.elapsed().await;
    }
  }

  /// Returns a value representing “now.”
  ///
  /// Unlike [`DateTime::now()`], consecutive calls to this function are
  /// gauranteed to return a value that is greater than or equal to all previously
  /// returned values.
  pub fn now() -> Instant {
    Instant(std::time::Instant::now())
  }

  /// Converts to a [`std::time::Instant`].
  pub fn to_std(&self) -> std::time::Instant {
    self.0
  }
}

impl Add<Duration> for Instant {
  type Output = Instant;

  fn add(mut self, rhs: Duration) -> Self::Output {
    self += rhs;
    self
  }
}

impl AddAssign<Duration> for Instant {
  fn add_assign(&mut self, rhs: Duration) {
    self.0 += rhs.to_std();
  }
}

impl Debug for Instant {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.0.fmt(f)
  }
}

impl Sub<Instant> for Instant {
  type Output = Duration;

  fn sub(self, rhs: Instant) -> Self::Output {
    (rhs.0 - self.0).into()
  }
}

impl Sub<Duration> for Instant {
  type Output = Instant;

  fn sub(mut self, rhs: Duration) -> Self::Output {
    self -= rhs;
    self
  }
}

impl SubAssign<Duration> for Instant {
  fn sub_assign(&mut self, rhs: Duration) {
    self.0 -= rhs.to_std();
  }
}
