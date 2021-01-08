// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Common derive macros.

pub use af_macros::Error;
pub use derive_more::*;
pub use std::error::Error;

pub(crate) use serde::{Deserialize, Serialize};
