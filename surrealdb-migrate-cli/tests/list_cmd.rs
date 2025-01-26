mod fixtures;

use crate::fixtures::db::{
    connect_to_test_database_as_database_user, prepare_test_database, start_surrealdb_testcontainer,
};
use crate::fixtures::surmig;
use assert_fs::TempDir;
use snapbox::file;
use std::time::Duration;
use surrealdb_migrate::checksum::hash_migration_script;
use surrealdb_migrate::config::DEFAULT_MIGRATIONS_TABLE;
use surrealdb_migrate::migration::{Execution, Migration, MigrationKind};
use surrealdb_migrate::test_dsl::{datetime, key};
use surrealdb_migrate_db_client::insert_migration_execution;

#[tokio::test]
async fn list_migrations_non_applied() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;

    let cmd = surmig().args([
        "--config-dir",
        "tests/list_cmd",
        "--db-address",
        &db_config.address,
        "list",
    ]);

    cmd.assert()
        .code(0)
        .stdout_eq(file!("list_cmd/forward_migrations_non_applied.stdout"))
        .stderr_eq("");
}

#[tokio::test]
async fn list_forward_migrations_non_applied() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;

    let cmd = surmig().args([
        "--config-dir",
        "tests/list_cmd",
        "--db-address",
        &db_config.address,
        "list",
        "--up",
    ]);

    cmd.assert()
        .code(0)
        .stdout_eq(file!("list_cmd/forward_migrations_non_applied.stdout"))
        .stderr_eq("");
}

#[tokio::test]
async fn list_forward_migrations_one_applied() {
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

    let cmd = surmig().args([
        "--config-dir",
        "tests/list_cmd",
        "--db-address",
        &db_config.address,
        "list",
        "--up",
    ]);

    cmd.assert()
        .code(0)
        .stdout_eq(file!("list_cmd/forward_migrations_one_applied.stdout"))
        .stderr_eq("");
}

#[tokio::test]
async fn list_open_migrations_one_applied() {
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

    let cmd = surmig().args([
        "--config-dir",
        "tests/list_cmd",
        "--db-address",
        &db_config.address,
        "list",
        "--open",
    ]);

    cmd.assert()
        .code(0)
        .stdout_eq(file!("list_cmd/open_migrations_one_applied.stdout"))
        .stderr_eq("");
}

#[tokio::test]
async fn list_applied_migrations_one_applied() {
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

    let cmd = surmig().args([
        "--config-dir",
        "tests/list_cmd",
        "--db-address",
        &db_config.address,
        "list",
        "--applied",
    ]);

    cmd.assert()
        .code(0)
        .stdout_eq(file!("list_cmd/applied_migrations_one_applied.stdout"))
        .stderr_eq("");
}

#[tokio::test]
async fn list_backward_migrations_non_applied() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;

    let cmd = surmig().args([
        "--config-dir",
        "tests/list_cmd",
        "--db-address",
        &db_config.address,
        "list",
        "--down",
    ]);

    cmd.assert()
        .code(0)
        .stdout_eq(file!("list_cmd/backward_migrations_non_applied.stdout"))
        .stderr_eq("");
}

#[tokio::test]
async fn list_open_backward_migrations_one_applied() {
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

    let cmd = surmig().args([
        "--config-dir",
        "tests/list_cmd",
        "--db-address",
        &db_config.address,
        "list",
        "--down",
        "--open",
    ]);

    cmd.assert()
        .code(0)
        .stdout_eq(file!(
            "list_cmd/open_backward_migrations_one_applied.stdout"
        ))
        .stderr_eq("");
}

#[tokio::test]
async fn list_applied_backward_migrations_one_applied() {
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

    let cmd = surmig().args([
        "--config-dir",
        "tests/list_cmd",
        "--db-address",
        &db_config.address,
        "list",
        "--down",
        "--applied",
    ]);

    cmd.assert()
        .code(0)
        .stdout_eq(file!(
            "list_cmd/applied_backward_migrations_one_applied.stdout"
        ))
        .stderr_eq("");
}

#[tokio::test]
async fn list_migrations_in_empty_directory() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;
    let temp_dir = TempDir::new().expect("failed to create temp dir");

    let cmd = surmig().args([
        "--config-dir",
        "tests/list_cmd",
        "--migrations-folder",
        temp_dir
            .path()
            .to_str()
            .expect("failed to convert migrations folder path to str"),
        "--db-address",
        &db_config.address,
        "list",
    ]);

    cmd.assert()
        .code(0)
        .stdout_eq(file!("list_cmd/in_empty_directory.stdout"))
        .stderr_eq("");
}
