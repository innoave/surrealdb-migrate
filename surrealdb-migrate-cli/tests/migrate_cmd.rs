mod fixtures;

use crate::fixtures::db::{
    connect_to_test_database_as_database_user, prepare_test_database, start_surrealdb_testcontainer,
};
use crate::fixtures::surmig;
use database_migration::test_dsl::{datetime, key};
use std::path::Path;
use std::time::Duration;
use surrealdb_migrate::checksum::hash_migration_script;
use surrealdb_migrate::config::{RunnerConfig, DEFAULT_MIGRATIONS_TABLE};
use surrealdb_migrate::migration::{Execution, Migration, MigrationKind};
use surrealdb_migrate::runner::MigrationRunner;
use surrealdb_migrate_db_client::insert_migration_execution;

#[tokio::test]
async fn migrate_empty_database() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;

    let cmd = surmig().args([
        "--config-dir",
        "tests/migrate_cmd",
        "--db-address",
        &db_config.address,
        "migrate",
    ]);

    cmd.assert()
        .code(0)
        .stdout_eq(snapbox::file!("migrate_cmd/empty_database.stdout"))
        .stderr_eq("");
}

#[tokio::test]
async fn migrate_fully_migrated_database_ignoring_checksum() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;
    let db = connect_to_test_database_as_database_user(&db_config).await;

    let migration1 = Migration {
        key: key("20250103_140520"),
        title: "define quote table".into(),
        kind: MigrationKind::Up,
        script_path:
            "../fixture/with_down_migrations/migrations/20250103_140520_define_quote_table.surql"
                .into(),
    };

    let checksum1 = hash_migration_script(&migration1, "");

    let execution1 = Execution {
        key: key("20250103_140520"),
        applied_rank: 1,
        applied_by: "tester".into(),
        applied_at: datetime("2025-01-20T09:10:19Z"),
        checksum: checksum1,
        execution_time: Duration::from_micros(256),
    };

    insert_migration_execution(migration1, execution1, DEFAULT_MIGRATIONS_TABLE, &db)
        .await
        .expect("failed to insert migration 1");

    let migration2 = Migration {
        key: key("20250103_140521"),
        title: "create some quotes".into(),
        kind: MigrationKind::Up,
        script_path:
            "../fixture/with_down_migrations/migrations/20250103_140521_create_some_quotes.surql"
                .into(),
    };

    let checksum2 = hash_migration_script(&migration2, "");

    let execution2 = Execution {
        key: key("20250103_140521"),
        applied_rank: 2,
        applied_by: "tester".into(),
        applied_at: datetime("2025-01-20T09:10:20Z"),
        checksum: checksum2,
        execution_time: Duration::from_micros(122),
    };

    insert_migration_execution(migration2, execution2, DEFAULT_MIGRATIONS_TABLE, &db)
        .await
        .expect("failed to insert migration 2");

    let cmd = surmig().args([
        "--config-dir",
        "tests/migrate_cmd",
        "--db-address",
        &db_config.address,
        "migrate",
        "--ignore-checksum",
    ]);

    cmd.assert()
        .code(0)
        .stdout_eq(snapbox::file!("migrate_cmd/fully_migrated_database.stdout"))
        .stderr_eq("");
}

#[tokio::test]
async fn migrate_partially_migrated_database_ignoring_order() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;
    let db = connect_to_test_database_as_database_user(&db_config).await;

    let runner = MigrationRunner::new(
        RunnerConfig::default()
            .with_migrations_folder(Path::new("../fixtures/with_down_migrations/migrations")),
    );
    runner
        .migrate_to(key("20250103_140520"), &db)
        .await
        .expect("failed to run migrate");

    let cmd = surmig().args([
        "--config-dir",
        "tests/migrate_cmd",
        "--db-address",
        &db_config.address,
        "migrate",
        "--ignore-order",
    ]);

    cmd.assert()
        .code(0)
        .stdout_eq(snapbox::file!(
            "migrate_cmd/partially_migrated_database.stdout"
        ))
        .stderr_eq("");
}

#[tokio::test]
async fn migrate_empty_database_up_to_20250103_140520() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;

    let cmd = surmig().args([
        "--config-dir",
        "tests/migrate_cmd",
        "--db-address",
        &db_config.address,
        "migrate",
        "--to",
        "20250103_140520",
    ]);

    cmd.assert()
        .code(0)
        .stdout_eq(snapbox::file!(
            "migrate_cmd/empty_database_up_to_20250103_140520.stdout"
        ))
        .stderr_eq("");
}

#[tokio::test]
async fn migrate_empty_database_up_to_missing_key() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;

    let cmd = surmig().args([
        "--config-dir",
        "tests/migrate_cmd",
        "--db-address",
        &db_config.address,
        "migrate",
        "--to",
    ]);

    cmd.assert().code(2).stdout_eq("").stderr_eq(snapbox::file!(
        "migrate_cmd/empty_database_up_to_missing_key.stderr"
    ));
}

#[tokio::test]
async fn migrate_empty_database_up_to_empty_key() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;

    let cmd = surmig().args([
        "--config-dir",
        "tests/migrate_cmd",
        "--db-address",
        &db_config.address,
        "migrate",
        "--to",
        "",
    ]);

    cmd.assert().code(1).stdout_eq("").stderr_eq(snapbox::file!(
        "migrate_cmd/empty_database_up_to_empty_key.stderr"
    ));
}

#[tokio::test]
async fn migrate_empty_database_up_to_invalid_key() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;

    let cmd = surmig().args([
        "--config-dir",
        "tests/migrate_cmd",
        "--db-address",
        &db_config.address,
        "migrate",
        "--to",
        "V0101",
    ]);

    cmd.assert().code(1).stdout_eq("").stderr_eq(snapbox::file!(
        "migrate_cmd/empty_database_up_to_invalid_key.stderr"
    ));
}
