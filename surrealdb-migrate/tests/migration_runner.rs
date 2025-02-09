mod fixtures;

use crate::fixtures::db::{
    connect_to_test_database_as_database_user, get_db_tables_info, prepare_test_database,
    start_surrealdb_testcontainer,
};
use assertor::*;
use database_migration::checksum::hash_migration_script;
use database_migration::config::DEFAULT_MIGRATIONS_TABLE;
use database_migration::migration::{Execution, Migration, MigrationKind};
use database_migration::result::{Migrated, Reverted};
use database_migration::test_dsl::{datetime, key};
use std::collections::HashMap;
use std::iter::once;
use std::path::Path;
use std::time::Duration;
use surrealdb_migrate::config::RunnerConfig;
use surrealdb_migrate::runner::MigrationRunner;
use surrealdb_migrate_db_client::insert_migration_execution;

#[tokio::test]
async fn list_applied_migrations_from_an_empty_database() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;
    let db = connect_to_test_database_as_database_user(&db_config).await;

    let config = RunnerConfig::default();
    let runner = MigrationRunner::new(config);

    let applied_migrations = runner
        .list_applied_migrations(&db)
        .await
        .expect("failed to query list of applied migrations");

    assert_that!(applied_migrations).is_empty();
}

#[tokio::test]
async fn list_applied_migrations_from_a_database_with_two_migrations_applied() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;
    let db = connect_to_test_database_as_database_user(&db_config).await;

    let migration1 = Migration {
        key: key("20250103_140520"),
        title: "define quote table".into(),
        kind: MigrationKind::Up,
        script_path: "../fixture/basic/migrations/20250103_140520_define_quote_table.surql".into(),
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
        script_path: "../fixture/basic/migrations/20250103_140521_create_some_quotes.surql".into(),
    };

    let checksum2 = hash_migration_script(&migration2, "");

    let execution2 = Execution {
        key: key("20250103_140521"),
        applied_rank: 2,
        applied_by: "tester".into(),
        applied_at: datetime("2025-01-20T09:10:20Z"),
        checksum: checksum2,
        execution_time: Duration::from_micros(42),
    };

    insert_migration_execution(migration2, execution2, DEFAULT_MIGRATIONS_TABLE, &db)
        .await
        .expect("failed to insert migration 2");

    let config = RunnerConfig::default();
    let runner = MigrationRunner::new(config);

    let applied_migrations = runner
        .list_applied_migrations(&db)
        .await
        .expect("failed to query list of applied migrations");

    assert_that!(applied_migrations).contains_exactly_in_order(vec![
        Execution {
            key: key("20250103_140520"),
            applied_rank: 1,
            applied_by: "tester".into(),
            applied_at: datetime("2025-01-20T09:10:19Z"),
            checksum: checksum1,
            execution_time: Duration::from_micros(256),
        },
        Execution {
            key: key("20250103_140521"),
            applied_rank: 2,
            applied_by: "tester".into(),
            applied_at: datetime("2025-01-20T09:10:20Z"),
            checksum: checksum2,
            execution_time: Duration::from_micros(42),
        },
    ]);
}

#[tokio::test]
async fn run_migrations_on_empty_db() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;
    let db = connect_to_test_database_as_database_user(&db_config).await;

    let config =
        RunnerConfig::default().with_migrations_folder(Path::new("../fixtures/basic/migrations"));
    let runner = MigrationRunner::new(config);

    let migrated = runner.migrate(&db).await.expect("failed to run migrations");

    assert_that!(migrated).is_equal_to(Migrated::UpTo(key("20250103_140521")));

    let tables_info = get_db_tables_info(&db).await;

    assert_that!(tables_info.keys())
        .contains_exactly(["migrations".to_string(), "quote".to_string()].iter());

    let quotes: Vec<HashMap<String, String>> = db
        .query("SELECT text FROM quote ORDER BY text")
        .await
        .expect("failed to query quotes")
        .take(0)
        .expect("did not get expected query result");

    assert_that!(quotes.iter().map(|row| &row["text"])).contains_exactly_in_order(
        [
            "Behind every great man is a woman rolling her eyes. - Jim Carrey".to_string(),
            "If you want a guarantee, buy a toaster. - Clint Eastwood".to_string(),
            "It takes considerable knowledge just to realize the extent of your own ignorance. - Thomas Sowell".to_string(),
            "don't seek happiness - create it".to_string(),
        ]
        .iter(),
    );
}

