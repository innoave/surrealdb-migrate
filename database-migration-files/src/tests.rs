#![allow(clippy::manual_string_new, clippy::too_many_lines)]

use super::*;
use assert_fs::TempDir;
use asserting::prelude::*;
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

    assert_that!(migrations).contains_exactly_in_any_order([
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
    assert_that!(migrations).has_error(Error::ScanningMigrationDirectory(
        r#"migrations folder "../fixtures/not_existing/migrations" does not exist"#.to_string(),
    ));
}

#[test]
fn list_all_migrations_in_migrations_dir_with_dir_tree_up_only() {
    let migrations_folder = Path::new("../fixtures/dir_tree_up_only/migrations");
    let excluded_files = ExcludedFiles::default();
    let migration_directory = MigrationDirectory::new(migrations_folder, &excluded_files);

    let migrations = migration_directory
        .list_all_migrations()
        .unwrap_or_else(|err| panic!("failed to list all migrations: {err}"))
        .collect::<Vec<_>>();

    assert_that!(migrations).contains_exactly_in_any_order([
        Ok(Migration {
            key: key("20250601_181901"),
            title: "file01".into(),
            kind: MigrationKind::Up,
            script_path: Path::new(
                "../fixtures/dir_tree_up_only/migrations/20250601_181901_file01.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250601_181902"),
            title: "file02".into(),
            kind: MigrationKind::Up,
            script_path: Path::new(
                "../fixtures/dir_tree_up_only/migrations/20250601_181902_file02.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250601_181903"),
            title: "file03".into(),
            kind: MigrationKind::Up,
            script_path: Path::new(
                "../fixtures/dir_tree_up_only/migrations/20250601_181903_file03.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250601_201240"),
            title: "file01-01".into(),
            kind: MigrationKind::Up,
            script_path: Path::new(
                "../fixtures/dir_tree_up_only/migrations/subdir01/20250601_201240_file01-01.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250601_211024"),
            title: "file01-02".into(),
            kind: MigrationKind::Up,
            script_path: Path::new(
                "../fixtures/dir_tree_up_only/migrations/subdir01/20250601_211024_file01-02.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250602_090901"),
            title: "file02-01-01".into(),
            kind: MigrationKind::Up,
            script_path: Path::new(
                "../fixtures/dir_tree_up_only/migrations/subdir02/subdir02-01/20250602_090901_file02-01-01.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250602_090902"),
            title: "file02-01-02".into(),
            kind: MigrationKind::Up,
            script_path: Path::new(
                "../fixtures/dir_tree_up_only/migrations/subdir02/subdir02-01/20250602_090902_file02-01-02.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250602_120101"),
            title: "file02-01-01-01".into(),
            kind: MigrationKind::Up,
            script_path: Path::new(
                "../fixtures/dir_tree_up_only/migrations/subdir02/subdir02-01/subdir02-01-01/20250602_120101_file02-01-01-01.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250602_142011"),
            title: "file02-01-01-02".into(),
            kind: MigrationKind::Up,
            script_path: Path::new(
                "../fixtures/dir_tree_up_only/migrations/subdir02/subdir02-01/subdir02-01-01/20250602_142011_file02-01-01-02.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250602_142550"),
            title: "file02-01-01-03".into(),
            kind: MigrationKind::Up,
            script_path: Path::new(
                "../fixtures/dir_tree_up_only/migrations/subdir02/subdir02-01/subdir02-01-01/20250602_142550_file02-01-01-03.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250602_150312"),
            title: "file02-01-02-01".into(),
            kind: MigrationKind::Up,
            script_path: Path::new(
                "../fixtures/dir_tree_up_only/migrations/subdir02/subdir02-01/subdir02-01-02/20250602_150312_file02-01-02-01.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250604_105532"),
            title: "file04-01-02-01".into(),
            kind: MigrationKind::Up,
            script_path: Path::new(
                "../fixtures/dir_tree_up_only/migrations/subdir04/subdir04-01/subdir04-01-02/20250604_105532_file04-01-02-01.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250604_105542"),
            title: "file04-01-02-02".into(),
            kind: MigrationKind::Up,
            script_path: Path::new(
                "../fixtures/dir_tree_up_only/migrations/subdir04/subdir04-01/subdir04-01-02/20250604_105542_file04-01-02-02.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250604_110101"),
            title: "file04-01-02-03".into(),
            kind: MigrationKind::Up,
            script_path: Path::new(
                "../fixtures/dir_tree_up_only/migrations/subdir04/subdir04-01/subdir04-01-02/20250604_110101_file04-01-02-03.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250604_113022"),
            title: "file04-01-03-01".into(),
            kind: MigrationKind::Up,
            script_path: Path::new(
                "../fixtures/dir_tree_up_only/migrations/subdir04/subdir04-01/subdir04-01-03/20250604_113022_file04-01-03-01.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250604_113108"),
            title: "file04-01-03-02".into(),
            kind: MigrationKind::Up,
            script_path: Path::new(
                "../fixtures/dir_tree_up_only/migrations/subdir04/subdir04-01/subdir04-01-03/20250604_113108_file04-01-03-02.surql",
            )
            .into(),
        }),
    ]);
}

#[test]
fn list_all_migrations_in_migrations_dir_with_dir_tree_up_and_down_via_extension() {
    let migrations_folder = Path::new("../fixtures/dir_tree_up_down_ext/migrations");
    let excluded_files = ExcludedFiles::default();
    let migration_directory = MigrationDirectory::new(migrations_folder, &excluded_files);

    let migrations = migration_directory
        .list_all_migrations()
        .unwrap_or_else(|err| panic!("failed to list all migrations: {err}"))
        .collect::<Vec<_>>();

    assert_that!(migrations).contains_exactly_in_any_order([
        Ok(Migration {
            key: key("20250601_181901"),
            title: "file01".into(),
            kind: MigrationKind::Up,
            script_path: Path::new(
                "../fixtures/dir_tree_up_down_ext/migrations/20250601_181901_file01.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250601_181901"),
            title: "file01".into(),
            kind: MigrationKind::Down,
            script_path: Path::new(
                "../fixtures/dir_tree_up_down_ext/migrations/20250601_181901_file01.down.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250601_181902"),
            title: "file02".into(),
            kind: MigrationKind::Up,
            script_path: Path::new(
                "../fixtures/dir_tree_up_down_ext/migrations/20250601_181902_file02.up.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250601_181902"),
            title: "file02".into(),
            kind: MigrationKind::Down,
            script_path: Path::new(
                "../fixtures/dir_tree_up_down_ext/migrations/20250601_181902_file02.down.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250601_201240"),
            title: "file01-01".into(),
            kind: MigrationKind::Up,
            script_path: Path::new(
                "../fixtures/dir_tree_up_down_ext/migrations/subdir01/20250601_201240_file01-01.up.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250601_201240"),
            title: "file01-01".into(),
            kind: MigrationKind::Down,
            script_path: Path::new(
                "../fixtures/dir_tree_up_down_ext/migrations/subdir01/20250601_201240_file01-01.down.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250601_211024"),
            title: "file01-02".into(),
            kind: MigrationKind::Up,
            script_path: Path::new(
                "../fixtures/dir_tree_up_down_ext/migrations/subdir01/20250601_211024_file01-02.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250601_211024"),
            title: "file01-02".into(),
            kind: MigrationKind::Down,
            script_path: Path::new(
                "../fixtures/dir_tree_up_down_ext/migrations/subdir01/20250601_211024_file01-02.down.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250602_090901"),
            title: "file02-01-01".into(),
            kind: MigrationKind::Up,
            script_path: Path::new(
                "../fixtures/dir_tree_up_down_ext/migrations/subdir02/subdir02-01/20250602_090901_file02-01-01.up.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250602_090901"),
            title: "file02-01-01".into(),
            kind: MigrationKind::Down,
            script_path: Path::new(
                "../fixtures/dir_tree_up_down_ext/migrations/subdir02/subdir02-01/20250602_090901_file02-01-01.down.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250602_120101"),
            title: "file02-01-01-01".into(),
            kind: MigrationKind::Up,
            script_path: Path::new(
                "../fixtures/dir_tree_up_down_ext/migrations/subdir02/subdir02-01/subdir02-01-01/20250602_120101_file02-01-01-01.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250602_120101"),
            title: "file02-01-01-01".into(),
            kind: MigrationKind::Down,
            script_path: Path::new(
                "../fixtures/dir_tree_up_down_ext/migrations/subdir02/subdir02-01/subdir02-01-01/20250602_120101_file02-01-01-01.down.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250602_150312"),
            title: "file02-01-02-01".into(),
            kind: MigrationKind::Up,
            script_path: Path::new(
                "../fixtures/dir_tree_up_down_ext/migrations/subdir02/subdir02-01/subdir02-01-02/20250602_150312_file02-01-02-01.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250604_105532"),
            title: "file04-01-02-01".into(),
            kind: MigrationKind::Up,
            script_path: Path::new(
                "../fixtures/dir_tree_up_down_ext/migrations/subdir04/subdir04-01/subdir04-01-02/20250604_105532_file04-01-02-01.up.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250604_105532"),
            title: "file04-01-02-01".into(),
            kind: MigrationKind::Down,
            script_path: Path::new(
                "../fixtures/dir_tree_up_down_ext/migrations/subdir04/subdir04-01/subdir04-01-02/20250604_105532_file04-01-02-01.down.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250604_105542"),
            title: "file04-01-02-02".into(),
            kind: MigrationKind::Up,
            script_path: Path::new(
                "../fixtures/dir_tree_up_down_ext/migrations/subdir04/subdir04-01/subdir04-01-02/20250604_105542_file04-01-02-02.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250604_105542"),
            title: "file04-01-02-02".into(),
            kind: MigrationKind::Down,
            script_path: Path::new(
                "../fixtures/dir_tree_up_down_ext/migrations/subdir04/subdir04-01/subdir04-01-02/20250604_105542_file04-01-02-02.down.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250604_110101"),
            title: "file04-01-02-03".into(),
            kind: MigrationKind::Up,
            script_path: Path::new(
                "../fixtures/dir_tree_up_down_ext/migrations/subdir04/subdir04-01/subdir04-01-02/20250604_110101_file04-01-02-03.up.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250604_110101"),
            title: "file04-01-02-03".into(),
            kind: MigrationKind::Down,
            script_path: Path::new(
                "../fixtures/dir_tree_up_down_ext/migrations/subdir04/subdir04-01/subdir04-01-02/20250604_110101_file04-01-02-03.down.surql",
            )
            .into(),
        }),
        Ok(Migration {
            key: key("20250604_113022"),
            title: "file04-01-03-01".into(),
            kind: MigrationKind::Up,
            script_path: Path::new(
                "../fixtures/dir_tree_up_down_ext/migrations/subdir04/subdir04-01/subdir04-01-03/20250604_113022_file04-01-03-01.surql",
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

    assert_that!(migrations)
        .contains_exactly_in_any_order([Err(Error::Definition(DefinitionError::InvalidFilename))]);
}

#[test]
fn list_migrations_ignores_configured_filenames_default_pattern_dot_keep_file() {
    let migrations_folder = Path::new("../fixtures/empty/migrations");

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
        .unwrap_or_else(|err| panic!("could not write README.md file: {err}"));

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
        .unwrap_or_else(|err| panic!("could not write TODO.txt file: {err}"));

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

    assert_that!(script_contents).contains_exactly_in_any_order([
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

    assert_that!(migration)
        .ok()
        .is_equal_to(Migration {
            key: key("20250115_201642"),
            title: "create some table".into(),
            kind: MigrationKind::Up,
            script_path: migrations_folder.join("20250115_201642_create_some_table.up.surql"),
        })
        .extracting(|mig| mig.script_path.exists())
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

    assert_that!(migration)
        .ok()
        .is_equal_to(Migration {
            key: key("20250115_201642"),
            title: "".into(),
            kind: MigrationKind::Up,
            script_path: migrations_folder.join("20250115_201642.up.surql"),
        })
        .extracting(|mig| mig.script_path.exists())
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

    assert_that!(migration)
        .ok()
        .is_equal_to(Migration {
            key: key("20250115_201642"),
            title: "create some table".into(),
            kind: MigrationKind::Down,
            script_path: migrations_folder.join("20250115_201642_create_some_table.down.surql"),
        })
        .extracting(|mig| mig.script_path.exists())
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

    assert_that!(migration)
        .ok()
        .is_equal_to(Migration {
            key: key("20250115_201642"),
            title: "".into(),
            kind: MigrationKind::Down,
            script_path: migrations_folder.join("20250115_201642.down.surql"),
        })
        .extracting(|mig| mig.script_path.exists())
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

    assert_that!(existing_migration)
        .ok()
        .is_equal_to(Migration {
            key: key("20250115_201642"),
            title: "create some table".into(),
            kind: MigrationKind::Up,
            script_path: migrations_folder.join("20250115_201642_create_some_table.up.surql"),
        })
        .extracting(|mig| mig.script_path.exists())
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
        title: "create some table".into(),
        kind: MigrationKind::Up,
    };

    let result = migration_files.create_new_migration(new_migration);

    assert_that!(matches!(result, Err(Error::CreatingScriptFile(_)))).is_true();
}
