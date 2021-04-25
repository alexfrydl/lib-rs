// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// Defers an expression so that it is run at the end of the current block.
#[macro_export]
macro_rules! defer {
  ($($tt:tt)*) => {
    let guard = defer(|| {
      $($tt)*;
    });
  };
}
