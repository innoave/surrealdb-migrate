use crate::fixtures::db::connect_to_test_database_as_database_user;
use speculoos::prelude::*;
use surrealdb_migrate::config::DEFAULT_MIGRATIONS_TABLE;
use surrealdb_migrate::db::find_migrations_table_info;
use surrealdb_migrate::migration::MigrationsTableInfo;

mod fixtures;

#[tokio::test]
async fn find_migration_table_info_in_empty_database() {
    let db = connect_to_test_database_as_database_user().await;
    let tables_removed = db.query("REMOVE TABLE some_table").await;
    assert_that!(tables_removed).is_ok();

    let result = find_migrations_table_info(DEFAULT_MIGRATIONS_TABLE, &db).await;

    assert_that!(result).is_ok_containing(MigrationsTableInfo::NoTables);
}

#[tokio::test]
async fn find_migration_table_info_in_database_with_no_migrations_table_but_not_empty() {
    let db = connect_to_test_database_as_database_user().await;
    let some_table_defined = db
        .query("DEFINE TABLE some_table SCHEMALESS TYPE NORMAL PERMISSIONS FULL")
        .await;
    assert_that!(some_table_defined).is_ok();

    let result = find_migrations_table_info(DEFAULT_MIGRATIONS_TABLE, &db).await;

    assert_that!(result).is_ok_containing(MigrationsTableInfo::Missing);
}
