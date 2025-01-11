#![allow(clippy::ref_option)]

use crate::checksum::Checksum;
use crate::config::{DbAuthLevel, DbClientConfig, MIGRATION_KEY_FORMAT_STR};
use crate::error::Error;
use crate::migration::{
    ApplicableMigration, Execution, Migration, MigrationKind, MigrationsTableInfo,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Instant;
use surrealdb::engine::any::{connect, Any};
use surrealdb::opt::auth;
use surrealdb::opt::auth::Jwt;
use surrealdb::{sql, Surreal};

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

pub async fn connect_to_database(config: &DbClientConfig<'_>) -> Result<DbConnection, DbError> {
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

pub async fn define_migrations_table(table_name: &str, db: &DbConnection) -> Result<(), Error> {
    db.query(DEFINE_MIGRATIONS_TABLE.replace("$migrations_table", table_name))
        .await
        .map_err(|err| Error::DbQuery(err.to_string()))?
        .check()
        .map_err(|err| Error::DbQuery(err.to_string()))?;
    Ok(())
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

#[derive(Serialize, Deserialize)]
struct MigrationExecutionData {
    applied_rank: i64,
    key: String,
    title: String,
    kind: MigrationKind,
    script_path: String,
    checksum: Checksum,
    applied_at: sql::Datetime,
    applied_by: String,
    execution_time: sql::Duration,
}

pub async fn insert_migration_execution(
    migration: Migration,
    execution: Execution,
    migrations_table: &str,
    db: &DbConnection,
) -> Result<(), Error> {
    let key = execution.key.format(MIGRATION_KEY_FORMAT_STR).to_string();

    let content = MigrationExecutionData {
        applied_rank: execution.applied_rank,
        key: key.clone(),
        title: migration.title,
        kind: migration.kind,
        script_path: migration.script_path.to_string_lossy().into(),
        checksum: execution.checksum,
        applied_at: sql::Datetime::from(execution.applied_at),
        applied_by: execution.applied_by,
        execution_time: sql::Duration::from(execution.execution_time),
    };

    let response: Option<MigrationExecutionData> = db
        .create((migrations_table, key.clone()))
        .content(content)
        .await
        .map_err(|err| Error::DbQuery(err.to_string()))?;

    _ = response.ok_or_else(|| Error::ExecutionNotInserted(key.to_string()))?;
    Ok(())
}

pub async fn apply_migration_in_transaction(
    migration: &ApplicableMigration,
    username: &str,
    migrations_table: &str,
    db: &DbConnection,
) -> Result<Execution, Error> {
    let applied_at = Utc::now();
    let start = Instant::now();

    let script_content = &migration.script_content;
    let query = format!(
        "\
BEGIN TRANSACTION;
{script_content}
COMMIT TRANSACTION;
RETURN SELECT math::max(applied_rank) AS max_rank FROM {migrations_table} GROUP ALL;
"
    );

    let mut response = db
        .query(query)
        .await
        .map_err(|err| Error::DbQuery(err.to_string()))?;

    let script_errors = response.take_errors();
    if script_errors.is_empty() {
        let num_stmts = response.num_statements();
        let result: Option<i64> = response
            .take(num_stmts - 1)
            .map_err(|err| Error::DbQuery(err.to_string()))?;

        let max_rank = result.unwrap_or(0);
        let applied_rank = max_rank + 1;

        let execution_time = Instant::now().duration_since(start);

        Ok(Execution {
            key: migration.key,
            applied_rank,
            applied_by: username.into(),
            applied_at,
            checksum: migration.checksum,
            execution_time,
        })
    } else {
        let errors = script_errors
            .into_iter()
            .map(|(index, err)| (index, err.to_string()))
            .collect();
        Err(Error::DbScript(errors))
    }
}

pub fn within_transaction(query: &str) -> String {
    format!("BEGIN TRANSACTION;\n\n{query}\n\nCOMMIT TRANSACTION;")
}

#[cfg(test)]
mod tests;
