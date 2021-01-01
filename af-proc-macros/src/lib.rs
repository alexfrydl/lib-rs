// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

mod future;
mod prelude;
mod runtime;

use crate::prelude::*;

/// A derive macro for the `Error` trait that uses all the default method
/// implementations.
#[proc_macro_derive(Error)]
pub fn derive_error(item: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let item = syn::parse_macro_input!(item as syn::Item);

  let (name, generics) = match &item {
    syn::Item::Enum(item) => (&item.ident, &item.generics),
    syn::Item::Struct(item) => (&item.ident, &item.generics),
    _ => abort!(item.span(), "Expected enum or a struct."),
  };

  let generic_params = &generics.params;
  let where_clause = &generics.where_clause;

  let result = quote! {
    impl<#generic_params> std::error::Error for #name<#generic_params> #where_clause {}
  };

  result.into()
}

/// Waits for all given futures to complete and returns their outputs as a
/// tuple.
#[proc_macro]
#[proc_macro_error]
pub fn future_join(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  future::join(input).into()
}

/// Waits for one of the given futures to complete and then returns the output.
///
/// The other futures are canceled.
#[proc_macro]
#[proc_macro_error]
pub fn future_race(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  future::race(input).into()
}

/// Defines an async main function that runs on the af-core runtime.
///
/// ## Example
///
/// ```ignore
/// use __af_coreprelude::*;
///
/// #[__af_coremain]
/// async fn main() {
///   println!("Hello af-core!");
/// }
/// ```
///
/// ## Returning a result
///
/// The `main` function may return a `Result<(), T>`. If the return value is an
/// `Err(T)` the error is written to stderr and the process exits with an exit
/// code of `1`.
///
/// ```ignore
/// #[__af_coremain]
/// async fn main() -> Result<(), String> {
///   Err("This message is written to stderr.".into())
/// }
/// ```
#[proc_macro_attribute]
pub fn runtime_main(
  _: proc_macro::TokenStream,
  item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
  runtime::main(item)
}
