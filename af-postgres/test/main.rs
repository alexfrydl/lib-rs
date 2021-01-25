// Copyright © 2021 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use af_core::test::prelude::*;
use af_postgres as pg;
use structopt::*;

/// Test schema.
const SCHEMA: &str = r#"
  DROP TABLE IF EXISTS test;
  CREATE TABLE test (id SERIAL PRIMARY KEY, value int);
"#;

/// Command line options.
#[derive(StructOpt)]
pub struct Options {
  /// Sets the URL of the PostgreSQL database to connect to.
  #[structopt(long, env = "POSTGRES_URL")]
  url: af_postgres::Config,
}

/// Tests the `af_postgres` package.
#[test::main]
fn main(cx: &mut test::Context) {
  let options = Options::from_args();

  test!(cx, "can execute a transaction", {
    let (mut client, _conn) = pg::connect(&options.url).await?;

    client.batch_execute(SCHEMA).await?;

    let insert = client.prepare("INSERT INTO test (value) VALUES ($1) RETURNING id").await?;
    let select = client.prepare("SELECT value FROM test WHERE id = $1").await?;

    let a: i32 = random();
    let b: i32 = random();

    client.begin().await?;

    let a_id: i32 = client.query_one(&insert, &[&a]).await?.get(0);
    let b_id: i32 = client.query_one(&insert, &[&b]).await?.get(0);

    client.commit().await?;

    fail::when!(a_id != 1);
    fail::when!(b_id != 2);

    let a_value: i32 = client.query_one(&select, &[&a_id]).await?.get(0);
    let b_value: i32 = {
      let mut s = pg::StatementBuilder::new();

      let id = s.add_param(&b_id);
      let value = s.add_param(&b);

      write!(s, "SELECT value FROM test WHERE id = ${} AND value = ${}", id, value)?;

      client.query_one(s.text(), s.params()).await?.get(0)
    };

    fail::when!(a_value != a);
    fail::when!(b_value != b);
  });
}
