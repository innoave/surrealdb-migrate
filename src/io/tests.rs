use super::*;
use crate::migration::{Direction, Migration};
use crate::Error;
use speculoos::prelude::*;
use std::path::Path;
use time::macros::datetime;

#[test]
fn list_all_migrations_in_basic_migrations_dir() {
    let migration_directory = migration_directory("fixtures/basic/migrations");

    let migrations = migration_directory
        .list_all_migrations()
        .expect("failed to scan migration directory")
        .collect::<Vec<_>>();

    assert_that!(migrations).equals_iterator(
        &[
            Ok(Migration {
                id: datetime!(2025-01-03 14:05:20),
                title: "define_quote_table".to_string(),
                direction: Direction::Up,
                script_path: Path::new(
                    "fixtures/basic/migrations/20250103_140520_define_quote_table.surql",
                )
                .into(),
            }),
            Ok(Migration {
                id: datetime!(2025-01-03 14:05:21),
                title: "create_some_quotes".to_string(),
                direction: Direction::Up,
                script_path: Path::new(
                    "fixtures/basic/migrations/20250103_140521_create_some_quotes.surql",
                )
                .into(),
            }),
        ]
        .iter(),
    );
}

#[test]
fn list_all_migrations_in_non_existing_directory() {
    let migration_directory = migration_directory("fixtures/not_existing/migrations");

    let migrations = migration_directory.list_all_migrations();

    assert_that!(migrations)
        .is_err()
        .matches(|err| matches!(err, Error::ScanningMigrationDirectory(_)));
}

#[test]
fn list_all_migrations_in_migrations_dir_with_subdirectory() {
    let migration_directory = migration_directory("fixtures/with_subdir/migrations");

    let migrations = migration_directory
        .list_all_migrations()
        .expect("failed to scan migration directory")
        .collect::<Vec<_>>();

    assert_that!(migrations).equals_iterator(
        &[
            Ok(Migration {
                id: datetime!(2025-01-03 14:05:20),
                title: "define_quote_table".to_string(),
                direction: Direction::Up,
                script_path: Path::new(
                    "fixtures/with_subdir/migrations/20250103_140520_define_quote_table.surql",
                )
                .into(),
            }),
            Ok(Migration {
                id: datetime!(2025-01-03 14:05:21),
                title: "create_some_quotes".to_string(),
                direction: Direction::Up,
                script_path: Path::new(
                    "fixtures/with_subdir/migrations/20250103_140521_create_some_quotes.surql",
                )
                .into(),
            }),
        ]
        .iter(),
    );
}
