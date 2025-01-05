use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use surrealdb_migrate::config::{DbAuthLevel, DbClientConfig};
use surrealdb_migrate::db::{connect_to_database, DbConnection, DbError};

const PREPARE_TEST_PLAYGROUND_SCRIPT: &str = include_str!("prepare_test_playground.surql");

static INIT_DB_ONCE: AtomicBool = AtomicBool::new(true);

pub fn root_username() -> String {
    env::var("DB_ROOT_USER").expect("environment variable DB_ROOT_USER not set")
}

pub fn root_password() -> String {
    env::var("DB_ROOT_PASS").expect("environment variable DB_ROOT_PASS not set")
}

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

pub async fn connect_as_root_user() -> Result<DbConnection, DbError> {
    let config = DbClientConfig::default()
        .with_auth_level(DbAuthLevel::Root)
        .with_username("root")
        .with_password("s3cr3t");
    connect_to_database(config).await
}

pub async fn prepare_test_playground(db: &DbConnection) -> Result<(), DbError> {
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

pub async fn initialize_database() {
    if INIT_DB_ONCE.swap(false, Ordering::Relaxed) {
        let db = connect_as_root_user()
            .await
            .expect("failed to connect to database as root user");
        prepare_test_playground(&db)
            .await
            .expect("failed to prepare test playground in database");
    }
}
