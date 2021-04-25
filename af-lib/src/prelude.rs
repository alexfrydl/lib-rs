// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! A “prelude” module containing common imports.

pub use std::any::Any;
pub use std::borrow::*;
pub use std::cell::{self, Cell, RefCell};
pub use std::cmp::{self, Eq, Ord, PartialEq, PartialOrd};
pub use std::convert::{Infallible, TryFrom, TryInto};
pub use std::error::Error;
pub use std::hash::{self, Hash, Hasher};
pub use std::io::{BufRead as _, Read as _, Seek as _, Write as _};
pub use std::marker::PhantomData;
pub use std::mem;
pub use std::ops::*;
pub use std::pin::Pin;
pub use std::ptr;
pub use std::rc::{Rc, Weak as RcWeak};
pub use std::str::{self, FromStr};
pub use std::sync::{Arc, Weak as ArcWeak};
pub use std::{char, slice};
pub use std::{f32, f64};
pub use std::{i128, i16, i32, i64, i8, isize};
pub use std::{u128, u16, u32, u64, u8, usize};

pub(crate) use cfg_if::cfg_if;

pub use crate::log::{debug, error, info, trace, warn};
pub use crate::math::{FloatExt as _, Number};
pub use crate::time::{self, Date, Duration, Time};
pub use crate::util::derive::*;
pub use crate::util::failure::{self, fail, failure, Failure, Result};
pub use crate::util::fmt::{self, Debug, Display, Write as _};
pub use crate::util::future::{self, Future};
pub use crate::util::iter::{self, Itertools as _};
pub use crate::util::lazy::{self, Lazy};
pub use crate::util::panic::{self, Panic};
pub use crate::util::random::{self, random, Random};
pub use crate::util::Uuid;
pub use crate::util::{default, pin, pin_project};
