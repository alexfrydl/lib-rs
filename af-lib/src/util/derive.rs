// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! Miscellaneous derive macros.

#[doc(no_inline)]
pub use derive_more::{
  Add, AddAssign, AsMut, AsRef, Binary, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor,
  BitXorAssign, Constructor, Deref, DerefMut, Display, Div, DivAssign, From, FromStr, Index,
  IndexMut, Into, IntoIterator, LowerExp, LowerHex, Mul, MulAssign, Neg, Not, Octal, Pointer,
  Product, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign, Sum, TryInto, UpperExp,
  UpperHex,
};
#[doc(no_inline)]
pub use serde::{Deserialize, Deserializer, Serialize, Serializer};
#[doc(hidden)]
pub use thiserror;
#[doc(no_inline)]
pub use thiserror::Error;
