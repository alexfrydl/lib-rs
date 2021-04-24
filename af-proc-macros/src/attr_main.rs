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

  // Check requirements.

  if sig.asyncness.is_none() {
    abort!(sig.fn_token, "main function must be async");
  }

  if !sig.inputs.is_empty() {
    abort!(sig.inputs, "main function must not have inputs");
  }

  // Wrap the function call depending on whether it returns a result.

  let run = match sig.output {
    syn::ReturnType::Default => quote! {
      #name().await;
      Ok(())
    },

    _ => quote! {
      #name().await.map_err(|err| err.to_string())
    },
  };

  // Generate the output.

  let mut code = TokenStream::new();

  #[cfg(feature = "logger")]
  code.append_all(quote! {
    af_lib::log::init!();
  });

  code.append_all(quote! {
    unsafe {
      af_lib::concurrency::runtime::run(module_path!(), async { #run });
    }
  });

  // Generate code to call the main function.

  let result = quote! {
    #vis fn main() {
      #(#attrs)*
      #sig #block

      #code
    }
  };

  result.into()
}
