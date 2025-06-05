#![allow(clippy::manual_string_new)]

use super::*;
use assert_fs::TempDir;
use assertor::*;
use database_migration::definition::MigrationFilenameStrategy;
use database_migration::error::{DefinitionError, Error};
use database_migration::migration::{Migration, MigrationKind};
use database_migration::test_dsl::key;
use std::path::Path;

const BASIC_MIGRATION_CONTENT1: &str =
    include_str!("../../fixtures/basic/migrations/20250103_140520_define_quote_table.surql");
const BASIC_MIGRATION_CONTENT2: &str =
    include_str!("../../fixtures/basic/migrations/20250103_140521_create_some_quotes.surql");

#[test]
fn list_all_migrations_in_basic_migrations_dir() {
    let excluded_files = ExcludedFiles::empty();
    let migration_directory =
        MigrationDirectory::new(Path::new("../fixtures/basic/migrations"), &excluded_files);

    let migrations = migration_directory
        .list_all_migrations()
        .expect("failed to scan migration directory")
        .collect::<Vec<_>>();

    assert_that!(migrations).contains_exactly(vec![
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
    let excluded_files = ExcludedFiles::empty();
    let migration_directory = MigrationDirectory::new(
        Path::new("../fixtures/not_existing/migrations"),
        &excluded_files,
    );

    let migrations = migration_directory.list_all_migrations();

    assert_that!(matches!(
        migrations,
        Err(Error::ScanningMigrationDirectory(_))
    ))
    .is_true();
}

#[test]
fn list_all_migrations_in_migrations_dir_with_subdirectory() {
    let excluded_files = ExcludedFiles::empty();
    let migration_directory = MigrationDirectory::new(
        Path::new("../fixtures/with_subdir/migrations"),
        &excluded_files,
    );

    let migrations = migration_directory
        .list_all_migrations()
        .expect("failed to scan migration directory")
        .collect::<Vec<_>>();

    assert_that!(migrations).contains_exactly(vec![
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
fn list_migrations_ignores_configured_filenames_empty_pattern_dot_keep_file() {
    let temp_dir = TempDir::new().unwrap_or_else(|err| panic!("could not create temp dir: {err}"));
    let migrations_folder = temp_dir.path();

    fs::write(migrations_folder.join(".keep"), "")
        .unwrap_or_else(|err| panic!("could not write .keep file: {err}"));

    let excluded_files = ExcludedFiles::empty();
    let migration_directory = MigrationDirectory::new(migrations_folder, &excluded_files);

    let migrations = migration_directory
        .list_all_migrations()
        .unwrap_or_else(|err| panic!("failed to list all migrations: {err}"))
        .collect::<Vec<_>>();

    assert_that!(migrations).contains_exactly(vec![Err(Error::Definition(
        DefinitionError::InvalidFilename,
    ))]);
}

#[test]
fn list_migrations_ignores_configured_filenames_default_pattern_dot_keep_file() {
    let temp_dir = TempDir::new().unwrap_or_else(|err| panic!("could not create temp dir: {err}"));
    let migrations_folder = temp_dir.path();

    fs::write(migrations_folder.join(".keep"), "")
        .unwrap_or_else(|err| panic!("could not write .keep file: {err}"));

    let excluded_files = ExcludedFiles::default();
    let migration_directory = MigrationDirectory::new(migrations_folder, &excluded_files);

    let migrations = migration_directory
        .list_all_migrations()
        .unwrap_or_else(|err| panic!("failed to list all migrations: {err}"))
        .collect::<Vec<_>>();

    assert_that!(migrations).is_empty();
}

#[test]
fn list_migrations_ignores_configured_filenames_default_pattern_readme_md_file() {
    let temp_dir = TempDir::new().unwrap_or_else(|err| panic!("could not create temp dir: {err}"));
    let migrations_folder = temp_dir.path();

    fs::write(migrations_folder.join("README.md"), "")
        .unwrap_or_else(|err| panic!("could not write .keep file: {err}"));

    let excluded_files = ExcludedFiles::default();
    let migration_directory = MigrationDirectory::new(migrations_folder, &excluded_files);

    let migrations = migration_directory
        .list_all_migrations()
        .unwrap_or_else(|err| panic!("failed to list all migrations: {err}"))
        .collect::<Vec<_>>();

    assert_that!(migrations).is_empty();
}

#[test]
fn list_migrations_ignores_configured_filenames_default_pattern_todo_txt_file() {
    let temp_dir = TempDir::new().unwrap_or_else(|err| panic!("could not create temp dir: {err}"));
    let migrations_folder = temp_dir.path();

    fs::write(migrations_folder.join("TODO.txt"), "")
        .unwrap_or_else(|err| panic!("could not write .keep file: {err}"));

    let excluded_files = ExcludedFiles::default();
    let migration_directory = MigrationDirectory::new(migrations_folder, &excluded_files);

    let migrations = migration_directory
        .list_all_migrations()
        .unwrap_or_else(|err| panic!("failed to list all migrations: {err}"))
        .collect::<Vec<_>>();

    assert_that!(migrations).is_empty();
}

#[test]
fn read_script_content_for_basic_migrations() {
    let migrations_folder = Path::new("../fixtures/basic/migrations");
    let excluded_files = ExcludedFiles::empty();
    let migration_directory = MigrationDirectory::new(migrations_folder, &excluded_files);

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

    let script_contents = migration_directory
        .read_script_content_for_migrations(migrations)
        .expect("failed to read script content");

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
    let excluded_files = ExcludedFiles::empty();
    let migration_directory = MigrationDirectory::new(migrations_folder, &excluded_files);

    let migrations = Migration {
        key: key("20250103_140520"),
        title: "non existing".into(),
        kind: MigrationKind::Up,
        script_path: migrations_folder.join("20250103_140520_non_existing.surql"),
    };

    let result = migration_directory.read_script_content(&migrations);

    assert_that!(matches!(result, Err(Error::ReadingMigrationFile(_)))).is_true();
}

#[test]
fn create_migrations_folder_if_not_existing_folder_does_not_exist() {
    let parent_dir = TempDir::new().expect("failed to create temp dir");
    let migrations_folder = parent_dir.join("migrations");

    let excluded_files = ExcludedFiles::empty();
    let migration_directory = MigrationDirectory::new(&migrations_folder, &excluded_files);
    let result = migration_directory.create_directory_if_not_existing();

    assert_that!(result).is_ok();
    assert_that!(migration_directory.path.exists()).is_true();
}

#[test]
fn create_migrations_folder_if_not_existing_folder_already_exists() {
    let parent_dir = TempDir::new().expect("failed to create temp dir");
    let expected_folder = parent_dir.path().join("my_migrations");
    fs::create_dir(&expected_folder).expect("failed to create existing migrations folder");
    let migrations_folder = parent_dir.join("my_migrations");

    let excluded_files = ExcludedFiles::empty();
    let migration_directory = MigrationDirectory::new(&migrations_folder, &excluded_files);
    let result = migration_directory.create_directory_if_not_existing();

    assert_that!(result).is_ok();
    assert_that!(migration_directory.path.exists()).is_true();
}

#[test]
fn create_migrations_folder_if_not_existing_parent_folder_does_not_exist() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let parent_dir = temp_dir.path().join("package_dir");
    let migrations_folder = parent_dir.join("script_migrations");

    let excluded_files = ExcludedFiles::empty();
    let migration_directory = MigrationDirectory::new(&migrations_folder, &excluded_files);
    let result = migration_directory.create_directory_if_not_existing();

    assert_that!(result).is_ok();
    assert_that!(migration_directory.path.exists()).is_true();
}

#[test]
fn get_migration_files_from_migrations_directory() {
    let migrations_folder = Path::new("../fixtures/basic/migrations");
    let excluded_files = ExcludedFiles::empty();
    let migration_directory = MigrationDirectory::new(migrations_folder, &excluded_files);

    let filename_strategy = MigrationFilenameStrategy::default();
    let migration_files = migration_directory.files(filename_strategy);

    assert_that!(migration_files).is_equal_to(MigrationFiles {
        path: migrations_folder,
        filename_strategy,
    });
}

#[test]
fn create_migration_file_for_new_migration() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let migrations_folder = temp_dir.path();

    let filename_strategy = MigrationFilenameStrategy::default();
    let migration_files = MigrationFiles::new(migrations_folder, filename_strategy);

    let new_migration = NewMigration {
        key: key("20250115_201642"),
        title: "create some table".into(),
        kind: MigrationKind::Up,
    };

    let migration = migration_files.create_new_migration(new_migration);

    assert_that!(migration).is_equal_to(Ok(Migration {
        key: key("20250115_201642"),
        title: "create some table".into(),
        kind: MigrationKind::Up,
        script_path: migrations_folder.join("20250115_201642_create_some_table.up.surql"),
    }));

    assert_that!(
        migration
            .expect("failed to create new migration file")
            .script_path
            .exists()
    )
    .is_true();
}

#[test]
fn create_migration_file_for_new_migration_with_empty_title() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let migrations_folder = temp_dir.path();

    let filename_strategy = MigrationFilenameStrategy::default();
    let migration_files = MigrationFiles::new(migrations_folder, filename_strategy);

    let new_migration = NewMigration {
        key: key("20250115_201642"),
        title: "".into(),
        kind: MigrationKind::Up,
    };

    let migration = migration_files.create_new_migration(new_migration);

    assert_that!(migration).is_equal_to(Ok(Migration {
        key: key("20250115_201642"),
        title: "".into(),
        kind: MigrationKind::Up,
        script_path: migrations_folder.join("20250115_201642.up.surql"),
    }));

    assert_that!(
        migration
            .expect("failed to create new migration file")
            .script_path
            .exists()
    )
    .is_true();
}

#[test]
fn create_migration_file_for_down_migration() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let migrations_folder = temp_dir.path();

    let filename_strategy = MigrationFilenameStrategy::default();
    let migration_files = MigrationFiles::new(migrations_folder, filename_strategy);

    let new_migration = NewMigration {
        key: key("20250115_201642"),
        title: "create some table".into(),
        kind: MigrationKind::Down,
    };

    let migration = migration_files.create_new_migration(new_migration);

    assert_that!(migration).is_equal_to(Ok(Migration {
        key: key("20250115_201642"),
        title: "create some table".into(),
        kind: MigrationKind::Down,
        script_path: migrations_folder.join("20250115_201642_create_some_table.down.surql"),
    }));

    assert_that!(
        migration
            .expect("failed to create new migration file")
            .script_path
            .exists()
    )
    .is_true();
}

#[test]
fn create_migration_file_for_down_migration_with_empty_title() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let migrations_folder = temp_dir.path();

    let filename_strategy = MigrationFilenameStrategy::default();
    let migration_files = MigrationFiles::new(migrations_folder, filename_strategy);

    let new_migration = NewMigration {
        key: key("20250115_201642"),
        title: "".into(),
        kind: MigrationKind::Down,
    };

    let migration = migration_files.create_new_migration(new_migration);

    assert_that!(migration).is_equal_to(Ok(Migration {
        key: key("20250115_201642"),
        title: "".into(),
        kind: MigrationKind::Down,
        script_path: migrations_folder.join("20250115_201642.down.surql"),
    }));

    assert_that!(
        migration
            .expect("failed to create new migration file")
            .script_path
            .exists()
    )
    .is_true();
}

