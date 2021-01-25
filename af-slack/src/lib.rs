// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Slack integration for af-core.

pub mod api;
pub mod block;
pub mod chat;

pub use self::api::Client;
pub use self::chat::{permalink_to, post, reply, Attachment, Message, MessageId};
