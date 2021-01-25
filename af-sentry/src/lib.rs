// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! [Sentry](https://sentry.io) integration for
//! [af-lib](https://docs.rs/af-lib/0.1).

pub use sentry::{ClientInitGuard, ClientOptions, IntoDsn, User};

use af_core::prelude::*;
use std::collections::BTreeMap;

/// The type of the [`Error::fingerprint`] field.
pub type Fingerprint = Cow<'static, [Cow<'static, str>]>;

/// Initializes Sentry with the given DSN or [`ClientOptions`].
pub fn init(options: impl Into<ClientOptions>) -> ClientInitGuard {
  let options = options.into();

  sentry::init(options)
}

/// Creates a sentry error.
#[macro_export]
macro_rules! error {
  ($type_name:expr, $format:literal, $($args:tt)+) => {
    $crate::Error::new($type_name).with_description(format_args!($format, $($args)+))
  };

  ($type_name:expr, $($args:tt)+) => {
    $crate::Error::new($type_name).with_description($($args)+)
  };

  ($($args:tt)+) => {
    $crate::Error::new($($args)*)
  };
}

/// Returns `true` if error reporting is enabled.
///
/// Error reporting is enabled if [`init()`] has been called with a valid DSN.
pub fn is_enabled() -> bool {
  sentry::Hub::with(|hub| hub.client().map(|c| c.is_enabled()).unwrap_or_default())
}

/// A sentry error.
///
/// Errors are automatically reported when dropped.
#[derive(Debug)]
pub struct Error {
  /// A short description of the error.
  pub description: String,
  /// A detailed description of the error.
  pub detail: String,
  /// The fingerprint of the error.
  ///
  /// Errors with the same fingerprint are grouped together. The default groups
  /// by [`type`] and [`ClientOptions::environment`].
  pub fingerprint: Fingerprint,
  /// The type of the error.
  pub type_name: String,
  /// Additional tags to apply to the error.
  pub tags: BTreeMap<String, String>,
  /// User data to send with the error.
  pub user: User,
  /// The UUID of the event.
  pub uuid: Uuid,
}

const DEFAULT_FINGERPRINT: Fingerprint =
  Cow::Borrowed(&[Cow::Borrowed("{{ type }}"), Cow::Borrowed("{{ tags.environment }}")]);

impl Error {
  /// Creates a new error with the given type.
  pub fn new(type_name: impl Into<String>) -> Self {
    Self {
      description: default(),
      detail: default(),
      fingerprint: DEFAULT_FINGERPRINT,
      type_name: type_name.into(),
      tags: default(),
      user: default(),
      uuid: Uuid::new(),
    }
  }

  /// Sets the short description of the error.
  pub fn set_description(&mut self, description: impl ToString) {
    let mut description = description.to_string();

    if self.detail.is_empty() {
      self.detail = description.clone();
    }

    if let Some(i) = description.find('\n') {
      description.truncate(i);
      description.truncate(description.trim_end().len());
    }

    if description.ends_with(':') {
      description.pop();
      description.push('.');
    }

    if description.len() > 256 {
      description.truncate(255);
      description.push('…');
    }

    self.description = description;
  }

  /// Sets the detailed description of the error.
  pub fn set_detail(&mut self, detail: impl ToString) {
    self.detail = detail.to_string();
  }

  /// Adds extra tagged information.
  pub fn set_tag(&mut self, name: impl Into<String>, value: impl ToString) {
    self.tags.insert(name.into(), value.to_string());
  }

  /// Sets the short description of the error.
  pub fn with_description(mut self, description: impl ToString) -> Self {
    self.set_description(description);
    self
  }

  /// Sets the detailed description of the error.
  pub fn with_detail(mut self, detail: impl ToString) -> Self {
    self.set_detail(detail);
    self
  }

  /// Sets the fingerprint used to group the error.
  pub fn with_fingerprint(mut self, fingerprint: Fingerprint) -> Self {
    self.fingerprint = fingerprint;
    self
  }

  /// Adds extra tagged information.
  pub fn with_tag(mut self, name: impl Into<String>, value: impl ToString) -> Self {
    self.set_tag(name, value);
    self
  }

  /// Adds user information.
  pub fn with_user(mut self, user: User) -> Self {
    self.user = user;
    self
  }

  /// Adds user ID information.
  pub fn with_user_id(mut self, id: impl ToString) -> Self {
    self.user.id = Some(id.to_string());
    self
  }

  /// Reports this error to sentry.
  ///
  /// Equivalent to dropping the error, but returns the error [`Uuid`].
  pub fn report(mut self) -> Uuid {
    let uuid = self.report_mut();
    mem::forget(self);
    uuid
  }

  /// Reports this error to sentry.
  fn report_mut(&mut self) -> Uuid {
    let mut event = sentry::protocol::Event::new();

    if !self.detail.is_empty() {
      event.message = Some(mem::take(&mut self.detail));
    }

    event.exception.values.push(sentry::protocol::Exception {
      ty: mem::take(&mut self.type_name),
      value: Some(mem::take(&mut self.description)),
      ..default()
    });

    event.fingerprint = mem::replace(&mut self.fingerprint, DEFAULT_FINGERPRINT);

    mem::swap(&mut event.tags, &mut self.tags);

    if let Some(env) = event.tags.remove("environment") {
      event.environment = Some(env.into());
    }

    event.user = Some(mem::take(&mut self.user));

    sentry::capture_event(event).into()
  }
}

impl Drop for Error {
  fn drop(&mut self) {
    self.report_mut();
  }
}
