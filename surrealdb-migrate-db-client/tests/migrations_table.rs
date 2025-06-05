#![allow(clippy::similar_names, clippy::manual_string_new)]

mod fixtures;

use crate::fixtures::db::{
    client_config_for_testcontainer, connect_to_test_database_as_database_user, get_db_tables_info,
    start_surrealdb_testcontainer,
};
use asserting::prelude::*;
use chrono::DateTime;
use database_migration::checksum::{Checksum, hash_migration_script};
use database_migration::config::{DEFAULT_MIGRATIONS_TABLE, MIGRATION_KEY_FORMAT_STR};
use database_migration::error::Error;
use database_migration::migration::{
    Execution, Migration, MigrationKind, MigrationsTableInfo, Reversion,
};
use database_migration::test_dsl::{datetime, key};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;
use surrealdb::sql;
use surrealdb_migrate_db_client::{
    define_migrations_table, delete_migration_execution, find_max_applied_migration_key,
    find_migrations_table_info, insert_migration_execution,
};

const DEFINE_MIGRATIONS_TABLE: &str = include_str!("../surql/define_migrations_table.surql");

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
struct MigrationExecutionData {
    applied_rank: i64,
    key: String,
    title: String,
    kind: MigrationKind,
    script_path: String,
    checksum: Checksum,
    applied_at: sql::Datetime,
    applied_by: String,
    execution_time: sql::Duration,
}

#[tokio::test]
async fn define_migrations_table_in_empty_database() {
    let db_server = start_surrealdb_testcontainer().await;
    let config = client_config_for_testcontainer(&db_server).await;
    let db = connect_to_test_database_as_database_user(config).await;

    let result = define_migrations_table("my_migrations", &db).await;

    assert_that!(result).is_ok();

    let db_tables_info = get_db_tables_info(&db).await;

    assert_that!(db_tables_info).contains_key("my_migrations");
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

    assert_that!(result).err().is_equal_to(Error::DbQuery(
        "The table 'migrations' already exists".into(),
    ));
}

#[tokio::test]
async fn find_migrations_table_info_in_empty_database() {
    let db_server = start_surrealdb_testcontainer().await;
    let config = client_config_for_testcontainer(&db_server).await;
    let db = connect_to_test_database_as_database_user(config).await;

    let result = find_migrations_table_info(DEFAULT_MIGRATIONS_TABLE, &db).await;

    assert_that!(result)
        .ok()
        .is_equal_to(MigrationsTableInfo::NoTables);
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

    assert_that!(result)
        .ok()
        .is_equal_to(MigrationsTableInfo::Missing);
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

    assert_that!(result).ok().is_equal_to(MigrationsTableInfo::Table {
        name: table_name.into(),
        version: Some("1.0".into()),
        definition: "DEFINE TABLE my_migrations TYPE NORMAL SCHEMAFULL COMMENT 'version:1.0' PERMISSIONS FOR select FULL, FOR create, update, delete NONE".into(),
    });
}

#[tokio::test]
async fn find_migrations_table_info_in_database_with_migrations_table_existing_but_with_different_name()
 {
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

    assert_that!(result)
        .ok()
        .is_equal_to(MigrationsTableInfo::Missing);
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
        title: "define some tables".into(),
        kind: MigrationKind::Up,
        script_path: PathBuf::from("migrations/20250103_153309_define_some_tables.surql"),
    };

    let checksum = hash_migration_script(
        &migration,
        r#"LET $data = ["J. Jonah Jameson", "James Earl Jones"];"#,
    );

    let execution = Execution {
        key,
        applied_rank: 1,
        applied_by: "some.user".into(),
        applied_at: datetime("2025-01-06 07:12:50+01:00"),
        checksum,
        execution_time: Duration::from_millis(380),
    };

    let result =
        insert_migration_execution(migration, execution, DEFAULT_MIGRATIONS_TABLE, &db).await;

    assert_that!(result).is_ok();

    let exec_key = key.format(MIGRATION_KEY_FORMAT_STR).to_string();
    let stored_execution: Option<MigrationExecutionData> = db
        .select((DEFAULT_MIGRATIONS_TABLE, exec_key))
        .await
        .expect("failed to select inserted migration execution");

    assert_that!(stored_execution).is_equal_to(Some(MigrationExecutionData {
        applied_rank: 1,
        key: "20250103_153309".into(),
        title: "define some tables".into(),
        kind: MigrationKind::Up,
        script_path: "migrations/20250103_153309_define_some_tables.surql".into(),
        checksum,
        applied_at: datetime("2025-01-06 07:12:50+01:00").into(),
        applied_by: "some.user".into(),
        execution_time: Duration::from_millis(380).into(),
    }));
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
        title: "define some tables".into(),
        kind: MigrationKind::Up,
        script_path: PathBuf::from("migrations/20250103_153309_define_some_tables.surql"),
    };

    let execution = Execution {
        key,
        applied_rank: 2,
        applied_by: "some.user".into(),
        applied_at: datetime("2025-01-06 07:12:50+01:00"),
        checksum: hash_migration_script(
            &migration,
            r#"LET $data = ["J. Jonah Jameson", "James Earl Jones"];"#,
        ),
        execution_time: Duration::from_millis(380),
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

    assert_that!(result).err().is_equal_to(Error::DbQuery("There was a problem with the database: Database record `migrations:⟨20250103_153309⟩` already exists".into()));
}

