pub use tokio_postgres::error::{DbError, ErrorPosition, Severity, SqlState};

use af_core::prelude::*;
use af_core::string::SharedString;

#[derive(Debug, Display, Error)]
pub enum Error {
  #[display(fmt = "{}", _0)]
  Db(Box<DbError>),
  #[display(fmt = "{}", _0)]
  Other(SharedString),
}

impl From<tokio_postgres::Error> for Error {
  fn from(err: tokio_postgres::Error) -> Self {
    match err.code() {
      Some(_) => Error::Db(std::error::Error::downcast(err.into_source().unwrap()).unwrap()),
      None => Error::Other(err.to_string().into()),
    }
  }
}
