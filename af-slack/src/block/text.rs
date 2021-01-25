// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use af_core::prelude::*;

/// A text object.
#[derive(Debug, Serialize)]
pub struct Text<'a> {
  /// The kind of text object.
  #[serde(rename = "type")]
  pub kind: TextKind,
  /// The text value.
  pub text: Cow<'a, str>,
}

impl<'a> Text<'a> {
  /// Returns a mrkdwn text object.
  pub fn mrkdwn(text: impl Into<Cow<'a, str>>) -> Self {
    Self { kind: TextKind::Mrkdwn, text: text.into() }
  }

  /// Returns a plain text object.
  pub fn plain(text: impl Into<Cow<'a, str>>) -> Self {
    Self { kind: TextKind::Plain, text: text.into() }
  }
}

/// One of the possible kinds of text object.
#[derive(Debug, Serialize)]
pub enum TextKind {
  /// A mrkdown text object.
  #[serde(rename = "mrkdwn")]
  Mrkdwn,
  /// A plain text object.
  #[serde(rename = "plain_text")]
  Plain,
}
