// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::prelude::*;

/// A list of input futures.
struct FutureList {
  items: Vec<syn::Expr>,
}

/// Runs the `future::join` macro.
pub fn join(args: proc_macro::TokenStream) -> TokenStream {
  let futures = match syn::parse(args) {
    Ok(FutureList { items }) => items,
    Err(err) => abort!(err.span(), err),
  };

  match futures.len() {
    0 => return quote! { () },
    1 => return quote! { ((#(#futures)*).await,) },
    2 => return quote! { __af_corefuture::join(#(#futures),*).await },
    _ => {}
  }

  let future_idents: Vec<_> = futures
    .iter()
    .enumerate()
    .map(|(i, f)| syn::Ident::new(&format!("_{}", i), f.span()))
    .collect();

  quote! {{
    #(let mut #future_idents = __af_corefuture::buffered(#futures);)*

    unsafe {
      #(let mut #future_idents = Pin::new_unchecked(&mut #future_idents);)*

      __af_corefuture::poll_fn(|cx: &mut std::task::Context| -> std::task::Poll<()> {
        let mut ready = true;

        #(
          if let std::task::Poll::Pending = Pin::as_mut(&mut #future_idents).poll(cx) {
            ready = false;
          }
        )*

        match ready {
          true => std::task::Poll::Ready(()),
          false => std::task::Poll::Pending,
        }
      }).await;
    }

    (
      #(#future_idents.into_output(),)*
    )
  }}
}

/// Runs the `future::race` macro.
pub fn race(args: proc_macro::TokenStream) -> TokenStream {
  let futures = match syn::parse(args) {
    Ok(FutureList { items }) => items,
    Err(err) => abort!(err.span(), err),
  };

  match futures.len() {
    0 => return quote! { () },
    1 => return quote! { (#(#futures)*).await },
    2 => return quote! { __af_corefuture::race(#(#futures),*).await },
    _ => {}
  }

  let future_idents: Vec<_> = futures
    .iter()
    .enumerate()
    .map(|(i, f)| syn::Ident::new(&format!("_{}", i), f.span()))
    .collect();

  quote! {{
    #(let mut #future_idents = #futures;)*

    unsafe {
      #(let mut #future_idents = Pin::new_unchecked(&mut #future_idents);)*

      __af_corefuture::poll_fn(|cx: &mut std::task::Context| -> std::task::Poll<_> {
        let mut ready = true;

        #(
          if let std::task::Poll::Ready(value) = Pin::as_mut(&mut #future_idents).poll(cx) {
            return std::task::Poll::Ready(value);
          }
        )*

        std::task::Poll::Pending
      }).await
    }
  }}
}

// Parse a list of futures.

impl Parse for FutureList {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let mut items = Vec::new();

    while !input.is_empty() {
      items.push(input.parse()?);

      if !input.is_empty() {
        input.parse::<Token![,]>()?;
      }
    }

    Ok(Self { items })
  }
}
