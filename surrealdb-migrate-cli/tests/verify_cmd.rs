use crate::fixtures::db::{
    connect_to_test_database_as_database_user, prepare_test_database, start_surrealdb_testcontainer,
};
use crate::fixtures::surmig;
use assert_fs::TempDir;
use assertor::*;
use database_migration::test_dsl::{datetime, key};
use std::fs;
use std::fs::read_to_string;
use std::path::Path;
use std::time::Duration;
use surrealdb_migrate::checksum::hash_migration_script;
use surrealdb_migrate::config::{DEFAULT_MIGRATIONS_TABLE, RunnerConfig};
use surrealdb_migrate::migration::{Execution, Migration, MigrationKind};
use surrealdb_migrate::result::Migrated;
use surrealdb_migrate::runner::MigrationRunner;
use surrealdb_migrate_db_client::insert_migration_execution;

mod fixtures;

#[tokio::test]
async fn verify_migrations_on_empty_migrations_folder() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;

    let temp_dir =
        TempDir::new().unwrap_or_else(|err| panic!("could not create temporary directory: {err}"));
    let empty_folder = temp_dir.path();

    let cmd = surmig().args([
        "--config-dir",
        "tests/verify_cmd",
        "--db-address",
        &db_config.address,
        "--migrations-folder",
        &empty_folder.to_string_lossy(),
        "verify",
    ]);

    cmd.assert()
        .code(0)
        .stdout_eq(snapbox::file!("verify_cmd/empty_migrations_folder.stdout"))
        .stderr_eq("");
}

#[tokio::test]
async fn verify_migrations_empty_database_no_problematic_migrations() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;

    let cmd = surmig().args([
        "--config-dir",
        "tests/verify_cmd",
        "--db-address",
        &db_config.address,
        "verify",
    ]);

    cmd.assert()
        .code(0)
        .stdout_eq(snapbox::file!(
            "verify_cmd/empty_database_no_problematic_migrations.stdout"
        ))
        .stderr_eq("");
}

#[tokio::test]
async fn verify_migrations_fully_migrated_database_no_problematic_migrations() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;
    let db = connect_to_test_database_as_database_user(&db_config).await;

    let migration1 = Migration {
        key: key("20250103_140520"),
        title: "define quote table".into(),
        kind: MigrationKind::Up,
        script_path:
            "../fixtures/with_down_migrations/migrations/20250103_140520_define_quote_table.up.surql"
                .into(),
    };

    let script_content1 = read_to_string(&migration1.script_path)
        .unwrap_or_else(|err| panic!("failed to read script 1: {err}"));
    let checksum1 = hash_migration_script(&migration1, &script_content1);

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
        key: key("20250103_141521"),
        title: "create some quotes".into(),
        kind: MigrationKind::Up,
        script_path:
            "../fixtures/with_down_migrations/migrations/20250103_141521_create_some_quotes.surql"
                .into(),
    };

    let script_content2 = read_to_string(&migration2.script_path)
        .unwrap_or_else(|err| panic!("failed to read script 2: {err}"));
    let checksum2 = hash_migration_script(&migration2, &script_content2);

    let execution2 = Execution {
        key: key("20250103_141521"),
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
        "tests/verify_cmd",
        "--db-address",
        &db_config.address,
        "verify",
    ]);

    cmd.assert()
        .code(0)
        .stdout_eq(snapbox::file!(
            "verify_cmd/fully_migrated_database_no_problematic_migrations.stdout"
        ))
        .stderr_eq("");
}

#[tokio::test]
async fn verify_migrations_one_migration_changed_one_migration_out_of_order() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;
    let db = connect_to_test_database_as_database_user(&db_config).await;

    let temp_dir = TempDir::new().unwrap_or_else(|err| panic!("could not create temp dir: {err}"));
    let migrations_folder = temp_dir.path();

    // copy migration files to temp folder
    let read_dir = fs::read_dir(Path::new("../fixtures/with_down_migrations/migrations"))
        .unwrap_or_else(|err| panic!("could not read migrations folder: {err}"));
    for dir_entry in read_dir.flatten() {
        let src_path = dir_entry.path();
        if src_path.is_file() {
            let filename = src_path.file_name().expect("src path has no filename");
            fs::copy(&src_path, migrations_folder.join(filename))
                .unwrap_or_else(|err| panic!("failed to copy migration file {src_path:?}: {err}"));
        }
    }

    // migrate database
    let runner_config = RunnerConfig::default().with_migrations_folder(migrations_folder);
    let runner = MigrationRunner::new(runner_config);
    let migrated = runner
        .migrate(&db)
        .await
        .unwrap_or_else(|err| panic!("failed to migrate database: {err}"));

    assert_that!(migrated).is_equal_to(Migrated::UpTo(key("20250103_141521")));

    // modify already applied migration script
    let script_path = migrations_folder.join("20250103_141521_create_some_quotes.surql");
    fs::write(&script_path, "")
        .unwrap_or_else(|err| panic!("failed to write out-of-order migration file: {err}"));

    // create new migration file out of order
    fs::write(
        migrations_folder.join("20250103_141030_out_of_order_migration.surql"),
        "",
    )
    .unwrap_or_else(|err| panic!("failed to write out-of-order migration file: {err}"));

    let cmd = surmig().args([
        "--config-dir",
        "tests/verify_cmd",
        "--db-address",
        &db_config.address,
        "--migrations-folder",
        &migrations_folder.to_string_lossy(),
        "verify",
    ]);

    cmd.assert()
        .code(0)
        .stdout_eq(snapbox::file!(
            "verify_cmd/one_migration_changed_one_migration_out_of_order.stdout"
        ))
        .stderr_eq("");
}

