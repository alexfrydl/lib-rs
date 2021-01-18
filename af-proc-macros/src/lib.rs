// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod attr_future_boxed;
mod attr_main;
mod attr_test_main;
mod prelude;

/// Modifies an `async` function to return a `Box<dyn Future + Send>`.
#[proc_macro_attribute]
pub fn future_boxed(
  _: proc_macro::TokenStream,
  item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  attr_future_boxed::run(item, true)
}

/// Modifies an `async` function to return a `Box<dyn Future>`.
#[proc_macro_attribute]
pub fn future_boxed_local(
  _: proc_macro::TokenStream,
  item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  attr_future_boxed::run(item, false)
}

/// Defines an async main function for an `af-core` application.
#[proc_macro_attribute]
pub fn main(_: proc_macro::TokenStream, item: proc_macro::TokenStream) -> proc_macro::TokenStream {
  attr_main::run(item)
}

/// Defines a main function for an `af_core::test` suite.
#[proc_macro_attribute]
pub fn test_main(
  _: proc_macro::TokenStream,
  item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  attr_test_main::run(item)
}
