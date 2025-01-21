use super::*;
use assert_fs::TempDir;
use assertor::*;
use database_migration::definition::MigrationFilenameStrategy;
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

#[test]
fn create_migrations_folder_if_not_existing_folder_does_not_exist() {
    let parent_dir = TempDir::new().expect("failed to create temp dir");

    let migrations_folder =
        create_migrations_folder_if_not_existing(parent_dir.path(), "migrations");

    assert_that!(migrations_folder).is_equal_to(Ok(parent_dir.path().join("migrations")));
}

#[test]
fn create_migrations_folder_if_not_existing_folder_already_exists() {
    let parent_dir = TempDir::new().expect("failed to create temp dir");
    let expected_folder = parent_dir.path().join("my_migrations");
    fs::create_dir(&expected_folder).expect("failed to create existing migrations folder");

    let migrations_folder =
        create_migrations_folder_if_not_existing(parent_dir.path(), "my_migrations");

    assert_that!(migrations_folder).is_equal_to(Ok(expected_folder));
}

#[test]
fn create_migrations_folder_if_not_existing_parent_folder_does_not_exist() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let parent_dir = temp_dir.path().join("package_dir");

    let migrations_folder =
        create_migrations_folder_if_not_existing(&parent_dir, "script_migrations");

    assert_that!(migrations_folder).is_equal_to(Ok(parent_dir.join("script_migrations")));
}

#[test]
fn create_migration_file_for_new_migration() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");

    let filename_strategy = MigrationFilenameStrategy::default();

    let new_migration = NewMigration {
        key: key("20250115_201642"),
        title: "create some table".to_string(),
        kind: MigrationKind::Up,
    };

    let migration_file = create_migration_file(&filename_strategy, temp_dir.path(), &new_migration);

    assert_that!(migration_file).is_equal_to(Ok(temp_dir
        .path()
        .join("20250115_201642_create_some_table.up.surql")));
    assert_that!(migration_file.expect("migration file not created").exists()).is_true();
}

#[test]
fn create_migration_file_for_new_migration_file_already_existing() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");

    let filename_strategy = MigrationFilenameStrategy::default();

    let new_migration = NewMigration {
        key: key("20250115_201642"),
        title: "create some table".to_string(),
        kind: MigrationKind::Up,
    };

    let existing_file = create_migration_file(&filename_strategy, temp_dir.path(), &new_migration);

    assert_that!(existing_file).is_equal_to(Ok(temp_dir
        .path()
        .join("20250115_201642_create_some_table.up.surql")));
    assert_that!(existing_file.expect("existing file not created").exists()).is_true();

    let result = create_migration_file(&filename_strategy, temp_dir.path(), &new_migration);

    assert_that!(matches!(result, Err(Error::CreatingScriptFile(_)))).is_true();
}

#[test]
fn create_migration_file_for_new_migration_folder_does_not_exist() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let folder = temp_dir.path().join("not_existing");

    let filename_strategy = MigrationFilenameStrategy::default();

    let new_migration = NewMigration {
        key: key("20250115_201642"),
        title: "create some table".to_string(),
        kind: MigrationKind::Up,
    };

    let result = create_migration_file(&filename_strategy, &folder, &new_migration);

    assert_that!(matches!(result, Err(Error::CreatingScriptFile(_)))).is_true();
}