#[tokio::test]
async fn verify_migrations_checksum_only_one_migration_changed_one_migration_out_of_order() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;
    let db = connect_to_test_database_as_database_user(&db_config).await;

    let temp_dir = TempDir::new().unwrap_or_else(|err| panic!("could not create temp dir: {err}"));
    let migrations_folder = temp_dir.path();

    // copy migration files to temp folder
    let read_dir = fs::read_dir(Path::new("../fixtures/with_down_migrations/migrations"))
        .unwrap_or_else(|err| panic!("could not read migrations folder: {err}"));
    for dir_entry in read_dir.flatten() {
        let src_path = dir_entry.path();
        if src_path.is_file() {
            let filename = src_path.file_name().expect("src path has no filename");
            fs::copy(&src_path, migrations_folder.join(filename))
                .unwrap_or_else(|err| panic!("failed to copy migration file {src_path:?}: {err}"));
        }
    }

    // migrate database
    let runner_config = RunnerConfig::default().with_migrations_folder(migrations_folder);
    let runner = MigrationRunner::new(runner_config);
    let migrated = runner
        .migrate(&db)
        .await
        .unwrap_or_else(|err| panic!("failed to migrate database: {err}"));

    assert_that!(migrated).is_equal_to(Migrated::UpTo(key("20250103_141521")));

    // modify already applied migration script
    let script_path = migrations_folder.join("20250103_141521_create_some_quotes.surql");
    fs::write(&script_path, "")
        .unwrap_or_else(|err| panic!("failed to write out-of-order migration file: {err}"));

    // create new migration file out of order
    fs::write(
        migrations_folder.join("20250103_141030_out_of_order_migration.surql"),
        "",
    )
    .unwrap_or_else(|err| panic!("failed to write out-of-order migration file: {err}"));

    let cmd = surmig().args([
        "--config-dir",
        "tests/verify_cmd",
        "--db-address",
        &db_config.address,
        "--migrations-folder",
        &migrations_folder.to_string_lossy(),
        "verify",
        "--checksum",
    ]);

    cmd.assert()
        .code(0)
        .stdout_eq(snapbox::file!(
            "verify_cmd/checksum_only_one_migration_changed_one_migration_out_of_order.stdout"
        ))
        .stderr_eq("");
}

#[tokio::test]
async fn verify_migrations_order_only_one_migration_changed_one_migration_out_of_order() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;
    let db = connect_to_test_database_as_database_user(&db_config).await;

    let temp_dir = TempDir::new().unwrap_or_else(|err| panic!("could not create temp dir: {err}"));
    let migrations_folder = temp_dir.path();

    // copy migration files to temp folder
    let read_dir = fs::read_dir(Path::new("../fixtures/with_down_migrations/migrations"))
        .unwrap_or_else(|err| panic!("could not read migrations folder: {err}"));
    for dir_entry in read_dir.flatten() {
        let src_path = dir_entry.path();
        if src_path.is_file() {
            let filename = src_path.file_name().expect("src path has no filename");
            fs::copy(&src_path, migrations_folder.join(filename))
                .unwrap_or_else(|err| panic!("failed to copy migration file {src_path:?}: {err}"));
        }
    }

    // migrate database
    let runner_config = RunnerConfig::default().with_migrations_folder(migrations_folder);
    let runner = MigrationRunner::new(runner_config);
    let migrated = runner
        .migrate(&db)
        .await
        .unwrap_or_else(|err| panic!("failed to migrate database: {err}"));

    assert_that!(migrated).is_equal_to(Migrated::UpTo(key("20250103_141521")));

    // modify already applied migration script
    let script_path = migrations_folder.join("20250103_141521_create_some_quotes.surql");
    fs::write(&script_path, "")
        .unwrap_or_else(|err| panic!("failed to write out-of-order migration file: {err}"));

    // create new migration file out of order
    fs::write(
        migrations_folder.join("20250103_141030_out_of_order_migration.surql"),
        "",
    )
    .unwrap_or_else(|err| panic!("failed to write out-of-order migration file: {err}"));

    let cmd = surmig().args([
        "--config-dir",
        "tests/verify_cmd",
        "--db-address",
        &db_config.address,
        "--migrations-folder",
        &migrations_folder.to_string_lossy(),
        "verify",
        "--order",
    ]);

    cmd.assert()
        .code(0)
        .stdout_eq(snapbox::file!(
            "verify_cmd/order_only_one_migration_changed_one_migration_out_of_order.stdout"
        ))
        .stderr_eq("");
}
