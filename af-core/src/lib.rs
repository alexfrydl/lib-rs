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
//! [dependencies]
//! af-core = "0.1"
//! ```
//!
//! In `src/main.rs`:
//! ```ignore
//! use af_core::prelude::*;
//!
//! #[af_core::main]
//! async fn main() {
//!   info!("Hello world!");
//! }
//! ```

pub mod atomic;
pub mod channel;
pub mod derive;
pub mod env;
pub mod error;
pub mod fail;
pub mod fmt;
pub mod future;
pub mod iter;
pub mod lazy;
pub mod log;
pub mod math;
pub mod path;
pub mod prelude;
pub mod random;
pub mod stream;
pub mod string;
pub mod task;
pub mod test;
pub mod thread;
pub mod time;
pub mod util;

pub use self::fail::fail;
pub use self::random::random;
pub use af_core_macros::main;
pub use serde_json as json;