#[tokio::test]
async fn run_migrations_on_fully_migrated_db() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;
    let db = connect_to_test_database_as_database_user(&db_config).await;

    let config =
        RunnerConfig::default().with_migrations_folder(Path::new("../fixtures/basic/migrations"));
    let runner = MigrationRunner::new(config);

    runner.migrate(&db).await.expect("failed to run migrations");

    let tables_info = get_db_tables_info(&db).await;

    assert_that!(tables_info.keys())
        .contains_exactly([DEFAULT_MIGRATIONS_TABLE.to_string(), "quote".to_string()].iter());

    let migrated = runner.migrate(&db).await.expect("failed to run migrations");

    assert_that!(migrated).is_equal_to(Migrated::Nothing);

    let tables_info = get_db_tables_info(&db).await;

    assert_that!(tables_info.keys())
        .contains_exactly(["migrations".to_string(), "quote".to_string()].iter());

    let quotes: Vec<HashMap<String, String>> = db
        .query("SELECT text FROM quote ORDER BY text")
        .await
        .expect("failed to query quotes")
        .take(0)
        .expect("did not get expected query result");

    assert_that!(quotes.iter().map(|row| &row["text"])).contains_exactly_in_order(
        [
            "Behind every great man is a woman rolling her eyes. - Jim Carrey".to_string(),
            "If you want a guarantee, buy a toaster. - Clint Eastwood".to_string(),
            "It takes considerable knowledge just to realize the extent of your own ignorance. - Thomas Sowell".to_string(),
            "don't seek happiness - create it".to_string(),
        ]
        .iter(),
    );
}

#[tokio::test]
async fn migrate_an_empty_db_up_to_migration_20250103_140520() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;
    let db = connect_to_test_database_as_database_user(&db_config).await;

    let config =
        RunnerConfig::default().with_migrations_folder(Path::new("../fixtures/basic/migrations"));
    let runner = MigrationRunner::new(config);

    let migrated = runner
        .migrate_to(key("20250103_140520"), &db)
        .await
        .expect("failed to run migrations");

    assert_that!(migrated).is_equal_to(Migrated::UpTo(key("20250103_140520")));

    let tables_info = get_db_tables_info(&db).await;

    assert_that!(tables_info.keys())
        .contains_exactly(["migrations".to_string(), "quote".to_string()].iter());

    let quotes: Vec<HashMap<String, String>> = db
        .query("SELECT text FROM quote ORDER BY text")
        .await
        .expect("failed to query quotes")
        .take(0)
        .expect("did not get expected query result");

    assert_that!(quotes.iter().map(|row| &row["text"])).is_empty();
}

#[tokio::test]
async fn revert_migrations_on_fully_migrated_db() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;
    let db = connect_to_test_database_as_database_user(&db_config).await;

    let config = RunnerConfig::default()
        .with_migrations_folder(Path::new("../fixtures/with_down_migrations/migrations"));
    let runner = MigrationRunner::new(config);

    runner.migrate(&db).await.expect("failed to run migrations");

    let tables_info = get_db_tables_info(&db).await;

    assert_that!(tables_info.keys())
        .contains_exactly([DEFAULT_MIGRATIONS_TABLE.to_string(), "quote".to_string()].iter());

    let reverted = runner
        .revert(&db)
        .await
        .expect("failed to revert migrations");

    assert_that!(reverted).is_equal_to(Reverted::Completely);

    let tables_info = get_db_tables_info(&db).await;

    assert_that!(tables_info.keys()).contains_exactly(once(&DEFAULT_MIGRATIONS_TABLE.to_string()));
}

#[tokio::test]
async fn revert_migrations_on_empty_db() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;
    let db = connect_to_test_database_as_database_user(&db_config).await;

    let config = RunnerConfig::default()
        .with_migrations_folder(Path::new("../fixtures/with_down_migrations/migrations"));
    let runner = MigrationRunner::new(config);

    let tables_info = get_db_tables_info(&db).await;

    assert_that!(tables_info.keys()).is_empty();

    let reverted = runner
        .revert(&db)
        .await
        .expect("failed to revert migrations");

    assert_that!(reverted).is_equal_to(Reverted::Nothing);

    let tables_info = get_db_tables_info(&db).await;

    assert_that!(tables_info.keys()).is_empty();
}

#[tokio::test]
async fn revert_a_fully_migrated_db_down_to_migration_20250103_140520() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;
    let db = connect_to_test_database_as_database_user(&db_config).await;

    let config = RunnerConfig::default()
        .with_migrations_folder(Path::new("../fixtures/with_down_migrations/migrations"));
    let runner = MigrationRunner::new(config);

    runner.migrate(&db).await.expect("failed to run migrations");

    let tables_info = get_db_tables_info(&db).await;

    assert_that!(tables_info.keys())
        .contains_exactly([DEFAULT_MIGRATIONS_TABLE.to_string(), "quote".to_string()].iter());

    let reverted = runner
        .revert_to(key("20250103_140520"), &db)
        .await
        .expect("failed to revert migrations");

    assert_that!(reverted).is_equal_to(Reverted::DownTo(key("20250103_140520")));

    let tables_info = get_db_tables_info(&db).await;

    assert_that!(tables_info.keys())
        .contains_exactly([DEFAULT_MIGRATIONS_TABLE.to_string(), "quote".to_string()].iter());
}
