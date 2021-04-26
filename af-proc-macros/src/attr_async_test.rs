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
  let syn::ItemFn { attrs, block, sig, vis, .. } = &func;

  // Check requirements.

  if sig.asyncness.is_none() {
    abort!(sig.fn_token, "test function must be async");
  }

  if !sig.inputs.is_empty() {
    abort!(sig.inputs, "test function must not have inputs");
  }

  if !matches!(sig.output, syn::ReturnType::Default) {
    abort!(sig.inputs, "test function must not return a value");
  }

  // Generate a synchronous test.

  let name = &sig.ident;
  let name_str = syn::LitStr::new(&name.to_string(), name.span());

  let result = quote! {
    #[test]
    #(#attrs)*
    #vis fn #name() {
      let result = __af_lib_macro_helpers::__run_scope_sync(async #block);

      if let Err(err) = result {
        eprintln!("test {:?} {}", #name_str, __af_lib_macro_helpers::__fmt_indent("", "  ", err));
        panic!("async test failure");
      }
    }
  };

  result.into()
}
