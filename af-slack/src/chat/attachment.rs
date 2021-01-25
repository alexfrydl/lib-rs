// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::block::Block;
use af_core::prelude::*;
use serde::ser::SerializeStruct;

/// A message attachment.
#[derive(Debug)]
pub struct Attachment<'a> {
  /// A list of content blocks.
  pub blocks: Vec<Block<'a>>,
  /// The color of the attachment.
  pub color: Cow<'a, str>,
}

impl<'a> Attachment<'a> {
  /// Creates a new, blank attachment with the default color.
  pub const fn new() -> Self {
    Self::info()
  }

  /// Creates a new, blank attachment with a “debug” color.
  pub const fn debug() -> Self {
    Self { blocks: Vec::new(), color: Cow::Borrowed("debug") }
  }

  /// Creates a new, blank attachment with an “info” color.
  pub const fn info() -> Self {
    Self { blocks: Vec::new(), color: Cow::Borrowed("info") }
  }

  /// Creates a new, blank attachment with a “warn” color.
  pub const fn warn() -> Self {
    Self { blocks: Vec::new(), color: Cow::Borrowed("warn") }
  }

  /// Creates a new, blank attachment with an “error” color.
  pub const fn error() -> Self {
    Self { blocks: Vec::new(), color: Cow::Borrowed("error") }
  }

  /// Creates a new, blank attachment with a “success” color.
  pub const fn success() -> Self {
    Self { blocks: Vec::new(), color: Cow::Borrowed("success") }
  }

  /// Adds a block to the attachment.
  pub fn add_block(&mut self, block: impl Into<Block<'a>>) -> &mut Self {
    self.blocks.push(block.into());
    self
  }

  /// Sets the color of the attachment.
  pub fn set_color(&mut self, color: impl Into<Cow<'a, str>>) -> &mut Self {
    self.color = color.into();
    self
  }

  /// Adds a block to the attachment.
  pub fn with_block(mut self, block: impl Into<Block<'a>>) -> Self {
    self.add_block(block);
    self
  }

  /// Sets the color of the attachment.
  pub fn with_color(mut self, color: impl Into<Cow<'a, str>>) -> Self {
    self.set_color(color);
    self
  }
}

impl<'a> Default for Attachment<'a> {
  fn default() -> Self {
    Self::new()
  }
}

impl<'a> Serialize for Attachment<'a> {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: Serializer,
  {
    let color = match self.color.as_ref() {
      "good" | "success" | "ok" => "#73d216",
      "debug" => "#75507b",
      "info" | "default" | "" => "#3465a4",
      "warn" | "warning" => "#edd400",
      "error" | "err" => "#cc0000",
      other => other,
    };

    let mut s = serializer.serialize_struct("Attachment", 2)?;

    s.serialize_field("color", &color)?;
    s.serialize_field("blocks", &self.blocks)?;
    s.end()
  }
}
