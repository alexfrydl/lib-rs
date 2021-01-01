// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

/// Returns a future that awaits a request defined by the given “closure”.
///
/// The “closure” is not a real closure. It may borrow from the current scope,
/// use `.await`, and so on.
#[macro_export]
macro_rules! async_request {
  (|$req:ident| $($args:tt)*) => {{
    let ($req, res) = __af_coresync::Request::new();

    $($args)*;

    res.recv()
  }};
}