#[tokio::test]
async fn delete_migration_execution_which_is_the_only_record() {
    let db_server = start_surrealdb_testcontainer().await;
    let config = client_config_for_testcontainer(&db_server).await;
    let db = connect_to_test_database_as_database_user(config).await;

    let migrations_table_defined = define_migrations_table(DEFAULT_MIGRATIONS_TABLE, &db).await;
    assert_that!(migrations_table_defined).is_ok();

    let mig_key = key("20250103_153309");

    let migration = Migration {
        key: mig_key,
        title: "define some tables".into(),
        kind: MigrationKind::Up,
        script_path: PathBuf::from("migrations/20250103_153309_define_some_tables.surql"),
    };

    let execution = Execution {
        key: mig_key,
        applied_rank: 1,
        applied_by: "some.user".into(),
        applied_at: datetime("2025-01-06 07:12:50+01:00"),
        checksum: hash_migration_script(
            &migration,
            r#"LET $data = ["J. Jonah Jameson", "James Earl Jones"];"#,
        ),
        execution_time: Duration::from_millis(380),
    };

    let result =
        insert_migration_execution(migration, execution, DEFAULT_MIGRATIONS_TABLE, &db).await;

    assert_that!(result).is_ok();

    let executions: Vec<MigrationExecutionData> = db
        .select(DEFAULT_MIGRATIONS_TABLE)
        .await
        .expect("failed to select migration executions");

    assert_that!(executions).has_length(1);

    let reversion = Reversion {
        key: key("20250103_153309"),
        reverted_by: "some.user".into(),
        reverted_at: datetime("2025-01-30 12:42:31+01:00"),
        execution_time: Duration::from_micros(230),
    };

    let result = delete_migration_execution(reversion, DEFAULT_MIGRATIONS_TABLE, &db).await;

    assert_that!(result).is_ok();

    let remaining_executions: Vec<MigrationExecutionData> = db
        .select(DEFAULT_MIGRATIONS_TABLE)
        .await
        .expect("failed to select migration execution");

    assert_that!(remaining_executions).is_empty();
}

