// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A core library and async runtime for Rust applications.

pub use af_core::*;

/// Postgres client.
#[cfg(feature = "postgres")]
pub mod postgres {
  pub use af_postgres::*;
}

/// Sentry client.
#[cfg(feature = "sentry")]
pub mod sentry {
  pub use af_sentry::*;
}

/// Slack client.
#[cfg(feature = "slack")]
pub mod slack {
  pub use af_slack::*;
}
