// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod context;
mod output;
mod path;

pub mod prelude {
  pub use crate::prelude::*;
  pub use crate::test;
}

pub use self::context::Context;
pub use self::output::{Output, OutputStream};
pub use self::path::Path;

#[cfg(feature = "test-runner")]
pub mod runner;

#[cfg(feature = "test-runner")]
pub use self::runner::main;