#[tokio::test]
async fn delete_migration_execution_from_two_records() {
    let db_server = start_surrealdb_testcontainer().await;
    let config = client_config_for_testcontainer(&db_server).await;
    let db = connect_to_test_database_as_database_user(config).await;

    let migrations_table_defined = define_migrations_table(DEFAULT_MIGRATIONS_TABLE, &db).await;
    assert_that!(migrations_table_defined).is_ok();

    let mig_key1 = key("20250103_153309");

    let migration1 = Migration {
        key: mig_key1,
        title: "define some tables".into(),
        kind: MigrationKind::Up,
        script_path: PathBuf::from("migrations/20250103_153309_define_some_tables.surql"),
    };

    let checksum1 = hash_migration_script(&migration1, "");

    let execution1 = Execution {
        key: mig_key1,
        applied_rank: 1,
        applied_by: "some.user".into(),
        applied_at: datetime("2025-01-06 07:12:50+01:00"),
        checksum: checksum1,
        execution_time: Duration::from_millis(380),
    };

    let result =
        insert_migration_execution(migration1, execution1, DEFAULT_MIGRATIONS_TABLE, &db).await;

    assert_that!(result).is_ok();

    let mig_key2 = key("20250122_091731");

    let migration2 = Migration {
        key: mig_key2,
        title: "define some tables".into(),
        kind: MigrationKind::Up,
        script_path: PathBuf::from("migrations/20250122_091731_define_some_tables.surql"),
    };

    let checksum2 = hash_migration_script(&migration2, "");

    let execution2 = Execution {
        key: mig_key2,
        applied_rank: 2,
        applied_by: "some.user".into(),
        applied_at: datetime("2025-01-06 07:12:50+01:00"),
        checksum: checksum2,
        execution_time: Duration::from_millis(420),
    };

    let result =
        insert_migration_execution(migration2, execution2, DEFAULT_MIGRATIONS_TABLE, &db).await;

    assert_that!(result).is_ok();

    let executions: Vec<MigrationExecutionData> = db
        .select(DEFAULT_MIGRATIONS_TABLE)
        .await
        .expect("failed to select migration executions");

    assert_that!(executions).has_length(2);

    let reversion = Reversion {
        key: key("20250122_091731"),
        reverted_by: "some.user".into(),
        reverted_at: datetime("2025-01-30 12:42:31+01:00"),
        execution_time: Duration::from_micros(210),
    };

    let result = delete_migration_execution(reversion, DEFAULT_MIGRATIONS_TABLE, &db).await;

    assert_that!(result).is_ok();

    let remaining_executions: Vec<MigrationExecutionData> = db
        .select(DEFAULT_MIGRATIONS_TABLE)
        .await
        .expect("failed to select migration execution");

    assert_that!(remaining_executions).contains_exactly(vec![MigrationExecutionData {
        applied_rank: 1,
        key: "20250103_153309".into(),
        title: "define some tables".into(),
        kind: MigrationKind::Up,
        script_path: "migrations/20250103_153309_define_some_tables.surql".into(),
        checksum: checksum1,
        applied_at: datetime("2025-01-06 07:12:50+01:00").into(),
        applied_by: "some.user".into(),
        execution_time: Duration::from_millis(380).into(),
    }]);
}

#[tokio::test]
async fn delete_migration_execution_not_existing() {
    let db_server = start_surrealdb_testcontainer().await;
    let config = client_config_for_testcontainer(&db_server).await;
    let db = connect_to_test_database_as_database_user(config).await;

    let migrations_table_defined = define_migrations_table(DEFAULT_MIGRATIONS_TABLE, &db).await;
    assert_that!(migrations_table_defined).is_ok();

    let mig_key = key("20250103_153309");

    let migration = Migration {
        key: mig_key,
        title: "define some tables".into(),
        kind: MigrationKind::Up,
        script_path: PathBuf::from("migrations/20250103_153309_define_some_tables.surql"),
    };

    let checksum = hash_migration_script(&migration, "");

    let execution = Execution {
        key: mig_key,
        applied_rank: 1,
        applied_by: "some.user".into(),
        applied_at: datetime("2025-01-06 07:12:50+01:00"),
        checksum,
        execution_time: Duration::from_millis(380),
    };

    let result =
        insert_migration_execution(migration, execution, DEFAULT_MIGRATIONS_TABLE, &db).await;

    assert_that!(result).is_ok();

    let executions: Vec<MigrationExecutionData> = db
        .select(DEFAULT_MIGRATIONS_TABLE)
        .await
        .expect("failed to select migration executions");

    assert_that!(executions).has_length(1);

    let reversion = Reversion {
        key: key("20250122_081731"),
        reverted_by: "some.user".into(),
        reverted_at: datetime("2025-01-30 12:42:31+01:00"),
        execution_time: Duration::from_micros(170),
    };

    let result = delete_migration_execution(reversion, DEFAULT_MIGRATIONS_TABLE, &db).await;

    assert_that!(result)
        .err()
        .is_equal_to(Error::ExecutionNotDeleted("20250122_081731".into()));

    let remaining_executions: Vec<MigrationExecutionData> = db
        .select(DEFAULT_MIGRATIONS_TABLE)
        .await
        .expect("failed to select migration executions");

    assert_that!(remaining_executions).contains_exactly(vec![MigrationExecutionData {
        applied_rank: 1,
        key: "20250103_153309".into(),
        title: "define some tables".into(),
        kind: MigrationKind::Up,
        script_path: "migrations/20250103_153309_define_some_tables.surql".into(),
        checksum,
        applied_at: datetime("2025-01-06 07:12:50+01:00").into(),
        applied_by: "some.user".into(),
        execution_time: Duration::from_millis(380).into(),
    }]);
}

