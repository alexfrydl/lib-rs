// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::prelude::*;

/// A cloneable reference to a shared string.
#[derive(Clone)]
pub struct SharedStr(Inner);

#[derive(Clone)]
enum Inner {
  ArcStr(Arc<str>),
  StaticStr(&'static str),
}

impl SharedStr {
  /// Returns a reference to this string as a `&str`.
  pub fn as_str(&self) -> &str {
    match &self.0 {
      Inner::ArcStr(value) => value.as_ref(),
      Inner::StaticStr(value) => value,
    }
  }
}

impl AsRef<str> for SharedStr {
  fn as_ref(&self) -> &str {
    self.as_str()
  }
}

impl Display for SharedStr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    Display::fmt(self.as_str(), f)
  }
}

impl Debug for SharedStr {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    Debug::fmt(self.as_str(), f)
  }
}

impl Default for SharedStr {
  fn default() -> Self {
    Self(Inner::StaticStr(""))
  }
}

impl Deref for SharedStr {
  type Target = str;

  fn deref(&self) -> &Self::Target {
    self.as_str()
  }
}

impl<'de> Deserialize<'de> for SharedStr {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: Deserializer<'de>,
  {
    String::deserialize(deserializer).map(SharedStr::from)
  }
}

impl From<&'static str> for SharedStr {
  fn from(value: &'static str) -> Self {
    Self(Inner::StaticStr(value))
  }
}

impl From<Arc<str>> for SharedStr {
  fn from(value: Arc<str>) -> Self {
    Self(Inner::ArcStr(value))
  }
}

impl From<String> for SharedStr {
  fn from(value: String) -> Self {
    Self(Inner::ArcStr(value.into()))
  }
}

impl From<Cow<'static, str>> for SharedStr {
  fn from(value: Cow<'static, str>) -> Self {
    match value {
      Cow::Borrowed(value) => Self(Inner::StaticStr(value)),
      Cow::Owned(value) => Self(Inner::ArcStr(value.into())),
    }
  }
}

impl Eq for SharedStr {}

impl Hash for SharedStr {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.as_str().hash(state)
  }
}

impl Ord for SharedStr {
  fn cmp(&self, other: &Self) -> cmp::Ordering {
    self.as_str().cmp(other.as_str())
  }
}

impl PartialEq for SharedStr {
  fn eq(&self, other: &Self) -> bool {
    self.as_str() == other.as_str()
  }
}

impl PartialOrd for SharedStr {
  fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
    self.as_str().partial_cmp(other.as_str())
  }
}

impl Serialize for SharedStr {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    self.as_str().serialize(serializer)
  }
}
