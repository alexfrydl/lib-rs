// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Create [Slack](https://slack.com) apps with
//! [af-lib](https://docs.rs/af-lib/0.1).

pub mod api;
pub mod block;
pub mod chat;

#[doc(inline)]
pub use {
  self::api::Client,
  self::chat::{permalink_to, post, reply, Attachment, Message, MessageId},
};
