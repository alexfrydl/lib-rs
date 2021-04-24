// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::env;
use crate::prelude::*;
use chrono_tz::{Tz, TZ_VARIANTS};
use std::process::Command;

/// The UTC time zone.
pub const UTC: Zone = Zone(Tz::UTC);

/// A time zone.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Zone(Tz);

impl Zone {
  /// Returns an iterator over all time zones.
  pub fn all() -> impl Iterator<Item = Self> {
    TZ_VARIANTS.iter().cloned().map(Zone)
  }

  /// Returns a time zone from the given name, or `None` if no such timezone
  /// exists.
  pub fn from_name(name: impl AsRef<str>) -> Result<Zone, Unrecognized> {
    name.as_ref().parse()
  }

  /// Returns the local time zone.
  pub fn local() -> Self {
    static ZONE: Lazy<Zone> = Lazy::new(|| {
      // First, check the TZ environment variable.

      if let Some(tz) = env::var("TZ") {
        if let Ok(tz) = tz.parse() {
          return Zone(tz);
        }
      }

      // Next, try OS-specific solutions.

      if cfg!(target_os = "linux") {
        // Try reading from `/etc/timezone`.

        if let Ok(tz) = std::fs::read_to_string("/etc/timezone") {
          if let Ok(tz) = tz.parse() {
            return Zone(tz);
          }
        }

        // Next, try running a command to find the current time zone.

        let output =
          Command::new("timedatectl").args(&["show", "--property=Timezone", "--value"]).output();

        if let Ok(output) = output {
          if output.status.success() {
            if let Ok(tz) = str::from_utf8(&output.stdout) {
              if let Ok(tz) = tz.parse() {
                return Zone(tz);
              }
            }
          }
        }
      }

      // Otherwise, just use UTC.

      Zone::utc()
    });

    *ZONE
  }

  /// Returns the UTC time zone.
  pub const fn utc() -> Self {
    Self(Tz::UTC)
  }

  /// Returns the name of the time zone.
  pub fn name(&self) -> &'static str {
    self.0.name()
  }

  pub(crate) fn as_tz(&self) -> &Tz {
    &self.0
  }
}

impl FromStr for Zone {
  type Err = Unrecognized;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.parse() {
      Ok(tz) => Ok(Zone(tz)),
      Err(_) => Err(Unrecognized),
    }
  }
}

/// An error returned when a time zone name is not recognized.
#[derive(Debug, Error)]
#[error("Unrecognized time zone.")]
pub struct Unrecognized;
