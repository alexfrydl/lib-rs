// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Formatted logging using the [log](https://docs.rs/log) crate.

pub use self::Level::*;
pub use log::{debug, error, info, trace, warn, Level};

use crate::prelude::*;

cfg_if! {
  if #[cfg(feature = "logger")] {
    mod logger;

    pub use self::logger::*;
  }
}
