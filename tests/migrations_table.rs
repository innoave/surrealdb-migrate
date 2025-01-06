mod fixtures;
mod test_dsl;

use crate::fixtures::db::{
    client_config_for_testcontainer, connect_to_test_database_as_database_user,
    start_surrealdb_testcontainer,
};
use crate::test_dsl::{datetime, key};
use speculoos::prelude::*;
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
use surrealdb_migrate::checksum::hash_migration_script;
use surrealdb_migrate::config::DEFAULT_MIGRATIONS_TABLE;
use surrealdb_migrate::db::{
    define_migrations_table, find_migrations_table_info, insert_migration_execution,
    DEFINE_MIGRATIONS_TABLE,
};
use surrealdb_migrate::error::Error;
use surrealdb_migrate::migration::{Direction, Execution, Migration, MigrationsTableInfo};

#[tokio::test]
async fn define_migrations_table_in_empty_database() {
    let db_server = start_surrealdb_testcontainer().await;
    let config = client_config_for_testcontainer(&db_server).await;
    let db = connect_to_test_database_as_database_user(config).await;

    let result = define_migrations_table("my_migrations", &db).await;

    assert_that!(result).is_ok();

    let tables: Option<HashMap<String, String>> = db
        .query("INFO FOR DB")
        .await
        .expect("failed query for db info")
        .take("tables")
        .expect("tables info not found");
    let tables = tables.expect("tables info not found");

    assert_that!(tables).contains_key("my_migrations".to_string());
}

#[tokio::test]
async fn define_migrations_table_in_database_with_existing_migrations_table() {
    let db_server = start_surrealdb_testcontainer().await;
    let config = client_config_for_testcontainer(&db_server).await;
    let db = connect_to_test_database_as_database_user(config).await;

    let migrations_table_defined = db
        .query(DEFINE_MIGRATIONS_TABLE.replace("$migrations_table", DEFAULT_MIGRATIONS_TABLE))
        .await
        .expect("failed to read response")
        .check();
    assert_that!(migrations_table_defined).is_ok();

    let result = define_migrations_table(DEFAULT_MIGRATIONS_TABLE, &db).await;

    assert_that!(result).is_err_containing(Error::DbQuery(
        "The table 'migrations' already exists".into(),
    ));
}

#[tokio::test]
async fn find_migrations_table_info_in_empty_database() {
    let db_server = start_surrealdb_testcontainer().await;
    let config = client_config_for_testcontainer(&db_server).await;
    let db = connect_to_test_database_as_database_user(config).await;

    let result = find_migrations_table_info(DEFAULT_MIGRATIONS_TABLE, &db).await;

    assert_that!(result).is_ok_containing(MigrationsTableInfo::NoTables);
}

#[tokio::test]
async fn find_migrations_table_info_in_database_with_no_migrations_table_but_not_empty() {
    let db_server = start_surrealdb_testcontainer().await;
    let config = client_config_for_testcontainer(&db_server).await;
    let db = connect_to_test_database_as_database_user(config).await;

    let some_table_defined = db
        .query("DEFINE TABLE some_table SCHEMALESS TYPE NORMAL PERMISSIONS FULL")
        .await
        .expect("failed to read response")
        .check();
    assert_that!(some_table_defined).is_ok();

    let result = find_migrations_table_info(DEFAULT_MIGRATIONS_TABLE, &db).await;

    assert_that!(result).is_ok_containing(MigrationsTableInfo::Missing);
}

#[tokio::test]
async fn find_migrations_table_info_in_database_with_migrations_table_existing() {
    let db_server = start_surrealdb_testcontainer().await;
    let config = client_config_for_testcontainer(&db_server).await;
    let db = connect_to_test_database_as_database_user(config).await;

    let table_name = "my_migrations";

    let migrations_table_defined = db
        .query(DEFINE_MIGRATIONS_TABLE.replace("$migrations_table", table_name))
        .await
        .expect("failed to read response")
        .check();
    assert_that!(migrations_table_defined).is_ok();

    let result = find_migrations_table_info(table_name, &db).await;

    assert_that!(result).is_ok_containing(MigrationsTableInfo::Table {
        name: table_name.into(),
        version: Some("1.0".into()),
        definition: "DEFINE TABLE my_migrations TYPE NORMAL SCHEMAFULL COMMENT 'version:1.0' PERMISSIONS FOR select FULL, FOR create, update, delete NONE".into(),
    });
}

#[tokio::test]
async fn find_migrations_table_info_in_database_with_migrations_table_existing_but_with_different_name(
) {
    let db_server = start_surrealdb_testcontainer().await;
    let config = client_config_for_testcontainer(&db_server).await;
    let db = connect_to_test_database_as_database_user(config).await;

    let table_name = "my_migrations";

    let migrations_table_defined = db
        .query(DEFINE_MIGRATIONS_TABLE.replace("$migrations_table", DEFAULT_MIGRATIONS_TABLE))
        .await
        .expect("failed to read response")
        .check();
    assert_that!(migrations_table_defined).is_ok();

    let result = find_migrations_table_info(table_name, &db).await;

    assert_that!(result).is_ok_containing(MigrationsTableInfo::Missing);
}

#[tokio::test]
async fn insert_migration_execution_first_record() {
    let db_server = start_surrealdb_testcontainer().await;
    let config = client_config_for_testcontainer(&db_server).await;
    let db = connect_to_test_database_as_database_user(config).await;

    let migrations_table_defined = define_migrations_table(DEFAULT_MIGRATIONS_TABLE, &db).await;
    assert_that!(migrations_table_defined).is_ok();

    let key = key("20250103_153309");

    let migration = Migration {
        key,
        title: "define_some_tables".to_string(),
        direction: Direction::Up,
        script_path: PathBuf::from("migrations/20250103_153309_define_some_tables.surql"),
    };

    let execution = Execution {
        key,
        applied_at: datetime("2025-01-06 07:12:50+01:00"),
        checksum: hash_migration_script(&migration, &[]),
        execution_time: Duration::from_millis(380),
        success: true,
    };

    let result =
        insert_migration_execution(migration, execution, DEFAULT_MIGRATIONS_TABLE, &db).await;

    assert_that!(result).is_ok();
}

#[tokio::test]
async fn insert_migration_execution_with_same_key_as_existing_one() {
    let db_server = start_surrealdb_testcontainer().await;
    let config = client_config_for_testcontainer(&db_server).await;
    let db = connect_to_test_database_as_database_user(config).await;

    let migrations_table_defined = define_migrations_table(DEFAULT_MIGRATIONS_TABLE, &db).await;
    assert_that!(migrations_table_defined).is_ok();

    let key = key("20250103_153309");

    let migration = Migration {
        key,
        title: "define_some_tables".to_string(),
        direction: Direction::Up,
        script_path: PathBuf::from("migrations/20250103_153309_define_some_tables.surql"),
    };

    let execution = Execution {
        key,
        applied_at: datetime("2025-01-06 07:12:50+01:00"),
        checksum: hash_migration_script(&migration, &[]),
        execution_time: Duration::from_millis(380),
        success: true,
    };

    let result = insert_migration_execution(
        migration.clone(),
        execution.clone(),
        DEFAULT_MIGRATIONS_TABLE,
        &db,
    )
    .await;

    assert_that!(result).is_ok();

    let result =
        insert_migration_execution(migration, execution, DEFAULT_MIGRATIONS_TABLE, &db).await;

    assert_that!(result).is_err_containing(Error::DbQuery("There was a problem with the database: Database record `migrations:20250103_153309` already exists".to_string()));
}
