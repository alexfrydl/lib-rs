// Copyright © 2020 Alexandra Frydl
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! [PostgreSQL](https://postresql.org) integration for
//! [af-lib](https://docs.rs/af-lib/0.1).

mod client;
mod error;
mod statement;

pub use self::client::{connect, Client};
pub use self::error::{Error, Result};
pub use self::statement::{Statement, StatementBuilder, ToStatement};
pub use tokio_postgres::types::{self, FromSql, ToSql, Type};
pub use tokio_postgres::{Config, Row, RowStream};
