#![allow(unused_crate_dependencies)]

mod examples_database;

use crate::examples_database::{
    client_config_for_testcontainer, connect_to_examples_database_as_database_user,
    start_surrealdb_testcontainer,
};
use color_eyre::Report;
use std::collections::HashMap;
use std::path::Path;
use surrealdb_migrate::config::RunnerConfig;
use surrealdb_migrate::db::select_all_executions_sorted_by_key;
use surrealdb_migrate::runner::MigrationRunner;

#[tokio::main]
async fn main() -> Result<(), Report> {
    color_eyre::install()?;

    // prepare database for the example
    let db_server = start_surrealdb_testcontainer().await?;

    // setup database connection
    let config = client_config_for_testcontainer(&db_server).await?;
    let db = connect_to_examples_database_as_database_user(config).await?;

    // Instantiate the `MigrationRunner`
    // Note: we could use `MigrationRunner::default();` instead if the default configuration is suitable.
    let config =
        RunnerConfig::default().with_migrations_folder(Path::new("fixtures/basic/migrations"));
    let runner = MigrationRunner::new(config.clone());

    // Run all forward (up) migrations
    runner.migrate(&db).await?;

    // Just for the example: display what happened
    let executions = select_all_executions_sorted_by_key(config.migrations_table, &db).await?;

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
