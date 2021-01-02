// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::prelude::*;

/// Runs the `runtime::main` attribute macro.
pub fn main(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
  // Extract function item information.

  let syn::ItemFn { attrs, vis, sig, block } = syn::parse_macro_input!(item as syn::ItemFn);
  let name = &sig.ident;
  let output = &sig.output;

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

  // Generate code to print errors.

  let wrap_result = match output {
    syn::ReturnType::Default => quote! { Ok(result) },
    _ => quote! { Ok(result?) },
  };

  // Generate the output.

  #[allow(unused_mut)]
  let mut init = TokenStream::new();

  #[cfg(feature = "dotenv")]
  init.extend(quote! {
    __af_core::env::load_dotenv();
  });

  let result = quote! {
    #vis fn main() {
      #(#attrs)*
      #sig #block

      #init

      __af_core::runtime::logger::init!();

      __af_core::runtime::run(async {
        let result = #name().await;

        #wrap_result
      })
    }
  };

  result.into()
}
