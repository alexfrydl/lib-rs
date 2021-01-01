// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::prelude::*;
use crate::sync::blocking::RwLock;

use std::{collections::HashMap, ptr};

/// An interned string.
///
/// Symbols are expensive to create but cheap to compare and hash. Each symbol
/// with the same value refers to the same string in memory.
///
/// Symbols should not be created from arbitrary strings. The memory allocated
/// for each unique symbol is never freed.
#[derive(Clone, Copy, Default, Deref)]
pub struct Symbol(#[deref] &'static str);

/// A map of interned symbols.
static INTERNED: Lazy<RwLock<HashMap<String, Symbol>>> = Lazy::new(|| {
  let mut map = HashMap::with_capacity(1024);

  map.insert(default(), default());

  RwLock::new(map)
});

impl Symbol {
  /// Creates a new symbol with a given value.
  ///
  /// If a symbol with the same value has already been created, a copy of
  /// the existing symbol is returned instead.
  pub fn new(value: impl AsRef<str> + Into<String>) -> Self {
    if let Some(existing) = INTERNED.read().get(value.as_ref()) {
      return *existing;
    }

    let value = value.into();
    let boxed = value.clone().into_boxed_str();

    // It's possible another thread created this symbol since the `read()`
    // above, so use the entry API to insert it into the interned map to prevent
    // overwriting it.

    *INTERNED.write().entry(value).or_insert_with(|| Symbol(&*Box::leak(boxed)))
  }

  /// Returns a reference to the underlying string value.
  pub fn as_str(&self) -> &'static str {
    self.0
  }
}

// Implement various conversions.

impl<'a> From<&'a str> for Symbol {
  fn from(value: &'a str) -> Self {
    Self::new(value)
  }
}

impl<'a> From<Cow<'a, str>> for Symbol {
  fn from(value: Cow<'a, str>) -> Self {
    Self::new(value)
  }
}

impl<'a> From<&'a String> for Symbol {
  fn from(value: &'a String) -> Self {
    Self::new(value)
  }
}

impl From<String> for Symbol {
  fn from(value: String) -> Self {
    Self::new(value)
  }
}

impl<'a> From<&'a Symbol> for String {
  fn from(symbol: &'a Symbol) -> Self {
    symbol.as_str().into()
  }
}

impl From<Symbol> for String {
  fn from(symbol: Symbol) -> Self {
    symbol.as_str().into()
  }
}

impl From<Symbol> for Cow<'static, str> {
  fn from(symbol: Symbol) -> Self {
    symbol.as_str().into()
  }
}

// Implement `AsRef` to expose the string.

impl AsRef<str> for Symbol {
  fn as_ref(&self) -> &str {
    self.as_str()
  }
}

// Implement `PartialEq` and `Eq` by pointer comparison.

impl PartialEq for Symbol {
  fn eq(&self, other: &Self) -> bool {
    ptr::eq(self.as_str(), other.as_str())
  }
}

impl Eq for Symbol {}

// Implement `Hash` by hashing a pointer to the beginning of the string.

impl Hash for Symbol {
  fn hash<H: Hasher>(&self, state: &mut H) {
    state.write_usize(unsafe { mem::transmute(&self.0.as_bytes()[0]) })
  }
}

// Delegate `Debug` and `Display` to the underlying string value.

impl Debug for Symbol {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    Debug::fmt(self.as_str(), f)
  }
}

impl Display for Symbol {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    Display::fmt(self.as_str(), f)
  }
}

// Unit tests.

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new() {
    use std::ptr;

    let a = Symbol::new("hello");
    let b = Symbol::new("hello");
    let c = Symbol::new("world");

    assert_eq!(*a, "hello");
    assert_eq!(*c, "world");
    assert_eq!(a, b);
    assert_ne!(a, c);
    assert_ne!(b, c);
    assert!(ptr::eq(*a, *b), "Two symbols of equal value should be pointer-equal.");
    assert!(!ptr::eq(*a, *c), "Two symbols of unequal value should not be pointer-equal.");
    assert!(!ptr::eq(*b, *c), "Two symbols of unequal value should not be pointer-equal.");
  }
}
