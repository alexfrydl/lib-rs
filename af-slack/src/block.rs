// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Content blocks for messages and surfaces.

mod text;

pub use self::text::Text;

use af_core::prelude::*;

/// Returns a Header block.
pub fn header<'a>(text: impl Into<Cow<'a, str>>) -> Block<'a> {
  Block::Header { text: Text::plain(text) }
}

/// Returns a Section block.
pub fn section<'a>(text: impl Into<Cow<'a, str>>) -> Block<'a> {
  Block::Section { text: Text::mrkdwn(text) }
}

/// A content block.
#[derive(Debug, Serialize)]
#[serde(rename_all = "snake_case", tag = "type")]
pub enum Block<'a> {
  Header { text: Text<'a> },
  Section { text: Text<'a> },
}

impl<'a, T> From<T> for Block<'a>
where
  Cow<'a, str>: From<T>,
{
  fn from(value: T) -> Self {
    section(value)
  }
}
