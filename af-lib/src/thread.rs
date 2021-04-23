// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Thread-based concurrency.

use crate::prelude::*;

pub fn spawn(name: impl Into<String>, future: impl Future + Send + 'static) {
  std::thread::Builder::new()
    .name(name.into())
    .spawn(move || {
      futures_lite::future::block_on(future);
    })
    .unwrap();
}
