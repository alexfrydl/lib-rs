// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::prelude::*;
use chrono_tz::{Tz, TZ_VARIANTS};

/// The local time zone.
pub const LOCAL: Zone = Zone::Local;

/// The UTC time zone.
pub const UTC: Zone = Zone::Tz(Tz::UTC);

/// A time zone.
#[derive(Debug, Clone, Copy)]
pub enum Zone {
  Local,
  Tz(Tz),
}

impl Zone {
  /// Returns an iterator over all time zones.
  pub fn all() -> impl Iterator<Item = Self> {
    TZ_VARIANTS.iter().cloned().map(Zone::Tz)
  }

  /// Returns the name of the time zone.
  pub fn name(&self) -> &'static str {
    match &self {
      Self::Local => "Local",
      Self::Tz(tz) => tz.name(),
    }
  }
}

// Implement parsing of zone names.

impl FromStr for Zone {
  type Err = fail::Error;

  fn from_str(s: &str) -> Result<Self> {
    let tz = s.parse().map_err(fail::Error::new)?;

    match tz {
      Tz::UTC => Ok(UTC),
      tz => Ok(Zone::Tz(tz)),
    }
  }
}
