// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use sentry::{ClientInitGuard, ClientOptions, IntoDsn};

use af_core::prelude::*;
use std::collections::BTreeMap;

/// The type of the [`Error::fingerprint`] field.
pub type Fingerprint<'a> = Cow<'a, [Cow<'a, str>]>;

/// Initializes Sentry with the given DSN or [`ClientOptions`].
pub fn init(options: impl Into<ClientOptions>) -> ClientInitGuard {
  let options = options.into();

  sentry::init(options)
}

/// Returns `true` if error reporting is enabled.
///
/// Error reporting is enabled if [`init()`] has been called with a valid DSN.
pub fn is_enabled() -> bool {
  sentry::Hub::with(|hub| hub.client().map(|c| c.is_enabled()).unwrap_or_default())
}

/// Reports an error to sentry with the given name.
pub fn report(name: impl Into<String>, err: impl Display) -> Uuid {
  Error::from_err(name, err).report()
}

/// An error to be captured by sentry.
#[derive(Debug)]
pub struct Error<'a> {
  /// A short description of the error.
  pub description: String,
  /// A detailed description of the error.
  pub detail: String,
  /// The fingerprint of the error.
  ///
  /// Errors with the same fingerprint are grouped together. The default groups
  /// by [`name`] and [`ClientOptions::environment`].
  pub fingerprint: Fingerprint<'a>,
  /// The name of the error.
  pub name: String,
  /// Additional tags to apply to the error.
  pub tags: BTreeMap<String, String>,
}

impl<'a> Default for Error<'a> {
  fn default() -> Self {
    Error {
      description: default(),
      detail: default(),
      fingerprint: Cow::Borrowed(&[
        Cow::Borrowed("{{ type }}"),
        Cow::Borrowed("{{ tags.environment }}"),
      ]),
      name: default(),
      tags: default(),
    }
  }
}

impl<'a> Error<'a> {
  /// Creates a new error with the given name.
  pub fn new(name: impl Into<String>) -> Self {
    Self { name: name.into(), ..default() }
  }

  /// Creates a new sentry error with the given name from an error.
  pub fn from_err(name: impl Into<String>, err: impl Display) -> Self {
    let mut description = format!("{}", err);
    let detail = format!("{:#}", err);

    if description.len() > 256 {
      description.truncate(255);
      description.push('…');
    }

    Self { name: name.into(), description, detail, ..default() }
  }

  /// Adds extra tagged information.
  pub fn tag(&mut self, name: impl Into<String>, value: impl ToString) {
    self.tags.insert(name.into(), value.to_string());
  }

  /// Reports this error to sentry.
  pub fn report(self) -> Uuid {
    let mut event = sentry::protocol::Event::new();

    if !self.detail.is_empty() {
      event.message = Some(self.detail);
    }

    event.exception.values.push(sentry::protocol::Exception {
      ty: self.name,
      value: Some(self.description),
      ..default()
    });

    event.tags = self.tags;

    sentry::capture_event(event).into()
  }
}
