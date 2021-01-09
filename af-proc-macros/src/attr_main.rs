// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::prelude::*;

/// Runs the `main` attribute macro.
pub fn run(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
  // Extract function item information.

  let syn::ItemFn { attrs, vis, sig, block } = syn::parse_macro_input!(item as syn::ItemFn);
  let name = &sig.ident;

  // Require an async function.

  if sig.asyncness.is_none() {
    return syn::Error::new_spanned(sig.fn_token, "The runtime main function must be async.")
      .to_compile_error()
      .into();
  }

  // Require no parameters.

  if !sig.inputs.is_empty() {
    return syn::Error::new_spanned(
      sig.inputs,
      "The runtime main function must not have parameters.",
    )
    .to_compile_error()
    .into();
  }

  // Generate the output.

  #[allow(unused_mut)]
  let mut init = TokenStream::new();

  #[cfg(feature = "logger")]
  init.append_all(quote! {
    af_core::log::init!();
  });

  // Generate code to print errors.

  let print_error = match &sig.output {
    syn::ReturnType::Default => quote! {},
    _ => quote! {
      match output {
        Err(err) => {
          eprintln!("{}", err);
          std::process::exit(-1)
        },

        Ok(Err(err)) => {
          eprintln!("{}", err);
          std::process::exit(1)
        },

        _ => {}
      }
    },
  };

  let result = quote! {
    #vis fn main() {
      #(#attrs)*
      #sig #block

      #init

      let output = af_core::thread::block_on(af_core::task::start(async { #name().await }));

      #print_error
    }
  };

  result.into()
}
