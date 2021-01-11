// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

pub mod error;

pub use self::error::Error;
pub use native_tls::Error as TlsError;
pub use tokio_postgres::Config;

use af_core::prelude::*;
use af_core::task::Task;
use native_tls::TlsConnector;
use postgres_native_tls::MakeTlsConnector;

pub struct Client {
  inner: tokio_postgres::Client,
}

pub async fn connect(config: &Config) -> Result<(Client, impl Task<(), Error>), Error> {
  let tls_connector = TlsConnector::builder().danger_accept_invalid_certs(true).build().unwrap();
  let (client, connection) = config.connect(MakeTlsConnector::new(tls_connector)).await?;
  let task = async move { connection.await.map_err(Error::from) };

  Ok((Client { inner: client }, task))
}
