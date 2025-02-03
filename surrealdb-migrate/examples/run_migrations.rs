#![allow(unused_crate_dependencies)]

mod examples_database;

use crate::examples_database::{
    client_config_for_testcontainer, db_password, db_username, setup_examples_database,
    start_surrealdb_testcontainer,
};
use anyhow::Context;
use database_migration::config::DbAuthLevel;
use std::collections::HashMap;
use std::path::Path;
use surrealdb_migrate::config::RunnerConfig;
use surrealdb_migrate::runner::MigrationRunner;
use surrealdb_migrate_db_client::{connect_to_database, select_all_executions_sorted_by_key};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // prepare database for the example
    let db_server = start_surrealdb_testcontainer().await?;
    let config = client_config_for_testcontainer(&db_server).await?;
    setup_examples_database(config.clone()).await?;

    // setup database connection
    let config = config
        .with_namespace("playground")
        .with_database("examples")
        .with_auth_level(DbAuthLevel::Database)
        .with_username(db_username()?)
        .with_password(db_password()?);
    let db = connect_to_database(&config)
        .await
        .context("failed to connect to examples database")?;

    // Instantiate the `MigrationRunner`
    let config =
        RunnerConfig::default().with_migrations_folder(Path::new("fixtures/basic/migrations"));
    let runner = MigrationRunner::new(config.clone());

    // Run all forward (up) migrations
    runner.migrate(&db).await?;

    // Just for the example: display what happened
    let executions = select_all_executions_sorted_by_key(&config.migrations_table, &db).await?;

    println!("\nApplied Migrations: ");
    println!("  1: {:?}", executions[0]);
    println!("  2: {:?}", executions[1]);

    let mut db_info = db.query("INFO FOR DB").await?;
    let tables: Option<HashMap<String, String>> = db_info.take("tables")?;
    let tables = tables.expect("missing tables");

    println!("\nDefined Tables: ");
    println!("  quote: {:?}", tables["quote"]);

    let quotes: Vec<HashMap<String, String>> = db
        .query("SELECT text FROM quote ORDER BY text")
        .await?
        .take(0)?;

    println!("\nInserted Quotes: ");
    for quote in quotes {
        println!("  {}", quote["text"]);
    }
    println!();

    Ok(())
}
