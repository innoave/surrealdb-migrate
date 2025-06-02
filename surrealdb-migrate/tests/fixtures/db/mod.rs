use super::load_environment_variables;
use std::collections::HashMap;
use std::env;
use surrealdb_migrate::config::{DbAuthLevel, DbClientConfig};
use surrealdb_migrate::db_client::{DbConnection, connect_to_database};
use surrealdb_migrate_db_client::SURREALDB_CONTAINER_IMAGE_TAG;
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
        .expect("failed to start SurrealDB testcontainer")
}

pub async fn prepare_test_database(db_server: &ContainerAsync<SurrealDb>) -> DbClientConfig<'_> {
    let db_host = db_server
        .get_host()
        .await
        .expect("failed to get SurrealDB host");
    let db_port = db_server
        .get_host_port_ipv4(SURREALDB_PORT)
        .await
        .expect("failed to get SurrealDB port");

    let config = DbClientConfig::default().with_address(format!("ws://{db_host}:{db_port}"));
    load_environment_variables();
    initialize_database(&config).await;
    config
        .with_namespace("playground")
        .with_database("test")
        .with_auth_level(DbAuthLevel::Database)
        .with_username(db_username())
        .with_password(db_password())
}

pub async fn connect_as_root_user(config: &DbClientConfig<'_>) -> DbConnection {
    let config = DbClientConfig::default().with_address(config.address.clone());
    connect_to_database(&config)
        .await
        .expect("failed to connect to database as root user")
}

pub async fn connect_to_test_database_as_database_user(
    config: &DbClientConfig<'_>,
) -> DbConnection {
    connect_to_database(config)
        .await
        .expect("failed to connect to test database")
}

async fn initialize_database(config: &DbClientConfig<'_>) {
    let db = connect_as_root_user(config).await;
    prepare_test_playground(&db).await;
}

async fn prepare_test_playground(db: &DbConnection) {
    db.query(
        PREPARE_TEST_PLAYGROUND_SCRIPT
            .replace("$ns_user", &ns_username())
            .replace("$ns_pass", &ns_password())
            .replace("$db_user", &db_username())
            .replace("$db_pass", &db_password()),
    )
    .await
    .expect("failed to prepare test playground in database");
}

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
