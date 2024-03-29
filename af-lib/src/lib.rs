// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A modular library for async Rust applications.
//!
//! # Quick start
//!
//! In `Cargo.toml`:
//! ```toml
//! [dependencies.af]
//! package = "af-lib"
//! version = "0.2"
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

pub mod concurrency;
pub mod fs;
pub mod math;
pub mod prelude;
pub mod time;
pub mod util;

pub use af_macros::{async_test, main};
