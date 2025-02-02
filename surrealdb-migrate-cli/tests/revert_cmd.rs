mod fixtures;

use crate::fixtures::db::{
    connect_to_test_database_as_database_user, prepare_test_database, start_surrealdb_testcontainer,
};
use crate::fixtures::surmig;
use database_migration::test_dsl::key;
use std::path::Path;
use surrealdb_migrate::config::RunnerConfig;
use surrealdb_migrate::runner::MigrationRunner;

#[tokio::test]
async fn revert_empty_database() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;

    let cmd = surmig().args([
        "--config-dir",
        "tests/revert_cmd",
        "--db-address",
        &db_config.address,
        "revert",
    ]);

    cmd.assert()
        .code(0)
        .stdout_eq(snapbox::file!("revert_cmd/empty_database.stdout"))
        .stderr_eq("");
}

#[tokio::test]
async fn revert_fully_migrated_database() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;
    let db = connect_to_test_database_as_database_user(&db_config).await;

    let runner_config = RunnerConfig::default()
        .with_migrations_folder(Path::new("../fixtures/with_down_migrations/migrations"));
    let runner = MigrationRunner::new(runner_config);

    runner
        .migrate(&db)
        .await
        .expect("failed to migrate database");

    let cmd = surmig().args([
        "--config-dir",
        "tests/revert_cmd",
        "--db-address",
        &db_config.address,
        "revert",
    ]);

    cmd.assert()
        .code(0)
        .stdout_eq(snapbox::file!("revert_cmd/fully_migrated_database.stdout"))
        .stderr_eq("");
}

#[tokio::test]
async fn revert_partially_migrated_database() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;
    let db = connect_to_test_database_as_database_user(&db_config).await;

    let runner_config = RunnerConfig::default()
        .with_migrations_folder(Path::new("../fixtures/with_down_migrations/migrations"));
    let runner = MigrationRunner::new(runner_config);

    runner
        .migrate_to(key("20250103_140520"), &db)
        .await
        .expect("failed to migrate database");

    let cmd = surmig().args([
        "--config-dir",
        "tests/revert_cmd",
        "--db-address",
        &db_config.address,
        "revert",
    ]);

    cmd.assert()
        .code(0)
        .stdout_eq(snapbox::file!(
            "revert_cmd/partially_migrated_database.stdout"
        ))
        .stderr_eq("");
}

#[tokio::test]
async fn revert_fully_migrated_database_down_to_20250103_140520() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;
    let db = connect_to_test_database_as_database_user(&db_config).await;

    let runner_config = RunnerConfig::default()
        .with_migrations_folder(Path::new("../fixtures/with_down_migrations/migrations"));
    let runner = MigrationRunner::new(runner_config);

    runner
        .migrate(&db)
        .await
        .expect("failed to migrate database");

    let cmd = surmig().args([
        "--config-dir",
        "tests/revert_cmd",
        "--db-address",
        &db_config.address,
        "revert",
        "--to",
        "20250103_140520",
    ]);

    cmd.assert()
        .code(0)
        .stdout_eq(snapbox::file!(
            "revert_cmd/fully_migrated_database_down_to_20250103_140520.stdout"
        ))
        .stderr_eq("");
}

#[tokio::test]
async fn revert_fully_migrated_database_down_to_missing_key() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;
    let db = connect_to_test_database_as_database_user(&db_config).await;

    let runner_config = RunnerConfig::default()
        .with_migrations_folder(Path::new("../fixtures/with_down_migrations/migrations"));
    let runner = MigrationRunner::new(runner_config);

    runner
        .migrate(&db)
        .await
        .expect("failed to migrate database");

    let cmd = surmig().args([
        "--config-dir",
        "tests/revert_cmd",
        "--db-address",
        &db_config.address,
        "revert",
        "--to",
    ]);

    cmd.assert().code(2).stdout_eq("").stderr_eq(snapbox::file!(
        "revert_cmd/fully_migrated_database_down_to_missing_key.stderr"
    ));
}

#[tokio::test]
async fn revert_fully_migrated_database_down_to_empty_key() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;
    let db = connect_to_test_database_as_database_user(&db_config).await;

    let runner_config = RunnerConfig::default()
        .with_migrations_folder(Path::new("../fixtures/with_down_migrations/migrations"));
    let runner = MigrationRunner::new(runner_config);

    runner
        .migrate(&db)
        .await
        .expect("failed to migrate database");

    let cmd = surmig().args([
        "--config-dir",
        "tests/revert_cmd",
        "--db-address",
        &db_config.address,
        "revert",
        "--to",
        "",
    ]);

    cmd.assert().code(1).stdout_eq("").stderr_eq(snapbox::file!(
        "revert_cmd/fully_migrated_database_down_to_empty_key.stderr"
    ));
}

#[tokio::test]
async fn revert_fully_migrated_database_down_to_invalid_key() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;
    let db = connect_to_test_database_as_database_user(&db_config).await;

    let runner_config = RunnerConfig::default()
        .with_migrations_folder(Path::new("../fixtures/with_down_migrations/migrations"));
    let runner = MigrationRunner::new(runner_config);

    runner
        .migrate(&db)
        .await
        .expect("failed to migrate database");

    let cmd = surmig().args([
        "--config-dir",
        "tests/revert_cmd",
        "--db-address",
        &db_config.address,
        "revert",
        "--to",
        "V0101",
    ]);

    cmd.assert().code(1).stdout_eq("").stderr_eq(snapbox::file!(
        "revert_cmd/fully_migrated_database_down_to_invalid_key.stderr"
    ));
}
