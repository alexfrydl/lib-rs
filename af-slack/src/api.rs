// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A client for the Slack API.

use af_core::prelude::*;

/// The URL of the Slack API.
const URL: &str = "https://slack.com/api/";

/// A Slack API client.
pub struct Client {
  authorization: String,
  http: reqwest::Client,
}

impl Client {
  /// Creates a new client with a given OAuth token.
  pub fn new(token: impl AsRef<str>) -> Self {
    Self { authorization: format!("Bearer {}", token.as_ref()), http: default() }
  }

  /// Sends a `POST` request.
  pub async fn get<Q, O>(&self, method: &str, query: &Q) -> Result<Result<O, ErrorResponse>>
  where
    Q: Serialize,
    O: for<'a> Deserialize<'a>,
  {
    let req = self
      .http
      .get(&format!("{}{}?{}", URL, method, serde_qs::to_string(query).unwrap()))
      .header("Authorization", &self.authorization);

    let mut res: ResponseProps = req.send().await?.json().await?;
    let ok = res.remove("ok").and_then(|v| v.as_bool()).unwrap_or_default();

    Ok(match ok {
      true => Ok(json::from_value(json::Value::Object(res))?),
      false => Err(json::from_value(json::Value::Object(res))?),
    })
  }

  /// Sends a `POST` request.
  pub async fn post<I, O>(&self, method: &str, body: &I) -> Result<Result<O, ErrorResponse>>
  where
    I: Serialize,
    O: for<'a> Deserialize<'a>,
  {
    let req = self
      .http
      .post(&format!("{}{}", URL, method))
      .header("Authorization", &self.authorization)
      .json(body);

    let mut res: ResponseProps = req.send().await?.json().await?;
    let ok = res.remove("ok").and_then(|v| v.as_bool()).unwrap_or_default();

    Ok(match ok {
      true => Ok(json::from_value(json::Value::Object(res))?),
      false => Err(json::from_value(json::Value::Object(res))?),
    })
  }
}

/// A Slack API error.
#[derive(Debug, Error)]
pub enum Error {
  /// The response body could not be deserialized.
  #[error("invalid response body: {0}")]
  InvalidResponse(#[from] json::Error),
  /// The HTTP request failed.
  #[error(transparent)]
  RequestFailed(#[from] reqwest::Error),
}

/// The result of a Slack API call.
pub type Result<T = (), E = Error> = std::result::Result<T, E>;

/// Properties on a Slack API response.
pub type ResponseProps = json::Map<String, json::Value>;

/// An Slack API error response.
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse {
  /// A machine-readable string indicating what kind of error occurred.
  pub error: String,
  /// The remaining properties of the error.
  #[serde(flatten)]
  pub props: ResponseProps,
}

impl Display for ErrorResponse {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let json = match f.alternate() {
      true => json::to_string_pretty(self).unwrap(),
      false => json::to_string(self).unwrap(),
    };

    Display::fmt(&json, f)
  }
}
