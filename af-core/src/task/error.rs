// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::prelude::*;

/// The result of a task.
pub type Result<T = (), E = Panic> = std::result::Result<T, E>;

/// A task error.
#[derive(Debug, Display, From)]
pub enum Error<E> {
  /// An error returned from the task.
  #[display(fmt = "{}", _0)]
  Err(E),
  /// A panic.
  #[display(fmt = "{}", _0)]
  #[from]
  Panic(Panic),
}

impl<E> std::error::Error for Error<E> where E: Debug + Display {}

/// An error indicating a task panicked.
#[derive(Error, From)]
pub struct Panic {
  /// The value the task panicked with.
  pub value: Box<dyn Any + Send>,
}

impl Panic {
  /// Returns a reference to the panic value if it is a string.
  pub fn value_str(&self) -> Option<&str> {
    if let Some(string) = self.value.downcast_ref::<String>() {
      Some(string)
    } else if let Some(string) = self.value.downcast_ref::<&'static str>() {
      Some(string)
    } else {
      None
    }
  }
}

impl Debug for Panic {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "PanicError")?;

    if let Some(value) = self.value_str() {
      write!(f, "({:?})", value)?;
    }

    Ok(())
  }
}

impl Display for Panic {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Panicked")?;

    if let Some(value) = self.value_str() {
      write!(f, " with `{}`", value)?;
    }

    write!(f, ".")
  }
}

/// An extension trait for `Result<Result<T, E>, Panic>`.
pub trait ResultResultExt<T, E> {
  /// Flattens this nested result by converting the errors to the same type.
  fn flatten_err(self) -> Result<T, Error<E>>;
}

impl<T, E> ResultResultExt<T, E> for Result<Result<T, E>> {
  fn flatten_err(self) -> Result<T, Error<E>> {
    match self {
      Ok(Ok(value)) => Ok(value),
      Ok(Err(err)) => Err(Error::Err(err)),
      Err(panic) => Err(panic.into()),
    }
  }
}
