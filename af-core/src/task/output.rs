use super::*;

/// The output of a task.
pub type Output<T, E> = Result<T, Failure<E>>;

/// Waits for a task to complete and captures its output.
pub async fn capture<T, E>(task: impl Task<T, E>) -> Output<T, E>
where
  T: Send + 'static,
  E: Send + 'static,
{
  future::catch_unwind(panic::AssertUnwindSafe(task))
    .await
    .map_err(|value| Failure::Panic(Panicked { value }))
    .and_then(|res| res.map_err(Failure::Err))
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
