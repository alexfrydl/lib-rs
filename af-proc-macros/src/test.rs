// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::prelude::*;
use std::fmt::Write;
use syn::punctuated::Punctuated;

/// Arguments to the macros.
struct MacroArgs {
  name: TokenStream,
  expr: TokenStream,
}

/// Runs the `test::context` attribute macro.
pub fn run_context_macro(args: TokenStream) -> proc_macro::TokenStream {
  let MacroArgs { name, expr } = match syn::parse2(args) {
    Ok(options) => options,
    Err(err) => abort!(err.span(), err),
  };

  let result = quote! {
    cx.context(#name, |cx| { #expr; });
  };

  result.into()
}

/// Runs the `test::test` attribute macro.
pub fn run_test_macro(args: TokenStream) -> proc_macro::TokenStream {
  let MacroArgs { name, expr } = match syn::parse2(args) {
    Ok(options) => options,
    Err(err) => abort!(err.span(), err),
  };

  let result = quote! {
    cx.test(#name, async move { #expr; #[allow(unreachable_code)] Ok(()) });
  };

  result.into()
}

impl Parse for MacroArgs {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    // First parse the name.
    let name: syn::LitStr = if input.peek(syn::LitStr) {
      // If the input is a string literal, that is the name.

      input.parse()?
    } else {
      // Otherwise, stringify a series of path-like components.

      let mut buf = String::new();

      loop {
        if input.peek(Token![::]) {
          // Add `::` directly.

          input.parse::<Token![::]>()?;
          buf.push_str("::");
        } else if input.peek(syn::Ident) {
          // Add idents directly.

          let ident: syn::Ident = input.parse()?;

          write!(buf, "{}", ident).unwrap();
        } else if input.peek(syn::Token![<]) {
          // Add generic arguments.

          let generics: syn::AngleBracketedGenericArguments = input.parse()?;

          stringify_generics(&mut buf, generics)?;
        } else if input.peek(syn::token::Paren) {
          // Add function arguments.

          let content;

          syn::parenthesized!(content in input);

          let args = content.parse_terminated(syn::Expr::parse)?;

          stringify_fn_args(&mut buf, args)?;
        } else {
          // Unrecgonized input, finish the name.

          break;
        }
      }

      // Convert the accumulated name into a literal.

      syn::LitStr::new(&buf, Span::call_site())
    };

    // Parse a comma followed by one or more arguments.

    input.parse::<Token![,]>()?;

    let mut args: Punctuated<_, Token![,]> = input.parse_terminated(syn::Expr::parse)?;

    // Remove the last argument, which is the code block or expression to run.

    let expr = match args.pop() {
      Some(pair) => pair.into_value(),
      None => abort!(args, "Expected a function path or an expression."),
    };

    // If the expression to run is a path, assume it's a function that accepts
    // `cx`.

    let expr = match expr {
      syn::Expr::Path(path) => quote! { #path(cx) },
      other => other.to_token_stream(),
    };

    // If there are any other arguments, use them as format arguments for the
    // name.

    let name = match args.is_empty() {
      true => name.to_token_stream(),
      false => quote! { format!(#name, #args) },
    };

    // Return the final result.

    Ok(Self { name, expr })
  }
}

/// Converts a list of function arguments into a string.
fn stringify_fn_args(
  output: &mut String,
  args: Punctuated<syn::Expr, Token![,]>,
) -> syn::Result<()> {
  output.push('(');

  let mut args = args.into_pairs().map(|p| p.into_tuple().0);

  if let Some(first) = args.next() {
    write!(output, "{}", first.to_token_stream().to_string()).unwrap();
  }

  for first in args {
    write!(output, ", {}", first.to_token_stream().to_string()).unwrap();
  }

  output.push(')');

  Ok(())
}

/// Contverts a list of generic arguments into a string.
fn stringify_generics(
  output: &mut String,
  generics: syn::AngleBracketedGenericArguments,
) -> syn::Result<()> {
  let syn::AngleBracketedGenericArguments { colon2_token, args, .. } = generics;

  if colon2_token.is_some() {
    output.push_str("::");
  }

  output.push('<');

  write!(output, "{}", args.to_token_stream().to_string()).unwrap();

  output.push('>');

  Ok(())
}
