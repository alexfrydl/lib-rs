// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use tokio_postgres::{Statement, ToStatement};

use crate::ToSql;
use af_core::prelude::*;
use smallstr::SmallString;
use smallvec::SmallVec;

/// A helper struct for building a complex one-off statement its parameter
/// values.
#[derive(Debug)]
pub struct StatementBuilder<'a> {
  text: SmallString<[u8; 2048]>,
  params: SmallVec<[&'a (dyn ToSql + Sync); 32]>,
}

impl<'a> StatementBuilder<'a> {
  /// Creates a new, empty statement builder.
  pub fn new() -> Self {
    Self { text: SmallString::new(), params: SmallVec::new() }
  }

  /// Appends text to the query.
  pub fn append(&mut self, text: impl Display) {
    write!(self.text, "{}", text).unwrap();
  }

  /// Adds a parameter to the statement and returns its index.
  ///
  /// The index is the number to use as a placeholder in statement text. This
  /// is intended for adding parameters when the number of parameters is not
  /// known at compile time. For example:
  /// ```ignore
  /// write!(builder, "WHERE id = ${}", builder.add_param(id))?;
  /// ```
  ///
  pub fn add_param(&mut self, param: &'a (dyn ToSql + Sync)) -> usize {
    self.params.push(param);
    self.params.len()
  }

  /// Returns a reference to the statement params.
  pub fn params(&self) -> &[&'a (dyn ToSql + Sync)] {
    &self.params
  }

  /// Returns a reference to the statement text.
  pub fn text(&self) -> &str {
    &self.text
  }
}

impl Default for StatementBuilder<'_> {
  fn default() -> Self {
    Self::new()
  }
}

// Implement fmt::Write for StatementBuilder to support the write! macro.

impl<'a> fmt::Write for StatementBuilder<'a> {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    self.text.write_str(s)
  }
}
