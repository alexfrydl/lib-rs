// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use super::Attachment;
use af_core::prelude::*;

/// A unique identifier for a message.
#[derive(Debug, Deserialize)]
pub struct MessageId {
  /// The ID of the channel.
  pub channel: String,
  /// The timestamp of the message.
  pub ts: String,
}

/// A chat message.
#[derive(Debug, Serialize)]
pub struct Message<'a> {
  pub attachments: Vec<Attachment<'a>>,
  pub text: Cow<'a, str>,
}

impl<'a> Message<'a> {
  /// Creates a new, empty chat message.
  pub const fn new() -> Self {
    Self { attachments: Vec::new(), text: Cow::Borrowed("") }
  }

  /// Adds an attachment to the message.
  pub fn add_attachment(&mut self, attachment: Attachment<'a>) -> &mut Self {
    self.attachments.push(attachment);
    self
  }

  /// Sets the text of the message.
  pub fn set_text(&mut self, text: impl Into<Cow<'a, str>>) -> &mut Self {
    self.text = text.into();
    self
  }

  /// Adds an attachment to the message.
  pub fn with_attachment(mut self, attachment: Attachment<'a>) -> Self {
    self.attachments.push(attachment);
    self
  }

  /// Sets the text of the message.
  pub fn with_text(mut self, text: impl Into<Cow<'a, str>>) -> Self {
    self.set_text(text);
    self
  }
}

impl<'a> Default for Message<'a> {
  fn default() -> Self {
    Self::new()
  }
}

impl<'a, T> From<T> for Message<'a>
where
  Cow<'a, str>: From<T>,
{
  fn from(text: T) -> Message<'a> {
    let mut msg = Message::new();

    msg.set_text(text);
    msg
  }
}

impl<'a> From<Attachment<'a>> for Message<'a> {
  fn from(attachment: Attachment<'a>) -> Self {
    let mut message = Self::new();

    message.attachments.push(attachment);
    message
  }
}
