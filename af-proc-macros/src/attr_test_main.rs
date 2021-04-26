// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::attr_main::generate;
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

  if sig.inputs.len() != 1 {
    abort!(sig.inputs, "main function should have a single &test::Context input");
  }

  if !matches!(sig.output, syn::ReturnType::Default) {
    abort!(sig.inputs, "main function must not return a value");
  }

  // Generate code to call the main function.

  let run = quote! {
    let output = __af_lib_macro_helpers::__run_test_suite(#name).await;

    __af_lib_macro_helpers::__set_exit_code(output.failed_count as i8);
    __af_lib_macro_helpers::__flush_log().await;

    eprintln!("\n{:#}", output);
  };

  generate(func, run)
}
