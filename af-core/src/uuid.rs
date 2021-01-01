// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::prelude::*;
use ::uuid::{Builder, Uuid as Inner, Variant, Version};

/// A universally-unique identifier.
#[derive(
  Clone, Copy, Default, Deserialize, Eq, FromStr, Hash, Ord, PartialEq, PartialOrd, Serialize,
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

// Implement postgres support.

#[cfg(feature = "postgres")]
use crate::postgres as pg;

#[cfg(feature = "postgres")]
impl<'a> pg::FromSql<'a> for Uuid {
  fn from_sql(ty: &pg::Type, raw: &'a [u8]) -> pg::FromSqlResult<Self> {
    Inner::from_sql(ty, raw).map(Self)
  }

  fn accepts(ty: &pg::Type) -> bool {
    <Inner as pg::FromSql>::accepts(ty)
  }
}

#[cfg(feature = "postgres")]
impl pg::ToSql for Uuid {
  fn to_sql(&self, ty: &pg::Type, out: &mut pg::BytesMut) -> pg::ToSqlResult
  where
    Self: Sized,
  {
    self.0.to_sql(ty, out)
  }

  fn accepts(ty: &pg::Type) -> bool
  where
    Self: Sized,
  {
    <Inner as pg::ToSql>::accepts(ty)
  }

  fn to_sql_checked(&self, ty: &pg::Type, out: &mut pg::BytesMut) -> pg::ToSqlResult {
    self.0.to_sql_checked(ty, out)
  }
}
