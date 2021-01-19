// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::prelude::*;
use crate::task;

/// The result of a task.
pub type Result<T = (), E = Panicked> = std::result::Result<T, E>;

/// Waits for a task to complete and captures its output.
pub async fn capture<T>(task: impl task::Future<T>) -> Result<T>
where
  T: Send + 'static,
{
  future::catch_unwind(panic::AssertUnwindSafe(task)).await.map_err(|value| Panicked { value })
}

/// A task failure.
#[derive(Debug, Display)]
pub enum Failure<E> {
  /// The task failed because it returned an [`Err`].
  #[display(fmt = "{}", _0)]
  Err(E),
  /// The task failed because it panicked.
  #[display(fmt = "{}", _0)]
  Panic(Panicked),
}

impl<E> Error for Failure<E> where E: Debug + Display {}

impl<E> Failure<E> {
  /// Converts this failure into an error.
  pub fn into_err(self) -> E
  where
    E: From<Panicked>,
  {
    match self {
      Self::Err(err) => err,
      Self::Panic(panic) => panic.into(),
    }
  }

  /// Convert the [`Err`][Self::Err] value if it exists.
  pub fn map_err<F>(self, map: impl FnOnce(E) -> F) -> Failure<F> {
    match self {
      Self::Err(err) => Failure::Err(map(err)),
      Self::Panic(panic) => Failure::Panic(panic),
    }
  }
}

/// An error indicating a task panicked.
#[derive(Error, From)]
pub struct Panicked {
  /// The value the task panicked with.
  pub value: Box<dyn Any + Send>,
}

// Implement formatting for Panicked.

impl Panicked {
  /// Returns a `&dyn Debug` of the panic value if possible.
  pub fn debug_value(&self) -> Option<&dyn Debug> {
    if let Some(string) = self.value.downcast_ref::<String>() {
      Some(string)
    } else if let Some(string) = self.value.downcast_ref::<&'static str>() {
      Some(string)
    } else {
      None
    }
  }

  /// Returns a `&dyn Display` of the panic value if possible.
  pub fn display_value(&self) -> Option<&dyn Display> {
    if let Some(string) = self.value.downcast_ref::<String>() {
      Some(string)
    } else if let Some(string) = self.value.downcast_ref::<&'static str>() {
      Some(string)
    } else {
      None
    }
  }
}

impl Debug for Panicked {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "PanicError")?;

    if let Some(debug) = self.debug_value() {
      write!(f, "({:?})", debug)?;
    }

    Ok(())
  }
}

impl Display for Panicked {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Panicked")?;

    if let Some(display) = self.display_value() {
      write!(f, " with `{}`", display)?;
    }

    write!(f, ".")
  }
}
