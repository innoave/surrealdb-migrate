mod fixtures;
mod test_dsl;

use crate::fixtures::db::{
    client_config_for_testcontainer, connect_to_test_database_as_database_user,
    define_default_migrations_table, start_surrealdb_testcontainer,
};
use crate::test_dsl::key;
use chrono::Utc;
use speculoos::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::Duration;
use surrealdb_migrate::checksum::hash_migration_script;
use surrealdb_migrate::db::apply_migration_in_transaction;
use surrealdb_migrate::migration::{ApplicableMigration, Migration, MigrationKind};

#[tokio::test]
async fn apply_migration_in_transaction_schema_migration() {
    let db_server = start_surrealdb_testcontainer().await;
    let config = client_config_for_testcontainer(&db_server).await;
    let db = connect_to_test_database_as_database_user(config).await;
    define_default_migrations_table(&db).await;

    let script_content =
        fs::read("fixtures/basic/migrations/20250103_140520_define_quote_table.surql")
            .expect("failed to read migration script file");

    let key = key("20250103_140520");

    let migration = Migration {
        key,
        title: "define quote table".to_string(),
        kind: MigrationKind::Up,
        script_path: PathBuf::from(
            "fixtures/basic/migrations/20250103_140520_define_quote_table.surql",
        ),
    };

    let checksum = hash_migration_script(&migration, &script_content);

    let migration = ApplicableMigration {
        key,
        rank: 1,
        checksum,
        script_content: String::from_utf8(script_content).expect("invalid utf8"),
    };

    let start = Utc::now();
    let result = apply_migration_in_transaction(&migration, "some.user", &db).await;

    assert_that!(result).is_ok();
    let execution = result.expect("unreachable");
    assert_that!(execution.key).is_equal_to(key);
    assert_that!(execution.applied_rank).is_equal_to(1);
    assert_that!(execution.applied_by).is_equal_to("some.user".to_string());
    assert_that!(execution.applied_at).is_greater_than_or_equal_to(start);
    assert_that!(execution.checksum).is_equal_to(checksum);
    assert_that!(execution.execution_time).is_greater_than(Duration::from_millis(0));

    let db_info: Option<HashMap<String, String>> = db
        .query("INFO FOR DB")
        .await
        .expect("failed to get info for db query")
        .take("tables")
        .expect("failed to get tables info");

    assert_that!(db_info)
        .is_some()
        .contains_key("quote".to_string());
}
