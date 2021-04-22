// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A “prelude” module containing common imports.

#[doc(hidden)]
pub use crate as af_core;

#[doc(no_inline)]
pub use {
  crate::derive::*,
  crate::error::{Error, Panic},
  crate::failure::{self, fail, failure, Failure, Result},
  crate::fmt::{self, Debug, Display, InColorExt as _, Write as _},
  crate::future::{self, Future},
  crate::iter::{self, Itertools as _},
  crate::json,
  crate::lazy::{self, Lazy},
  crate::log::{debug, error, info, trace, warn},
  crate::math::{FloatExt as _, Number},
  crate::random::{self, random, Random},
  crate::time::{self, Date, Duration, Time},
  crate::util::Uuid,
  crate::util::{cfg_if, default, pin, pin_project},
  std::any::Any,
  std::borrow::*,
  std::cell::{self, Cell, RefCell},
  std::cmp::{self, Eq, Ord, PartialEq, PartialOrd},
  std::convert::{Infallible, TryFrom, TryInto},
  std::hash::{self, Hash, Hasher},
  std::io::{BufRead as _, Read as _, Seek as _, Write as _},
  std::marker::PhantomData,
  std::mem::{self, ManuallyDrop},
  std::ops::*,
  std::pin::Pin,
  std::ptr,
  std::rc::{Rc, Weak as RcWeak},
  std::str::{self, FromStr},
  std::sync::{Arc, Weak as ArcWeak},
  std::{char, panic, slice},
  std::{f32, f64},
  std::{i128, i16, i32, i64, i8, isize},
  std::{u128, u16, u32, u64, u8, usize},
};
