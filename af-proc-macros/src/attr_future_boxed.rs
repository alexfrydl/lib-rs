// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::prelude::*;

/// Runs the `future_blocked` attribute macro.
pub fn run(item: proc_macro::TokenStream, send: bool) -> proc_macro::TokenStream {
  // Extract function item information.

  let syn::ItemFn { attrs, vis, sig, block } = syn::parse_macro_input!(item as syn::ItemFn);
  let syn::Signature { asyncness, unsafety, fn_token, ident, generics, inputs, output, .. } = sig;

  // Require an async function.

  if asyncness.is_none() {
    abort!(fn_token, "A function must be async to return a boxed future.");
  }

  // Generate a token stream for the output.

  let output = match output {
    syn::ReturnType::Default => quote! { () },
    syn::ReturnType::Type(_, ty) => ty.to_token_stream(),
  };

  // Generate an optional token stream for `B Send`.

  let plus_send = match send {
    true => Some(quote! { + Send }),
    false => None,
  };

  // Generate the final result.

  let result = quote! {
    #(#attrs)*
    #vis #unsafety fn #ident #generics (
      #inputs
    ) -> std::pin::Pin<Box<dyn Future<Output = #output> #plus_send>> {
      Box::pin(async move #block)
    }
  };

  result.into()
}
