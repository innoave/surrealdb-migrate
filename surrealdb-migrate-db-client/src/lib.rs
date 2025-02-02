use chrono::{NaiveDateTime, Utc};
use database_migration::checksum::Checksum;
use database_migration::config::{DbAuthLevel, DbClientConfig, MIGRATION_KEY_FORMAT_STR};
use database_migration::error::Error;
use database_migration::migration::{
    ApplicableMigration, Execution, Migration, MigrationKind, MigrationsTableInfo, Reversion,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use std::time::Instant;
use surrealdb::engine::any::{connect, Any};
use surrealdb::opt::auth;
use surrealdb::opt::auth::Jwt;
use surrealdb::{sql, Surreal};

const DEFINE_MIGRATIONS_TABLE: &str = include_str!("../surql/define_migrations_table.surql");

const TABLE_VERSION_KEY: &str = "version:";

pub type DbError = surrealdb::Error;

#[derive(Debug)]
struct Connection {
    client: Surreal<Any>,
    token: Jwt,
    username: String,
}

#[derive(Debug, Clone)]
pub struct DbConnection {
    inner: Arc<Connection>,
}

impl DbConnection {
    fn new(client: Surreal<Any>, token: Jwt, username: String) -> Self {
        Self {
            inner: Arc::new(Connection {
                client,
                token,
                username,
            }),
        }
    }

    pub fn client(&self) -> &Surreal<Any> {
        &self.inner.client
    }

    pub fn token(&self) -> &Jwt {
        &self.inner.token
    }

    pub fn username(&self) -> &str {
        &self.inner.username
    }
}

impl Deref for DbConnection {
    type Target = Surreal<Any>;

    fn deref(&self) -> &Self::Target {
        &self.inner.client
    }
}

pub async fn connect_to_database(config: &DbClientConfig<'_>) -> Result<DbConnection, DbError> {
    let client = connect(config.address.as_ref()).await?;

    let token = match config.auth_level {
        DbAuthLevel::Root => client.signin(auth::Root {
            username: &config.username,
            password: &config.password,
        }),
        DbAuthLevel::Namespace => client.signin(auth::Namespace {
            namespace: &config.namespace,
            username: &config.username,
            password: &config.password,
        }),
        DbAuthLevel::Database => client.signin(auth::Database {
            namespace: &config.namespace,
            database: &config.database,
            username: &config.username,
            password: &config.password,
        }),
    }
    .await?;

    let _db = client
        .use_ns(config.namespace.to_string())
        .use_db(config.database.to_string());

    Ok(DbConnection::new(
        client,
        token,
        config.username.to_string(),
    ))
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

pub async fn select_all_executions_sorted_by_key(
    migrations_table: &str,
    db: &DbConnection,
) -> Result<Vec<Execution>, Error> {
    let execution_data: Vec<MigrationExecutionData> = db
        .select(migrations_table)
        .await
        .map_err(|err| Error::DbQuery(err.to_string()))?;
    let mut executions = execution_data
        .into_iter()
        .map(|data| {
            NaiveDateTime::parse_from_str(&data.key, MIGRATION_KEY_FORMAT_STR)
                .map_err(|err| Error::DbQuery(err.to_string()))
                .map(|key| Execution {
                    key,
                    applied_rank: data.applied_rank,
                    applied_by: data.applied_by,
                    applied_at: data.applied_at.0,
                    checksum: data.checksum,
                    execution_time: data.execution_time.0,
                })
        })
        .collect::<Result<Vec<_>, _>>()?;
    executions.sort_unstable_by_key(|exec| exec.key);
    Ok(executions)
}

pub async fn select_all_executions(
    migrations_table: &str,
    db: &DbConnection,
) -> Result<HashMap<NaiveDateTime, Execution>, Error> {
    let execution_data: Vec<MigrationExecutionData> = db
        .select(migrations_table)
        .await
        .map_err(|err| Error::DbQuery(err.to_string()))?;
    execution_data
        .into_iter()
        .map(|data| {
            NaiveDateTime::parse_from_str(&data.key, MIGRATION_KEY_FORMAT_STR)
                .map_err(|err| Error::DbQuery(err.to_string()))
                .map(|key| {
                    (
                        key,
                        Execution {
                            key,
                            applied_rank: data.applied_rank,
                            applied_by: data.applied_by,
                            applied_at: data.applied_at.0,
                            checksum: data.checksum,
                            execution_time: data.execution_time.0,
                        },
                    )
                })
        })
        .collect::<Result<HashMap<_, _>, _>>()
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

    _ = response.ok_or_else(|| Error::ExecutionNotInserted(key))?;
    Ok(())
}

pub async fn delete_migration_execution(
    reversion: Reversion,
    migrations_table: &str,
    db: &DbConnection,
) -> Result<(), Error> {
    let key = reversion.key.format(MIGRATION_KEY_FORMAT_STR).to_string();

    let response: Option<MigrationExecutionData> = db
        .delete((migrations_table, key.clone()))
        .await
        .map_err(|err| Error::DbQuery(err.to_string()))?;

    _ = response.ok_or_else(|| Error::ExecutionNotDeleted(key))?;
    Ok(())
}

pub async fn find_max_applied_migration_key(
    migrations_table: &str,
    db: &DbConnection,
) -> Result<Option<NaiveDateTime>, Error> {
    let mut response = db
        .query(format!(
            "SELECT key AS max_key FROM (SELECT key FROM {migrations_table} ORDER BY key DESC) LIMIT 1"
        ))
        .await
        .map_err(|err| Error::DbQuery(err.to_string()))?;

    let result: Option<HashMap<String, String>> = response
        .take(0)
        .map_err(|err| Error::DbQuery(err.to_string()))?;

    let max_applied_key = result
        .and_then(|fields| {
            fields.get("max_key").map(|value| {
                NaiveDateTime::parse_from_str(value, MIGRATION_KEY_FORMAT_STR)
                    .map_err(|err| Error::DbQuery(err.to_string()))
            })
        })
        .transpose()?;

    Ok(max_applied_key)
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
        let result: Option<HashMap<String, i64>> = response
            .take(num_stmts - 1)
            .map_err(|err| Error::DbQuery(err.to_string()))?;

        let max_rank = result
            .and_then(|fields| fields.get("max_rank").copied())
            .unwrap_or(0);
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

pub async fn revert_migration_in_transaction(
    backward_migration: &ApplicableMigration,
    username: &str,
    db: &DbConnection,
) -> Result<Reversion, Error> {
    let reverted_at = Utc::now();
    let start = Instant::now();

    let script_content = &backward_migration.script_content;
    let query = format!(
        "\
BEGIN TRANSACTION;
{script_content}
COMMIT TRANSACTION;
"
    );

    let mut response = db
        .query(query)
        .await
        .map_err(|err| Error::DbQuery(err.to_string()))?;

    let script_errors = response.take_errors();
    if script_errors.is_empty() {
        let execution_time = Instant::now().duration_since(start);

        Ok(Reversion {
            key: backward_migration.key,
            reverted_by: username.into(),
            reverted_at,
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

#[cfg(test)]
mod tests;

// workaround for false positive 'unused extern crate' warnings until
// Rust issue [#95513](https://github.com/rust-lang/rust/issues/95513) is fixed
#[cfg(test)]
mod dummy_extern_uses {
    use dotenvy as _;
    use testcontainers_modules as _;
    use tokio as _;
}
