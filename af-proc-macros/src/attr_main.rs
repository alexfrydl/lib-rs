// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::prelude::*;

/// Runs the `main` attribute macro.
pub fn run(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
  // Extract function item information.

  let func = syn::parse_macro_input!(item as syn::ItemFn);
  let sig = &func.sig;
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

      Result::<(), std::convert::Infallible>::Ok(())
    },

    _ => quote! {
      #name().await.map_err(|err| err.to_string())
    },
  };

  generate(func, run)
}

pub fn generate(func: syn::ItemFn, run: TokenStream) -> proc_macro::TokenStream {
  let syn::ItemFn { vis, attrs, sig, block, .. } = func;

  // Generate code to call the main function.

  let result = quote! {
    #vis fn main() {
      #(#attrs)*
      #sig #block

      unsafe {
        __af_lib_macro_helpers::__log_init!();

        __af_lib_macro_helpers::__runtime_run(module_path!(), async {
          #run
        });
      }
    }
  };

  result.into()
}
