mod fixtures;

use crate::fixtures::db::{
    client_config_for_testcontainer, connect_to_test_database_as_database_user,
    define_default_migrations_table, start_surrealdb_testcontainer,
};
use assertor::*;
use chrono::Utc;
use database_migration::checksum::hash_migration_script;
use database_migration::config::DEFAULT_MIGRATIONS_TABLE;
use database_migration::error::Error;
use database_migration::migration::{ApplicableMigration, Migration, MigrationKind};
use database_migration::test_dsl::key;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use surrealdb_migrate_db_client::apply_migration_in_transaction;

#[tokio::test]
async fn apply_migration_in_transaction_schema_migration() {
    let db_server = start_surrealdb_testcontainer().await;
    let config = client_config_for_testcontainer(&db_server).await;
    let db = connect_to_test_database_as_database_user(config).await;
    define_default_migrations_table(&db).await;

    let script_content =
        fs::read_to_string("../fixtures/basic/migrations/20250103_140520_define_quote_table.surql")
            .expect("failed to read migration script file");

    let key = key("20250103_140520");

    let migration = Migration {
        key,
        title: "define quote table".to_string(),
        kind: MigrationKind::Up,
        script_path: PathBuf::from(
            "../fixtures/basic/migrations/20250103_140520_define_quote_table.surql",
        ),
    };

    let checksum = hash_migration_script(&migration, &script_content);

    let migration = ApplicableMigration {
        key,
        kind: MigrationKind::Up,
        checksum,
        script_content: script_content.clone(),
    };

    let start = Utc::now();
    let result =
        apply_migration_in_transaction(&migration, "some.user", DEFAULT_MIGRATIONS_TABLE, &db)
            .await;

    assert_that!(result).is_ok();
    let execution = result.expect("unreachable");
    assert_that!(execution.key).is_equal_to(key);
    assert_that!(execution.applied_rank).is_equal_to(1);
    assert_that!(execution.applied_by).is_equal_to("some.user".to_string());
    assert_that!(execution.applied_at).is_at_least(start);
    assert_that!(execution.checksum).is_equal_to(checksum);
    assert_that!(execution.execution_time).is_greater_than(Duration::from_millis(0));

    let db_info: Option<HashMap<String, String>> = db
        .query("INFO FOR DB")
        .await
        .expect("failed to get info for db query")
        .take("tables")
        .expect("failed to get tables info");

    assert_that!(db_info)
        .some()
        .contains_key("quote".to_string());
}

#[tokio::test]
async fn apply_migration_in_transaction_schema_migration_with_error() {
    let db_server = start_surrealdb_testcontainer().await;
    let config = client_config_for_testcontainer(&db_server).await;
    let db = connect_to_test_database_as_database_user(config).await;
    define_default_migrations_table(&db).await;

    let mut script_content =
        fs::read_to_string("../fixtures/basic/migrations/20250103_140520_define_quote_table.surql")
            .expect("failed to read migration script file");
    script_content.push_str(r#"THROW "test script error";"#);

    let key = key("20250103_140520");

    let migration = Migration {
        key,
        title: "define quote table".to_string(),
        kind: MigrationKind::Up,
        script_path: PathBuf::from(
            "../fixtures/basic/migrations/20250103_140520_define_quote_table.surql",
        ),
    };

    let checksum = hash_migration_script(&migration, &script_content);

    let migration = ApplicableMigration {
        key,
        kind: MigrationKind::Up,
        checksum,
        script_content: script_content.clone(),
    };

    let result =
        apply_migration_in_transaction(&migration, "some.user", DEFAULT_MIGRATIONS_TABLE, &db)
            .await;

    match result {
        Ok(value) => {
            panic!("script error expected but was Ok({value:?}");
        },
        Err(Error::DbScript(err_map)) => {
            assert_that!(err_map.get(&4))
                .some()
                .is_equal_to(&"An error occurred: test script error".to_string());
        },
        Err(other) => {
            panic!("expected Error::DbScript but was {other:?}");
        },
    }

    let table_info: Option<HashMap<String, String>> = db
        .query("INFO FOR DB")
        .await
        .expect("failed to get info for db query")
        .take("tables")
        .expect("failed to get tables info");

    assert_that!(table_info)
        .some()
        .does_not_contain_key("quote".to_string());
    assert_that!(table_info)
        .some()
        .contains_key(DEFAULT_MIGRATIONS_TABLE.to_string());
    assert_that!(table_info).some().has_length(1);
}
