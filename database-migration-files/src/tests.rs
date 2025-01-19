use super::*;
use assertor::*;
use database_migration::error::Error;
use database_migration::migration::{Migration, MigrationKind};
use database_migration::test_dsl::key;
use std::path::Path;

const BASIC_MIGRATION_CONTENT1: &str =
    include_str!("../../fixtures/basic/migrations/20250103_140520_define_quote_table.surql");
const BASIC_MIGRATION_CONTENT2: &str =
    include_str!("../../fixtures/basic/migrations/20250103_140521_create_some_quotes.surql");

#[test]
fn list_all_migrations_in_basic_migrations_dir() {
    let migration_directory = migration_directory("../fixtures/basic/migrations");

    let migrations = migration_directory
        .list_all_migrations()
        .expect("failed to scan migration directory")
        .collect::<Vec<_>>();

    assert_that!(migrations).contains_exactly_in_order(vec![
        Ok(Migration {
            key: key("20250103_140520"),
            title: "define quote table".into(),
            kind: MigrationKind::Up,
            script_path: Path::new(
                "../fixtures/basic/migrations/20250103_140520_define_quote_table.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250103_140521"),
            title: "create some quotes".into(),
            kind: MigrationKind::Up,
            script_path: Path::new(
                "../fixtures/basic/migrations/20250103_140521_create_some_quotes.surql",
            )
            .into(),
        }),
    ]);
}

#[test]
fn list_all_migrations_in_non_existing_directory() {
    let migration_directory = migration_directory("../fixtures/not_existing/migrations");

    let migrations = migration_directory.list_all_migrations();

    assert_that!(matches!(
        migrations,
        Err(Error::ScanningMigrationDirectory(_))
    ))
    .is_true();
}

#[test]
fn list_all_migrations_in_migrations_dir_with_subdirectory() {
    let migration_directory = migration_directory("../fixtures/with_subdir/migrations");

    let migrations = migration_directory
        .list_all_migrations()
        .expect("failed to scan migration directory")
        .collect::<Vec<_>>();

    assert_that!(migrations).contains_exactly_in_order(vec![
        Ok(Migration {
            key: key("20250103_140520"),
            title: "define quote table".into(),
            kind: MigrationKind::Up,
            script_path: Path::new(
                "../fixtures/with_subdir/migrations/20250103_140520_define_quote_table.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250103_140521"),
            title: "create some quotes".into(),
            kind: MigrationKind::Up,
            script_path: Path::new(
                "../fixtures/with_subdir/migrations/20250103_140521_create_some_quotes.surql",
            )
            .into(),
        }),
    ]);
}

#[test]
fn read_script_content_for_basic_migrations() {
    let migrations_folder = Path::new("../fixtures/basic/migrations");

    let migrations = &[
        Migration {
            key: key("20250103_140520"),
            title: "define quote table".into(),
            kind: MigrationKind::Up,
            script_path: migrations_folder.join("20250103_140520_define_quote_table.surql"),
        },
        Migration {
            key: key("20250103_140521"),
            title: "create come quotes".into(),
            kind: MigrationKind::Up,
            script_path: migrations_folder.join("20250103_140521_create_some_quotes.surql"),
        },
    ];
    let checksum1 = hash_migration_script(&migrations[0], BASIC_MIGRATION_CONTENT1);
    let checksum2 = hash_migration_script(&migrations[1], BASIC_MIGRATION_CONTENT2);

    let script_contents =
        read_script_content_for_migrations(migrations).expect("failed to read script content");

    assert_that!(script_contents).contains_exactly_in_order(vec![
        ScriptContent {
            key: key("20250103_140520"),
            kind: MigrationKind::Up,
            path: migrations_folder.join("20250103_140520_define_quote_table.surql"),
            content: BASIC_MIGRATION_CONTENT1.into(),
            checksum: checksum1,
        },
        ScriptContent {
            key: key("20250103_140521"),
            kind: MigrationKind::Up,
            path: migrations_folder.join("20250103_140521_create_some_quotes.surql"),
            content: BASIC_MIGRATION_CONTENT2.into(),
            checksum: checksum2,
        },
    ]);
}

#[test]
fn read_script_content_for_non_existing_migration() {
    let migrations_folder = Path::new("../fixtures/basic/migrations");

    let migrations = &[Migration {
        key: key("20250103_140520"),
        title: "non existing".into(),
        kind: MigrationKind::Up,
        script_path: migrations_folder.join("20250103_140520_non_existing.surql"),
    }];

    let result = read_script_content_for_migrations(migrations);

    assert_that!(matches!(result, Err(Error::ReadingMigrationFile(_)))).is_true();
}
