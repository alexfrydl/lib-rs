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

  // Generate the output.

  #[allow(unused_mut)]
  let mut init = TokenStream::new();

  #[cfg(feature = "logger")]
  init.append_all(quote! {
    af_core::log::init!();
  });

  // Generate code to wrap the output in a result.

  let wrap_result = match &sig.output {
    syn::ReturnType::Default => quote! { Result::<_, i32>::Ok(output) },
    _ => quote! { output },
  };

  // Generate code to call the main function.

  let run = match sig.inputs.len() {
    0 => quote! {
      af_core::run(async {
        let output = #name().await;

        #wrap_result
      });
    },

    _ => quote! {
      af_core::run_with(|cancel_signal| async {
        let output = #name(cancel_signal).await;

        #wrap_result
      });
    },
  };

  let result = quote! {
    #vis fn main() {
      #(#attrs)*
      #sig #block

      #init
      #run
    }
  };

  result.into()
}
