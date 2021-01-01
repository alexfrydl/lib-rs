// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub use bytes::BytesMut;
pub use tokio_postgres::binary_copy;
pub use tokio_postgres::error::Error;
pub use tokio_postgres::types::{FromSql, IsNull, ToSql, Type};
pub use tokio_postgres::{Client as Connection, Config, RowStream, Transaction};

use crate::prelude::*;
use crate::runtime::task;
use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;

/// The type of a query parameter.
pub type Param<'a> = &'a (dyn ToSql + Sync);

pub type FromSqlResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;
pub type ToSqlResult = Result<IsNull, Box<dyn std::error::Error + Send + Sync>>;

/// A generic postgres client, either a connection or a transaction.
pub trait Client: tokio_postgres::GenericClient + Send + Sync {}

impl<T> Client for T where T: tokio_postgres::GenericClient + Send + Sync {}

/// Returns a connection pool for a database.
pub async fn connect(config: &Config) -> Result<Connection> {
  let tls_connector = TlsConnector::builder()
    .danger_accept_invalid_certs(true)
    .build()
    .map_err(|err| fail::err!("Failed to create TLS connector. {}", err))?;

  let (client, connection) = config.connect(MakeTlsConnector::new(tls_connector)).await?;

  task::start_detached(async move {
    if let Err(err) = connection.await {
      error!("Postgres connection error — {}.", err);
    }
  });

  Ok(client)
}
