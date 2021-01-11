// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::prelude::*;
use ::uuid::{Builder, Uuid as Inner, Variant, Version};

/// A universally-unique identifier.
#[derive(
  Clone, Copy, Default, Deserialize, Eq, From, FromStr, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
#[serde(transparent)]
pub struct Uuid(Inner);

impl Uuid {
  /// Returns a "nil" UUID.
  pub const fn nil() -> Self {
    Self(Inner::nil())
  }

  /// Returns `true` if the UUID is the "nil" value.
  pub fn is_nil(&self) -> bool {
    self.0.is_nil()
  }

  /// Returns a new, random UUID.
  pub fn new() -> Self {
    random()
  }

  /// Returns a slice containing the bytes of the UUID.
  pub fn as_bytes(&self) -> &[u8] {
    self.0.as_bytes()
  }

  /// Converts the UUID to a `u128`.
  pub fn to_u128(&self) -> u128 {
    self.0.as_u128()
  }
}

// Implement random generation.

impl Random for Uuid {
  fn random_with(rng: &mut random::Rng) -> Self {
    let mut bytes = [0u8; 16];

    rng.fill_bytes(&mut bytes);

    let uuid =
      Builder::from_bytes(bytes).set_variant(Variant::RFC4122).set_version(Version::Random).build();

    Self(uuid)
  }
}

// Implement formatting.

macro_rules! impl_fmt {
  ($ty:ident) => {
    impl fmt::$ty for Uuid {
      fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::$ty::fmt(&self.0, f)
      }
    }
  };
}

impl_fmt!(Debug);
impl_fmt!(Display);
impl_fmt!(LowerHex);
impl_fmt!(UpperHex);