#[test]
fn create_migration_file_for_new_migration_file_already_existing() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let migrations_folder = temp_dir.path();

    let filename_strategy = MigrationFilenameStrategy::default();
    let migration_files = MigrationFiles::new(migrations_folder, filename_strategy);

    let new_migration = NewMigration {
        key: key("20250115_201642"),
        title: "create some table".into(),
        kind: MigrationKind::Up,
    };

    let existing_migration = migration_files.create_new_migration(new_migration.clone());

    assert_that!(existing_migration).is_equal_to(Ok(Migration {
        key: key("20250115_201642"),
        title: "create some table".into(),
        kind: MigrationKind::Up,
        script_path: migrations_folder.join("20250115_201642_create_some_table.up.surql"),
    }));

    assert_that!(
        existing_migration
            .expect("existing file not created")
            .script_path
            .exists()
    )
    .is_true();

    let result = migration_files.create_new_migration(new_migration);

    assert_that!(matches!(result, Err(Error::CreatingScriptFile(_)))).is_true();
}

#[test]
fn create_migration_file_for_new_migration_folder_does_not_exist() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let not_existing_folder = temp_dir.path().join("not_existing");

    let filename_strategy = MigrationFilenameStrategy::default();
    let migration_files = MigrationFiles::new(&not_existing_folder, filename_strategy);

    let new_migration = NewMigration {
        key: key("20250115_201642"),
        title: "create some table".to_string(),
        kind: MigrationKind::Up,
    };

    let result = migration_files.create_new_migration(new_migration);

    assert_that!(matches!(result, Err(Error::CreatingScriptFile(_)))).is_true();
}
