// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use anyhow::{bail, format_err as err, Result};
pub use derive_more::*;
pub use inflector::{Inflector as _, InflectorNumbers as _};
pub use proc_macro2::{Span, TokenStream};
pub use proc_macro_error::*;
pub use quote::{quote, ToTokens, TokenStreamExt as _};
pub use std::convert::{TryFrom, TryInto};
pub use std::str::FromStr;
pub use syn::parse::{self, Parse, ParseStream};
pub use syn::spanned::Spanned as _;
pub use syn::Token;
