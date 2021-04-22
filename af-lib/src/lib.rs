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

pub mod channel;
pub mod derive;
pub mod env;
pub mod error;
pub mod failure;
pub mod fmt;
pub mod fs;
pub mod future;
pub mod iter;
pub mod lazy;
pub mod log;
pub mod math;
pub mod path;
pub mod prelude;
pub mod random;
pub mod string;
pub mod time;
pub mod util;

pub use self::channel::channel;
pub use self::failure::fail;
pub use self::random::random;
pub use af_macros::main;
pub use serde_json as json;
