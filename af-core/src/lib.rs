// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod channel;
pub mod derive;
pub mod env;
pub mod fail;
pub mod fmt;
pub mod future;
pub mod iter;
pub mod log;
pub mod math;
pub mod path;
pub mod prelude;
pub mod random;
mod run;
pub mod string;
pub mod task;
pub mod thread;
pub mod time;
pub mod util;
pub mod uuid;

pub use self::fail::fail;
pub use self::random::{random, Random};
pub use self::run::{run, run_with};
pub use self::uuid::Uuid;
pub use af_macros::main;
