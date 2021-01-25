// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Post and interact with Slack messages.

mod attachment;
mod message;

pub use self::attachment::Attachment;
pub use self::message::{Message, MessageId};

use crate::api;
use af_core::prelude::*;

/// Gets a permalink to a message.
pub async fn permalink_to<'a>(client: &api::Client, id: &MessageId) -> Result<String> {
  #[derive(Serialize)]
  struct Params<'a> {
    channel: &'a str,
    message_ts: &'a str,
  }

  #[derive(Deserialize)]
  struct Response {
    permalink: String,
  }

  let params = Params { channel: &id.channel, message_ts: &id.ts };
  let res = client.get::<_, Response>("chat.getPermalink", &params).await?;

  match res {
    Ok(res) => Ok(res.permalink),
    Err(err) => match err.error.as_str() {
      "channel_not_found" => Err(Error::ChannelNotFound),
      _ => Err(Error::Other(err)),
    },
  }
}

/// Posts a message to a channel.
pub async fn post<'a>(
  client: &api::Client,
  channel: impl AsRef<str>,
  message: impl Into<Message<'a>>,
) -> Result<MessageId> {
  #[derive(Serialize)]
  struct Request<'a> {
    channel: &'a str,
    #[serde(flatten)]
    message: Message<'a>,
  }

  let req = Request { channel: channel.as_ref(), message: message.into() };
  let res = client.post::<_, MessageId>("chat.postMessage", &req).await?;

  match res {
    Ok(id) => Ok(id),
    Err(err) => match err.error.as_str() {
      "channel_not_found" => Err(Error::ChannelNotFound),
      "not_in_channel" => Err(Error::NotInChannel),
      _ => Err(Error::Other(err)),
    },
  }
}

/// Replies to an existing message thread in a channel.
pub async fn reply<'a>(
  client: &api::Client,
  thread_id: &MessageId,
  message: impl Into<Message<'a>>,
) -> Result<MessageId> {
  #[derive(Serialize)]
  struct Request<'a> {
    channel: &'a str,
    #[serde(flatten)]
    message: Message<'a>,
    thread_ts: &'a str,
  }

  let req =
    Request { channel: &thread_id.channel, message: message.into(), thread_ts: &thread_id.ts };
  let res = client.post::<_, MessageId>("chat.postMessage", &req).await?;

  match res {
    Ok(id) => Ok(id),
    Err(err) => match err.error.as_str() {
      "channel_not_found" => Err(Error::ChannelNotFound),
      "not_in_channel" => Err(Error::NotInChannel),
      _ => Err(Error::Other(err)),
    },
  }
}

/// A chat error.
#[derive(Debug, Error)]
pub enum Error {
  /// The given channel was not found.
  #[error("channel not found")]
  ChannelNotFound,
  /// The app is not in the given channel.
  #[error("not in channel")]
  NotInChannel,
  /// A low-level API error occurred.
  #[error("api error: {0}")]
  ApiError(#[from] api::Error),
  /// Some other response error occurred.
  #[error("{0:#}")]
  Other(api::ErrorResponse),
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;
