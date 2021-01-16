pub use self::QueryOneError::NoRowsReturned;
pub use tokio_postgres::error::{DbError, ErrorPosition, Severity, SqlState};

use af_core::prelude::*;
use af_core::string::SharedString;

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

#[derive(Debug, Display, Error)]
pub enum Error {
  #[display(fmt = "{}", _0)]
  Db(Box<DbError>),
  #[display(fmt = "{}", _0)]
  Other(SharedString),
}

/// An error returned from a `query_one` function.
#[derive(Debug, Display, Error, From)]
pub enum QueryOneError {
  /// An error indiating a statement returned no rows.
  #[display(fmt = "No rows returned.")]
  NoRowsReturned,
  #[display(fmt = "{}", _0)]
  #[from]
  Other(Error),
}

impl Error {
  /// Creates a new [`Error::Other`] with the given message.
  pub fn new(message: impl Into<SharedString>) -> Self {
    Self::Other(message.into())
  }
}

impl From<tokio_postgres::Error> for Error {
  fn from(err: tokio_postgres::Error) -> Self {
    match err.code() {
      Some(_) => Error::Db(std::error::Error::downcast(err.into_source().unwrap()).unwrap()),
      None => Error::Other(err.to_string().into()),
    }
  }
}