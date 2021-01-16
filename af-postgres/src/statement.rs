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
  params: SmallVec<[&'a dyn ToSql; 32]>,
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
  pub fn add_param(&mut self, param: &'a dyn ToSql) -> usize {
    self.params.push(param);
    self.params.len()
  }

  /// Returns a reference to the statement params.
  pub fn params(&self) -> &[&'a dyn ToSql] {
    &self.params
  }

  /// Returns a reference to the statement text.
  pub fn text(&self) -> &str {
    &self.text
  }
}

// Implement fmt::Write for StatementBuilder to support the write! macro.

impl<'a> fmt::Write for StatementBuilder<'a> {
  fn write_str(&mut self, s: &str) -> fmt::Result {
    self.text.write_str(s)
  }
}