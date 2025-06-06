use crate::fixtures::load_environment_variables;
use database_migration::config::{DEFAULT_MIGRATIONS_TABLE, DbAuthLevel, DbClientConfig};
use std::collections::HashMap;
use std::env;
use surrealdb_migrate_db_client::{
    DbConnection, DbError, SURREALDB_CONTAINER_IMAGE_TAG, connect_to_database,
    define_migrations_table,
};
use testcontainers_modules::surrealdb::{SURREALDB_PORT, SurrealDb};
use testcontainers_modules::testcontainers::runners::AsyncRunner;
use testcontainers_modules::testcontainers::{ContainerAsync, ImageExt};

const PREPARE_TEST_PLAYGROUND_SCRIPT: &str = include_str!("prepare_test_playground.surql");

pub fn ns_username() -> String {
    env::var("DB_NAMESPACE_USER").expect("environment variable DB_NAMESPACE_USER not set")
}

pub fn ns_password() -> String {
    env::var("DB_NAMESPACE_PASS").expect("environment variable DB_NAMESPACE_PASS not set")
}

pub fn db_username() -> String {
    env::var("DB_DATABASE_USER").expect("environment variable DB_DATABASE_USER not set")
}

pub fn db_password() -> String {
    env::var("DB_DATABASE_PASS").expect("environment variable DB_DATABASE_PASS not set")
}

pub async fn start_surrealdb_testcontainer() -> ContainerAsync<SurrealDb> {
    SurrealDb::default()
        .with_tag(SURREALDB_CONTAINER_IMAGE_TAG)
        .start()
        .await
        .expect("failed to start SurrealDb testcontainer")
}

pub async fn connect_as_root_user(config: &DbClientConfig<'_>) -> Result<DbConnection, DbError> {
    let config = config.clone().with_auth_level(DbAuthLevel::Root);
    connect_to_database(&config).await
}

pub async fn client_config_for_testcontainer(
    db_server: &ContainerAsync<SurrealDb>,
) -> DbClientConfig<'_> {
    let db_host = db_server
        .get_host()
        .await
        .expect("failed to get SurrealDb host");
    let db_port = db_server
        .get_host_port_ipv4(SURREALDB_PORT)
        .await
        .expect("failed to get SurrealDb port");

    DbClientConfig::default().with_address(format!("ws://{db_host}:{db_port}"))
}

async fn prepare_test_playground(db: &DbConnection) -> Result<(), DbError> {
    db.query(
        PREPARE_TEST_PLAYGROUND_SCRIPT
            .replace("$ns_user", &ns_username())
            .replace("$ns_pass", &ns_password())
            .replace("$db_user", &db_username())
            .replace("$db_pass", &db_password()),
    )
    .await?;
    Ok(())
}

pub async fn initialize_database(config: &DbClientConfig<'_>) {
    let db = connect_as_root_user(config)
        .await
        .expect("failed to connect to database as root user");
    prepare_test_playground(&db)
        .await
        .expect("failed to prepare test playground in database");
}

#[allow(dead_code)]
pub async fn connect_to_test_database_as_database_user(config: DbClientConfig<'_>) -> DbConnection {
    load_environment_variables();
    initialize_database(&config).await;

    let config = config
        .with_namespace("playground")
        .with_database("test")
        .with_auth_level(DbAuthLevel::Database)
        .with_username(db_username())
        .with_password(db_password());

    connect_to_database(&config)
        .await
        .expect("failed to connect to test database")
}

#[allow(dead_code)]
pub async fn define_default_migrations_table(db: &DbConnection) {
    define_migrations_table(DEFAULT_MIGRATIONS_TABLE, db)
        .await
        .expect("failed to define migrations table");
}

#[allow(dead_code)]
pub async fn get_db_tables_info(db: &DbConnection) -> HashMap<String, String> {
    let mut db_info = db
        .query("INFO FOR DB")
        .await
        .expect("failed to query info for db");
    let tables: Option<HashMap<String, String>> = db_info
        .take("tables")
        .expect("failed to get info about tables");
    tables.expect("no info about tables")
}
