// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Macros for [af-core](https://docs.rs/af-core/0.1).

mod failure;
mod path;
mod util;

pub use af_proc_macros::*;

#[cfg(feature = "logger")]
mod logger;
