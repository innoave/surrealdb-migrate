//! We use `testcontainers` to run a SurrealDB instance in a Docker container. This module provides
//! functions to start the SurrealDB container and setup client connections.
#![allow(clippy::doc_markdown)]

use color_eyre::eyre::WrapErr;
use color_eyre::Report;
use std::env;
use surrealdb_migrate::config::{DbAuthLevel, DbClientConfig};
use surrealdb_migrate::db::{connect_to_database, DbConnection};
use testcontainers_modules::surrealdb::{SurrealDb, SURREALDB_PORT};
use testcontainers_modules::testcontainers::runners::AsyncRunner;
use testcontainers_modules::testcontainers::{ContainerAsync, ImageExt};

const PREPARE_EXAMPLES_PLAYGROUND_SCRIPT: &str = include_str!("prepare_examples_playground.surql");

pub fn ns_username() -> Result<String, Report> {
    env::var("DB_NAMESPACE_USER").wrap_err("environment variable DB_NAMESPACE_USER not set")
}

pub fn ns_password() -> Result<String, Report> {
    env::var("DB_NAMESPACE_PASS").wrap_err("environment variable DB_NAMESPACE_PASS not set")
}

pub fn db_username() -> Result<String, Report> {
    env::var("DB_DATABASE_USER").wrap_err("environment variable DB_DATABASE_USER not set")
}

pub fn db_password() -> Result<String, Report> {
    env::var("DB_DATABASE_PASS").wrap_err("environment variable DB_DATABASE_PASS not set")
}

pub fn load_environment_variables() -> Result<(), Report> {
    dotenvy::dotenv().wrap_err("failed to load environment variables")?;
    Ok(())
}

pub async fn start_surrealdb_testcontainer() -> Result<ContainerAsync<SurrealDb>, Report> {
    SurrealDb::default()
        .with_tag("v2.1")
        .start()
        .await
        .wrap_err("failed to start SurrealDB testcontainer")
}

pub async fn client_config_for_testcontainer(
    db_server: &ContainerAsync<SurrealDb>,
) -> Result<DbClientConfig<'_>, Report> {
    let db_host = db_server
        .get_host()
        .await
        .wrap_err("failed to get SurrealDB host")?;
    let db_port = db_server
        .get_host_port_ipv4(SURREALDB_PORT)
        .await
        .wrap_err("failed to get SurrealDB port")?;

    Ok(DbClientConfig::default().with_address(format!("ws://{db_host}:{db_port}")))
}

pub async fn connect_as_root_user(config: &DbClientConfig<'_>) -> Result<DbConnection, Report> {
    let config = config.clone().with_auth_level(DbAuthLevel::Root);
    connect_to_database(&config)
        .await
        .wrap_err("failed to connect to database as root user")
}

pub async fn connect_to_examples_database_as_database_user(
    config: DbClientConfig<'_>,
) -> Result<DbConnection, Report> {
    load_environment_variables()?;
    initialize_database(&config).await?;

    let config = config
        .with_namespace("playground")
        .with_database("examples")
        .with_auth_level(DbAuthLevel::Database)
        .with_username(db_username()?)
        .with_password(db_password()?);

    connect_to_database(&config)
        .await
        .wrap_err("failed to connect to examples database")
}

async fn initialize_database(config: &DbClientConfig<'_>) -> Result<(), Report> {
    let db = connect_as_root_user(config).await?;
    prepare_examples_playground(&db).await
}

async fn prepare_examples_playground(db: &DbConnection) -> Result<(), Report> {
    db.query(
        PREPARE_EXAMPLES_PLAYGROUND_SCRIPT
            .replace("$ns_user", &ns_username()?)
            .replace("$ns_pass", &ns_password()?)
            .replace("$db_user", &db_username()?)
            .replace("$db_pass", &db_password()?),
    )
    .await
    .wrap_err("failed to prepare examples playground in database")?;
    Ok(())
}
