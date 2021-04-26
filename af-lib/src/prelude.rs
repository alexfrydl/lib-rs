// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A “prelude” module containing common imports.

#[doc(hidden)]
pub use thiserror;
#[doc(no_inline)]
pub use {
  crate::async_test,
  crate::math::{FloatExt as _, Number},
  crate::util::failure::{self, fail, failure, Failure, Result},
  crate::util::log::{debug, error, info, trace, warn},
  crate::util::{default, defer, fmt, pin, pin_project, process, Lazy, Uuid},
  derive_more::{
    Add, AddAssign, AsMut, AsRef, Binary, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor,
    BitXorAssign, Constructor, Deref, DerefMut, Display, Div, DivAssign, From, FromStr, Index,
    IndexMut, Into, IntoIterator, LowerExp, LowerHex, Mul, MulAssign, Neg, Not, Octal, Pointer,
    Product, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign, Sum, TryInto,
    UpperExp, UpperHex,
  },
  serde::{Deserialize, Deserializer, Serialize, Serializer},
  std::borrow::{Borrow, BorrowMut, Cow},
  std::cell::{Cell, RefCell, UnsafeCell},
  std::cmp,
  std::convert::{TryFrom, TryInto},
  std::error::Error,
  std::fmt::{Debug, Display, Write as _},
  std::future::Future,
  std::hash::{Hash, Hasher},
  std::io::{BufRead as _, Read as _, Seek as _, Write as _},
  std::marker::PhantomData,
  std::mem,
  std::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Deref,
    DerefMut, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Not, Range, RangeBounds,
    RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive, Rem, RemAssign, Shl,
    ShlAssign, Shr, ShrAssign, Sub, SubAssign,
  },
  std::pin::Pin,
  std::rc::{Rc, Weak as RcWeak},
  std::str::FromStr,
  std::sync::{Arc, Weak as ArcWeak},
  thiserror::Error,
};

#[doc(hidden)]
/// Exports various helpers for macros in a way that works even if the crate
/// does not directly depend on af_lib.
///
/// By exporting them with crazy names, they also won't get in the way of
/// autocomplete and autoimport.
pub mod __af_lib_macro_helpers {
  pub use crate::concurrency::runtime::run as __runtime_run;
  pub use crate::concurrency::scope::run_sync as __run_scope_sync;
  pub use crate::main as __main;
  pub use crate::util::fmt::indent as __fmt_indent;
  pub use crate::util::log::{
    flush as __flush_log, init as __log_init, set_level_of as __log_set_level_of,
    Level as __log_level,
  };
  pub use crate::util::process::set_exit_code as __set_exit_code;
}