#[tokio::test]
async fn delete_migration_execution_from_empty_table() {
    let db_server = start_surrealdb_testcontainer().await;
    let config = client_config_for_testcontainer(&db_server).await;
    let db = connect_to_test_database_as_database_user(config).await;

    let migrations_table_defined = define_migrations_table(DEFAULT_MIGRATIONS_TABLE, &db).await;
    assert_that!(migrations_table_defined).is_ok();

    let reversion = Reversion {
        key: key("20250122_081731"),
        reverted_by: "some.user".into(),
        reverted_at: datetime("2025-01-30 12:42:31+01:00"),
        execution_time: Duration::from_micros(170),
    };

    let result = delete_migration_execution(reversion, DEFAULT_MIGRATIONS_TABLE, &db).await;

    assert_that!(result)
        .err()
        .is_equal_to(Error::ExecutionNotDeleted("20250122_081731".into()));

    let executions: Vec<MigrationExecutionData> = db
        .select(DEFAULT_MIGRATIONS_TABLE)
        .await
        .expect("failed to select migration executions");

    assert_that!(executions).is_empty();
}

#[tokio::test]
async fn find_max_applied_migration_key_empty_table() {
    let db_server = start_surrealdb_testcontainer().await;
    let config = client_config_for_testcontainer(&db_server).await;
    let db = connect_to_test_database_as_database_user(config).await;
    define_migrations_table(DEFAULT_MIGRATIONS_TABLE, &db)
        .await
        .expect("failed to define migrations table");

    let result = find_max_applied_migration_key(DEFAULT_MIGRATIONS_TABLE, &db).await;

    assert_that!(result).ok().is_equal_to(None);
}

#[tokio::test]
async fn find_max_applied_migration_key_in_table_with_2_records() {
    let db_server = start_surrealdb_testcontainer().await;
    let config = client_config_for_testcontainer(&db_server).await;
    let db = connect_to_test_database_as_database_user(config).await;
    define_migrations_table(DEFAULT_MIGRATIONS_TABLE, &db)
        .await
        .expect("failed to define migrations table");

    let _: Option<MigrationExecutionData> = db
        .insert((DEFAULT_MIGRATIONS_TABLE, "20250103_140520"))
        .content(MigrationExecutionData {
            applied_rank: 1,
            key: "20250103_140520".into(),
            title: "".into(),
            kind: MigrationKind::Up,
            script_path: "migrations/20250103_140520.up.surql".into(),
            checksum: hash_migration_script(
                &Migration {
                    key: key("20250103_140520"),
                    title: "".into(),
                    kind: MigrationKind::Up,
                    script_path: "migrations/20250103_140520.up.surql".into(),
                },
                "",
            ),
            applied_at: DateTime::default().into(),
            applied_by: "some.user".into(),
            execution_time: Duration::from_micros(913).into(),
        })
        .await
        .expect("failed to insert migration table entry");

    let _: Option<MigrationExecutionData> = db
        .insert((DEFAULT_MIGRATIONS_TABLE, "20250103_140521"))
        .content(MigrationExecutionData {
            applied_rank: 2,
            key: "20250103_140521".into(),
            title: "".into(),
            kind: MigrationKind::Up,
            script_path: "migrations/20250103_140521.up.surql".into(),
            checksum: hash_migration_script(
                &Migration {
                    key: key("20250103_140521"),
                    title: "".into(),
                    kind: MigrationKind::Up,
                    script_path: "migrations/20250103_140521.up.surql".into(),
                },
                "",
            ),
            applied_at: DateTime::default().into(),
            applied_by: "another.user".into(),
            execution_time: Duration::from_micros(590).into(),
        })
        .await
        .expect("failed to insert migration table entry");

    let result = find_max_applied_migration_key(DEFAULT_MIGRATIONS_TABLE, &db).await;

    assert_that!(result)
        .ok()
        .is_equal_to(Some(key("20250103_140521")));
}
