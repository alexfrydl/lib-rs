// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod derive;
pub mod encoding;
pub mod env;
pub mod fail;
pub mod fmt;
pub mod fs;
pub mod future;
pub mod iter;
pub mod log;
pub mod math;
pub mod prelude;
pub mod random;
pub mod stream;
mod symbol;
pub mod sync;
pub mod thread;
pub mod time;
pub mod uuid;

pub use self::fail::fail;
pub use self::random::{random, Random};
pub use self::symbol::Symbol;
pub use self::uuid::Uuid;
pub use af_macros::{attempt, attempt_async};

#[cfg(feature = "postgres")]
pub mod postgres;

#[cfg(feature = "runtime")]
pub mod runtime;

#[cfg(feature = "runtime")]
pub use self::{
  runtime::main,
  runtime::task::{self, Task},
};
