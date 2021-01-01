// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Base64 encoding and decoding.

#[doc(inline)]
pub use ::base64::DecodeError;

use crate::prelude::*;

/// Decodes a Base64 string.
pub fn decode(input: &str) -> Result<Vec<u8>, DecodeError> {
  ::base64::decode_config(input, ::base64::STANDARD)
}

/// Decodes an URL-safe Base64 string.
pub fn decode_url_safe(input: &str) -> Result<Vec<u8>, DecodeError> {
  ::base64::decode_config(input, ::base64::URL_SAFE_NO_PAD)
}

/// Encodes a slice of bytes as a Base64 string.
pub fn encode(input: &[u8]) -> String {
  ::base64::encode_config(input, ::base64::STANDARD)
}

/// Encodes a slice of bytes as an URL-safe Base64 string.
pub fn encode_url_safe(input: &[u8]) -> String {
  ::base64::encode_config(input, ::base64::URL_SAFE_NO_PAD)
}
