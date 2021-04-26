// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Proc macros for [af-core](https://docs.rs/af-core/0.1).

mod attr_main;
mod attr_test_main;
mod prelude;

use proc_macro_error::proc_macro_error;

/// An attribute macro to be applied to the `main()` function of an executable.
///
/// This macro automatically runs boilerplate runtime initialization.
#[proc_macro_error]
#[proc_macro_attribute]
pub fn main(_: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
  attr_main::run(item)
}

/// Defines a main function for an `af_core::test` suite.
#[proc_macro_error]
#[proc_macro_attribute]
pub fn test_main(
  _: proc_macro::TokenStream,
  item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  attr_test_main::run(item)
}
