use crate::fixtures::db::{
    client_config_for_testcontainer, connect_to_test_database_as_database_user,
    start_surrealdb_testcontainer,
};
use speculoos::prelude::*;
use surrealdb_migrate::config::DEFAULT_MIGRATIONS_TABLE;
use surrealdb_migrate::db::{find_migrations_table_info, DEFINE_MIGRATIONS_TABLE};
use surrealdb_migrate::migration::MigrationsTableInfo;

mod fixtures;

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
        .await;
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
        .bind(("migrations_table", table_name))
        .await;
    assert_that!(migrations_table_defined).is_ok();

    let result = find_migrations_table_info(table_name, &db).await;

    assert_that!(result).is_ok_containing(MigrationsTableInfo::Table {
        name: table_name.to_string(),
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
        .bind(("migrations_table", table_name))
        .await;
    assert_that!(migrations_table_defined).is_ok();

    let result = find_migrations_table_info(table_name, &db).await;

    assert_that!(result).is_ok_containing(MigrationsTableInfo::Missing);
}
