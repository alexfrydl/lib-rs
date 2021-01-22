// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::error::{Error, NoRowsReturned, QueryOneError, Result};
use crate::{Config, Row, RowStream, Statement, ToSql, ToStatement};
use af_core::prelude::*;
use af_core::task::{self, Task};

/// A client for executing stataments on a Postgres connection.
///
/// The client can be cloned and shared across tasks or threads. All clones
/// share the same connection which executes statements in the order they are
/// received. Statements are sent to Postgres with automatic pipelining to
/// maximize throughput when using multiple clients simultaneously.
#[derive(Clone)]
pub struct Client {
  inner: Arc<tokio_postgres::Client>,
}

/// Connects to a Postgres database using the given config.
///
/// This function returns a tuple with a [`Client`] and a connection task. The
/// task must be polled (for example, by adding it to a
/// [`Batch`][af_core::task::Batch]) to communicate with the server and report
/// errors. The client can be cloned and shared between tasks.
pub async fn connect(config: &Config) -> Result<(Client, Task<Result>)> {
  use native_tls::TlsConnector;
  use postgres_native_tls::MakeTlsConnector;

  let tls_connector = TlsConnector::builder().danger_accept_invalid_certs(true).build().unwrap();
  let (client, connection) = config.connect(MakeTlsConnector::new(tls_connector)).await?;
  let task = task::start(connection.map_err(Error::from));

  Ok((Client::wrap(client), task))
}

/// Iterates over a slice of [`ToSql`] values, for providing statement params.
///
/// This is needed because Rust cannot infer types from this expression.
fn param_iter<'a>(
  p: &'a [&'a (dyn ToSql + Sync)],
) -> impl ExactSizeIterator<Item = &'a dyn ToSql> + 'a {
  p.iter().map(|p| *p as _)
}

impl Client {
  /// Wraps a [`tokio_postgres::Client`].
  pub(crate) fn wrap(client: tokio_postgres::Client) -> Self {
    Self { inner: Arc::new(client) }
  }

  /// Executes a statement and returns the number of rows affected.
  pub async fn execute(
    &self,
    statement: &impl ToStatement,
    params: &[&(dyn ToSql + Sync)],
  ) -> Result<u64> {
    Ok(self.inner.execute_raw(statement, param_iter(params)).await?)
  }

  /// Returns `true` if the client is disconnected.
  pub fn is_closed(&self) -> bool {
    self.inner.is_closed()
  }

  /// Executes a batch of statements separated by semicolons.
  ///
  /// If a statement fails, this function will return the error and stop
  /// executing the batch. When executing a batch of statements including
  /// transactions, it is the user's responsibility to roll back any running
  /// transactions if this function fails.
  ///
  /// This function is intended to execute, for example, a SQL file to create
  /// the initial schema of a database. When executing individual statements,
  /// other functions should be preferred.
  pub async fn batch_execute(&mut self, statements: impl AsRef<str>) -> Result {
    Ok(self.inner.batch_execute(statements.as_ref()).await?)
  }

  /// Begins a transaction on the associated connection.
  ///
  /// All clones of this client share the same transaction. They will execute
  /// statements within the transaction until this client or a clone of it
  /// commits or rolls back the transaction. It is the user's responsibilty to
  /// ensure consistency when creating a transaction using multiple clones
  /// concurrently.
  pub async fn begin(&mut self) -> Result {
    Ok(self.inner.batch_execute("BEGIN;").await?)
  }

  /// Commits the running transaction.
  pub async fn commit(&mut self) -> Result {
    Ok(self.inner.batch_execute("COMMIT;").await?)
  }

  /// Prepares a statement.
  ///
  /// Prepared statements can be used repeatedly, but only by the same
  /// [`Client`] that created them.
  pub async fn prepare(&self, query: impl AsRef<str>) -> Result<Statement> {
    Ok(self.inner.prepare(query.as_ref()).await?)
  }

  /// Executes a statement and returns its results as a stream of rows.
  pub async fn query(
    &self,
    query: &(impl ToStatement + ?Sized),
    params: &[&(dyn ToSql + Sync)],
  ) -> Result<RowStream> {
    Ok(self.inner.query_raw(query, param_iter(params)).await?)
  }

  /// Executes a statement and optionally returns the first row of the results.
  ///
  /// If no rows are returned, this function returns `None`.
  pub async fn query_opt(
    &self,
    query: &(impl ToStatement + ?Sized),
    params: &[&(dyn ToSql + Sync)],
  ) -> Result<Option<Row>> {
    let rows = self.query(query, params).await?;

    pin!(rows);

    Ok(rows.next().await.transpose()?)
  }

  /// Executes a statement and returns the first row of the results.
  ///
  /// If no rows are returned, this function returns a [`NoRowsReturned`] error.
  pub async fn query_one(
    &self,
    query: &(impl ToStatement + ?Sized),
    params: &[&(dyn ToSql + Sync)],
  ) -> Result<Row, QueryOneError> {
    self.query_opt(query, params).await?.ok_or(NoRowsReturned)
  }

  /// Rolls back the running transaction.
  pub async fn rollback(&mut self) -> Result {
    Ok(self.inner.batch_execute("ROLLBACK;").await?)
  }
}
