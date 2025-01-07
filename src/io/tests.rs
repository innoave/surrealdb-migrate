use super::*;
use crate::error::Error;
use crate::migration::{Migration, MigrationKind};
use crate::test_dsl::key;
use speculoos::prelude::*;
use std::path::Path;

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
                key: key("20250103_140520"),
                title: "define quote table".into(),
                kind: MigrationKind::Up,
                script_path: Path::new(
                    "fixtures/basic/migrations/20250103_140520_define_quote_table.surql",
                )
                .into(),
            }),
            Ok(Migration {
                key: key("20250103_140521"),
                title: "create some quotes".into(),
                kind: MigrationKind::Up,
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
                key: key("20250103_140520"),
                title: "define quote table".into(),
                kind: MigrationKind::Up,
                script_path: Path::new(
                    "fixtures/with_subdir/migrations/20250103_140520_define_quote_table.surql",
                )
                .into(),
            }),
            Ok(Migration {
                key: key("20250103_140521"),
                title: "create some quotes".into(),
                kind: MigrationKind::Up,
                script_path: Path::new(
                    "fixtures/with_subdir/migrations/20250103_140521_create_some_quotes.surql",
                )
                .into(),
            }),
        ]
        .iter(),
    );
}
