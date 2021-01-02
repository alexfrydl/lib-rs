// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Formatting utilities.

#[doc(no_inline)]
pub use std::fmt::*;

mod as_path;
mod indent;
mod surround;

pub use self::as_path::*;
pub use self::indent::*;
pub use self::surround::*;
