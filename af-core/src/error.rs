pub use std::error::Error;
pub use thiserror::Error;

use crate::prelude::*;

/// An error representing a panic.
#[derive(Error, From)]
pub struct Panic {
  /// The panic value.
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
