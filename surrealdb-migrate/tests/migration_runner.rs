mod fixtures;

use crate::fixtures::db::{
    connect_to_test_database_as_database_user, get_db_tables_info, prepare_test_database,
    start_surrealdb_testcontainer,
};
use assert_fs::TempDir;
use asserting::prelude::*;
use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::read_to_string;
use std::path::Path;
use std::time::Duration;
use surrealdb_migrate::checksum::hash_migration_script;
use surrealdb_migrate::config::DEFAULT_MIGRATIONS_TABLE;
use surrealdb_migrate::config::RunnerConfig;
use surrealdb_migrate::migration::{Execution, Migration, MigrationKind, Problem};
use surrealdb_migrate::result::{Migrated, Reverted, Verified};
use surrealdb_migrate::runner::MigrationRunner;
use surrealdb_migrate::test_dsl::{datetime, key};
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

    assert_that!(applied_migrations).contains_exactly([
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

    assert_that!(tables_info.keys()).contains_exactly_in_any_order(["migrations", "quote"]);

    let quotes: Vec<HashMap<String, String>> = db
        .query("SELECT text FROM quote ORDER BY text")
        .await
        .expect("failed to query quotes")
        .take(0)
        .expect("did not get expected query result");

    assert_that!(quotes.iter().map(|row| &row["text"])).contains_exactly_in_any_order(
        [
            "Behind every great man is a woman rolling her eyes. - Jim Carrey",
            "If you want a guarantee, buy a toaster. - Clint Eastwood",
            "It takes considerable knowledge just to realize the extent of your own ignorance. - Thomas Sowell",
            "don't seek happiness - create it",
        ],
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
        .contains_exactly_in_any_order([DEFAULT_MIGRATIONS_TABLE, "quote"]);

    let migrated = runner.migrate(&db).await.expect("failed to run migrations");

    assert_that!(migrated).is_equal_to(Migrated::Nothing);

    let tables_info = get_db_tables_info(&db).await;

    assert_that!(tables_info.keys()).contains_exactly_in_any_order(["migrations", "quote"]);

    let quotes: Vec<HashMap<String, String>> = db
        .query("SELECT text FROM quote ORDER BY text")
        .await
        .expect("failed to query quotes")
        .take(0)
        .expect("did not get expected query result");

    assert_that!(quotes.iter().map(|row| &row["text"])).contains_exactly_in_any_order(
        [
            "Behind every great man is a woman rolling her eyes. - Jim Carrey",
            "If you want a guarantee, buy a toaster. - Clint Eastwood",
            "It takes considerable knowledge just to realize the extent of your own ignorance. - Thomas Sowell",
            "don't seek happiness - create it",
        ],
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

    assert_that!(tables_info.keys()).contains_exactly_in_any_order(["migrations", "quote"]);

    let quotes: Vec<HashMap<String, String>> = db
        .query("SELECT text FROM quote ORDER BY text")
        .await
        .expect("failed to query quotes")
        .take(0)
        .expect("did not get expected query result");

    assert_that!(quotes.iter().map(|row| &row["text"]).next()).is_none();
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
        .contains_exactly_in_any_order([DEFAULT_MIGRATIONS_TABLE, "quote"]);

    let reverted = runner
        .revert(&db)
        .await
        .expect("failed to revert migrations");

    assert_that!(reverted).is_equal_to(Reverted::Completely);

    let tables_info = get_db_tables_info(&db).await;

    assert_that!(tables_info.keys()).contains_exactly_in_any_order([DEFAULT_MIGRATIONS_TABLE]);
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

    assert_that!(tables_info.keys().next()).is_none();

    let reverted = runner
        .revert(&db)
        .await
        .expect("failed to revert migrations");

    assert_that!(reverted).is_equal_to(Reverted::Nothing);

    let tables_info = get_db_tables_info(&db).await;

    assert_that!(tables_info.keys().next()).is_none();
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
        .contains_exactly_in_any_order([DEFAULT_MIGRATIONS_TABLE, "quote"]);

    let reverted = runner
        .revert_to(key("20250103_140520"), &db)
        .await
        .expect("failed to revert migrations");

    assert_that!(reverted).is_equal_to(Reverted::DownTo(key("20250103_140520")));

    let tables_info = get_db_tables_info(&db).await;

    assert_that!(tables_info.keys())
        .contains_exactly_in_any_order([DEFAULT_MIGRATIONS_TABLE, "quote"]);
}

#[tokio::test]
async fn verify_empty_database_no_migrations_in_folder() {
    let temp_dir = TempDir::new().unwrap_or_else(|err| panic!("could not create temp dir: {err}"));
    let migrations_folder = temp_dir.path();

    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;
    let db = connect_to_test_database_as_database_user(&db_config).await;

    let config = RunnerConfig::default().with_migrations_folder(migrations_folder);
    let runner = MigrationRunner::new(config);

    let result = runner.verify(&db).await;

    assert_that!(result)
        .ok()
        .is_equal_to(Verified::NoMigrationsFound);
}

#[tokio::test]
async fn verify_empty_database() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;
    let db = connect_to_test_database_as_database_user(&db_config).await;

    let config = RunnerConfig::default()
        .with_migrations_folder(Path::new("../fixtures/with_down_migrations/migrations"));
    let runner = MigrationRunner::new(config);

    let result = runner.verify(&db).await;

    assert_that!(result)
        .ok()
        .is_equal_to(Verified::NoProblemsFound);
}

#[tokio::test]
async fn verify_fully_migrated_database_no_problems() {
    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;
    let db = connect_to_test_database_as_database_user(&db_config).await;

    let config = RunnerConfig::default()
        .with_migrations_folder(Path::new("../fixtures/with_down_migrations/migrations"));
    let runner = MigrationRunner::new(config);

    let migrated = runner
        .migrate(&db)
        .await
        .unwrap_or_else(|err| panic!("failed to run migrations: {err}"));

    assert_that!(migrated).is_equal_to(Migrated::UpTo(key("20250103_141521")));

    let result = runner.verify(&db).await;

    assert_that!(result)
        .ok()
        .is_equal_to(Verified::NoProblemsFound);
}

#[tokio::test]
async fn verify_fully_migrated_database_one_migration_out_of_order() {
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

    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;
    let db = connect_to_test_database_as_database_user(&db_config).await;

    let config = RunnerConfig::default().with_migrations_folder(migrations_folder);
    let runner = MigrationRunner::new(config);

    let migrated = runner
        .migrate(&db)
        .await
        .unwrap_or_else(|err| panic!("failed to run migrations: {err}"));

    assert_that!(migrated).is_equal_to(Migrated::UpTo(key("20250103_141521")));

    // create new migration file out of order
    fs::write(
        migrations_folder.join("20250103_141030_out_of_order_migration.surql"),
        "",
    )
    .unwrap_or_else(|err| panic!("failed to write out-of-order migration file: {err}"));

    let result = runner.verify(&db).await;

    if let Ok(Verified::FoundProblems(problems)) = result {
        assert_that!(&problems[0].problem).is_equal_to(&Problem::OutOfOrder {
            last_applied_key: key("20250103_141521"),
        });
        assert_that!(problems[0].key).is_equal_to(key("20250103_141030"));
        assert_that!(problems[0].kind).is_equal_to(MigrationKind::Up);
        assert_that!(problems[0].script_path.file_name().and_then(OsStr::to_str))
            .is_equal_to(Some("20250103_141030_out_of_order_migration.surql"));
    } else {
        panic!("expected Ok(Verified::FoundProblems), but got {result:?}");
    }
}

#[tokio::test]
async fn verify_fully_migrated_database_one_migration_changed() {
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
    let script_path = migrations_folder.join("20250103_141521_create_some_quotes.surql");
    let execution_checksum = hash_migration_script(
        &Migration {
            key: key("20250103_141521"),
            title: "create some quotes".into(),
            kind: MigrationKind::Up,
            script_path: script_path.clone(),
        },
        &read_to_string(
            File::open(&script_path)
                .unwrap_or_else(|err| panic!("could not open migration script: {err}")),
        )
        .unwrap_or_else(|err| panic!("failed to read contents of migration script: {err}")),
    );

    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;
    let db = connect_to_test_database_as_database_user(&db_config).await;

    let config = RunnerConfig::default().with_migrations_folder(migrations_folder);
    let runner = MigrationRunner::new(config);

    let migrated = runner
        .migrate(&db)
        .await
        .unwrap_or_else(|err| panic!("failed to run migrations: {err}"));

    assert_that!(migrated).is_equal_to(Migrated::UpTo(key("20250103_141521")));

    // modify already applied migration script
    fs::write(&script_path, "")
        .unwrap_or_else(|err| panic!("failed to write out-of-order migration file: {err}"));

    let definition_checksum = hash_migration_script(
        &Migration {
            key: key("20250103_141521"),
            title: "create some quotes".into(),
            kind: MigrationKind::Up,
            script_path: script_path.clone(),
        },
        "",
    );

    let result = runner.verify(&db).await;

    if let Ok(Verified::FoundProblems(problems)) = result {
        assert_that!(&problems[0].problem).is_equal_to(&Problem::ChecksumMismatch {
            definition_checksum,
            execution_checksum,
        });
        assert_that!(problems[0].key).is_equal_to(key("20250103_141521"));
        assert_that!(problems[0].kind).is_equal_to(MigrationKind::Up);
        assert_that!(problems[0].script_path.file_name().and_then(OsStr::to_str))
            .is_equal_to(Some("20250103_141521_create_some_quotes.surql"));
    } else {
        panic!("expected Ok(Verified::FoundProblems), but got {result:?}");
    }
}

#[tokio::test]
async fn verify_fully_migrated_database_one_migration_changed_and_one_out_of_order_migration() {
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
    let script_path = migrations_folder.join("20250103_141521_create_some_quotes.surql");
    let execution_checksum = hash_migration_script(
        &Migration {
            key: key("20250103_141521"),
            title: "create some quotes".into(),
            kind: MigrationKind::Up,
            script_path: script_path.clone(),
        },
        &read_to_string(
            File::open(&script_path)
                .unwrap_or_else(|err| panic!("could not open migration script: {err}")),
        )
        .unwrap_or_else(|err| panic!("failed to read contents of migration script: {err}")),
    );

    let db_server = start_surrealdb_testcontainer().await;
    let db_config = prepare_test_database(&db_server).await;
    let db = connect_to_test_database_as_database_user(&db_config).await;

    let config = RunnerConfig::default().with_migrations_folder(migrations_folder);
    let runner = MigrationRunner::new(config);

    let migrated = runner
        .migrate(&db)
        .await
        .unwrap_or_else(|err| panic!("failed to run migrations: {err}"));

    assert_that!(migrated).is_equal_to(Migrated::UpTo(key("20250103_141521")));

    // modify already applied migration script
    fs::write(&script_path, "")
        .unwrap_or_else(|err| panic!("failed to write out-of-order migration file: {err}"));

    // create new migration file out of order
    fs::write(
        migrations_folder.join("20250103_141030_out_of_order_migration.surql"),
        "",
    )
    .unwrap_or_else(|err| panic!("failed to write out-of-order migration file: {err}"));

    let definition_checksum = hash_migration_script(
        &Migration {
            key: key("20250103_141521"),
            title: "create some quotes".into(),
            kind: MigrationKind::Up,
            script_path: script_path.clone(),
        },
        "",
    );

    let result = runner.verify(&db).await;

    if let Ok(Verified::FoundProblems(problems)) = result {
        assert_that!(&problems[0].problem).is_equal_to(&Problem::OutOfOrder {
            last_applied_key: key("20250103_141521"),
        });
        assert_that!(problems[0].key).is_equal_to(key("20250103_141030"));
        assert_that!(problems[0].kind).is_equal_to(MigrationKind::Up);
        assert_that!(problems[0].script_path.file_name().and_then(OsStr::to_str))
            .is_equal_to(Some("20250103_141030_out_of_order_migration.surql"));

        assert_that!(&problems[1].problem).is_equal_to(&Problem::ChecksumMismatch {
            definition_checksum,
            execution_checksum,
        });
        assert_that!(problems[1].key).is_equal_to(key("20250103_141521"));
        assert_that!(problems[1].kind).is_equal_to(MigrationKind::Up);
        assert_that!(problems[1].script_path.file_name().and_then(OsStr::to_str))
            .is_equal_to(Some("20250103_141521_create_some_quotes.surql"));
    } else {
        panic!("expected Ok(Verified::FoundProblems), but got {result:?}");
    }
}
