// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::prelude::*;

/// Runs the `test::main` attribute macro.
pub fn run(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
  // Extract function item information.

  let syn::ItemFn { attrs, vis, sig, block } = syn::parse_macro_input!(item as syn::ItemFn);
  let name = &sig.ident;

  // Generate the final result.

  let result = quote! {
    #vis fn main() {
      #(#attrs)*
      #sig #block

      af_core::log::init!();

      let result = af_core::thread::block_on(af_core::test::runner::run(#name));

      if result.is_err() {
        std::process::exit(1);
      }
    }
  };

  result.into()
}
