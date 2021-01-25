// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A core library and async runtime for Rust applications.
//!
//! # Quick start
//!
//! In `Cargo.toml`:
//! ```toml
//! [dependencies.af]
//! package = "af-lib"
//! version = "0.1"
//! ```
//!
//! In `src/main.rs`:
//! ```ignore
//! use af::prelude::*;
//!
//! #[af::main]
//! async fn main() {
//!   info!("Hello world!");
//! }
//! ```

pub use af_core::random;
pub use af_core::*;

/// [PostgreSQL](https://postresql.org) database client.
#[cfg(feature = "postgres")]
pub mod postgres {
  pub use af_postgres::*;
}

/// [Sentry](https://sentry.io) error reporting.
#[cfg(feature = "sentry")]
pub mod sentry {
  pub use af_sentry::*;
}

#[cfg(feature = "slack")]
/// [Slack](https://slack.com) app creation.
pub mod slack {
  pub use af_slack::*;
}
