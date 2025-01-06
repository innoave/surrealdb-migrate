use crate::config::{DbAuthLevel, DbClientConfig};
use crate::error::Error;
use crate::migration::MigrationsTableInfo;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use surrealdb::engine::any::{connect, Any};
use surrealdb::opt::auth;
use surrealdb::opt::auth::Jwt;
use surrealdb::Surreal;

pub const DEFINE_MIGRATIONS_TABLE: &str = include_str!("../../surql/define_migrations_table.surql");

pub const TABLE_VERSION_KEY: &str = "version:";

pub type DbError = surrealdb::Error;

#[derive(Debug)]
struct Connection {
    client: Surreal<Any>,
    token: Jwt,
}

#[derive(Debug, Clone)]
pub struct DbConnection {
    inner: Arc<Connection>,
}

impl DbConnection {
    pub fn new(client: Surreal<Any>, token: Jwt) -> Self {
        Self {
            inner: Arc::new(Connection { client, token }),
        }
    }

    pub fn client(&self) -> &Surreal<Any> {
        &self.inner.client
    }

    pub fn token(&self) -> &Jwt {
        &self.inner.token
    }
}

impl From<(Surreal<Any>, Jwt)> for DbConnection {
    fn from((client, token): (Surreal<Any>, Jwt)) -> Self {
        Self::new(client, token)
    }
}

impl Deref for DbConnection {
    type Target = Surreal<Any>;

    fn deref(&self) -> &Self::Target {
        &self.inner.client
    }
}

pub async fn connect_to_database(config: DbClientConfig<'_>) -> Result<DbConnection, DbError> {
    let client = connect(config.address_or_default()).await?;

    let token = match config.auth_level() {
        DbAuthLevel::Root => client.signin(auth::Root {
            username: config.username_or_default(),
            password: config.password_or_default(),
        }),
        DbAuthLevel::Namespace => client.signin(auth::Namespace {
            namespace: config.namespace_or_default(),
            username: config.username_or_default(),
            password: config.password_or_default(),
        }),
        DbAuthLevel::Database => client.signin(auth::Database {
            namespace: config.namespace_or_default(),
            database: config.database_or_default(),
            username: config.username_or_default(),
            password: config.password_or_default(),
        }),
    }
    .await?;

    let _db = client
        .use_ns(config.namespace_or_default())
        .use_db(config.database_or_default());

    Ok(DbConnection::new(client, token))
}

pub async fn find_migrations_table_info(
    table_name: &str,
    db: &DbConnection,
) -> Result<MigrationsTableInfo, Error> {
    let mut db_info = db
        .query("INFO FOR DB")
        .await
        .map_err(|err| Error::DbQuery(err.to_string()))?;
    let tables: Option<HashMap<String, String>> = db_info
        .take("tables")
        .map_err(|err| Error::DbQuery(err.to_string()))?;
    let mut tables = tables.ok_or_else(|| Error::FetchingTableDefinitions(String::new()))?;
    if tables.is_empty() {
        return Ok(MigrationsTableInfo::NoTables);
    }
    tables
        .remove(table_name)
        .map_or(Ok(MigrationsTableInfo::Missing), |definition| {
            let version = extract_table_definition_version(&definition);
            Ok(MigrationsTableInfo::Table {
                name: table_name.to_owned(),
                version,
                definition,
            })
        })
}

fn extract_table_definition_version(table_definition: &str) -> Option<String> {
    table_definition
        .find(TABLE_VERSION_KEY)
        .and_then(|start| {
            table_definition
                .char_indices()
                .skip(start)
                .find_map(|(i, c)| {
                    if c == '\'' {
                        Some((start + TABLE_VERSION_KEY.len(), i))
                    } else {
                        None
                    }
                })
        })
        .map(|(start, end)| {
            table_definition
                .chars()
                .skip(start)
                .take(end - start)
                .collect::<String>()
        })
}

#[cfg(test)]
mod tests;
