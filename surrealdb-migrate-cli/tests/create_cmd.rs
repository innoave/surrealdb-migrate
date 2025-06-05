mod fixtures;

use crate::fixtures::files::list_filenames_in_dir;
use crate::fixtures::surmig;
use assert_fs::TempDir;
use asserting::prelude::*;

#[test]
fn create_migration_with_current_date_and_time_and_no_title() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let migrations_folder = temp_dir.path().join("migrations");

    let cmd = surmig().args([
        "--migrations-folder",
        migrations_folder
            .to_str()
            .expect("failed to convert migrations folder path to str"),
        "create",
    ]);

    cmd.assert()
        .code(0)
        .stdout_eq(snapbox::file![
            "create_cmd/create_migration_with_current_date_and_time_and_no_title.stdout"
        ])
        .stderr_eq("");

    assert_that!(migrations_folder.exists()).is_true();
    assert_that!(migrations_folder.is_dir()).is_true();
    let mig_files = list_filenames_in_dir(&migrations_folder)
        .filter(|filename| filename.ends_with(".up.surql"));
    assert_that!(mig_files.count()).is_equal_to(1);
}

#[test]
fn create_migration_with_current_date_and_time_and_a_given_title() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let migrations_folder = temp_dir.path().join("migrations");

    let cmd = surmig().args([
        "--migrations-folder",
        migrations_folder
            .to_str()
            .expect("failed to convert migrations folder path to str"),
        "create",
        "add some more quotes",
    ]);

    cmd.assert()
        .code(0)
        .stdout_eq(snapbox::file![
            "create_cmd/create_migration_with_current_date_and_time_and_a_given_title.stdout"
        ])
        .stderr_eq("");

    assert_that!(migrations_folder.exists()).is_true();
    assert_that!(migrations_folder.is_dir()).is_true();
    let mig_files = list_filenames_in_dir(&migrations_folder)
        .filter(|filename| filename.ends_with("_add_some_more_quotes.up.surql"));
    assert_that!(mig_files.count()).is_equal_to(1);
}

#[test]
fn create_migration_with_given_key_and_a_given_title() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let migrations_folder = temp_dir.path().join("migrations");

    let cmd = surmig().args([
        "--migrations-folder",
        migrations_folder
            .to_str()
            .expect("failed to convert migrations folder path to str"),
        "create",
        "--key",
        "20250126_120033",
        "add some more quotes",
    ]);

    cmd.assert()
        .code(0)
        .stdout_eq(snapbox::file![
            "create_cmd/create_migration_with_given_key_and_a_given_title.stdout"
        ])
        .stderr_eq("");

    assert_that!(migrations_folder.exists()).is_true();
    assert_that!(migrations_folder.is_dir()).is_true();
    let mig_files = list_filenames_in_dir(&migrations_folder).collect::<Vec<_>>();
    assert_that!(mig_files).contains_exactly(["20250126_120033_add_some_more_quotes.up.surql"]);
}

#[test]
fn create_migration_with_an_invalid_key_as_argument() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let migrations_folder = temp_dir.path().join("migrations");

    let cmd = surmig().args([
        "--migrations-folder",
        migrations_folder
            .to_str()
            .expect("failed to convert migrations folder path to str"),
        "create",
        "--key",
        "V0101",
        "add some more quotes",
    ]);

    cmd.assert().code(1).stdout_eq("").stderr_eq(snapbox::file![
        "create_cmd/create_migration_with_an_invalid_key_as_argument.stderr"
    ]);

    assert_that!(migrations_folder.exists()).is_true();
    assert_that!(migrations_folder.is_dir()).is_true();
    let mig_files = list_filenames_in_dir(&migrations_folder);
    assert_that!(mig_files.count()).is_equal_to(0);
}

#[test]
fn create_migration_including_down_migration() {
    let temp_dir = TempDir::new().expect("failed to create temp dir");
    let migrations_folder = temp_dir.path().join("migrations");

    let cmd = surmig().args([
        "--migrations-folder",
        migrations_folder
            .to_str()
            .expect("failed to convert migrations folder path to str"),
        "create",
        "--key",
        "20250126_120033",
        "add some more quotes",
        "--down",
    ]);

    cmd.assert()
        .code(0)
        .stdout_eq(snapbox::file![
            "create_cmd/create_migration_including_down_migration.stdout"
        ])
        .stderr_eq("");

    assert_that!(migrations_folder.exists()).is_true();
    assert_that!(migrations_folder.is_dir()).is_true();
    let mig_files = list_filenames_in_dir(&migrations_folder).collect::<Vec<_>>();
    assert_that!(mig_files).contains_exactly_in_any_order([
        "20250126_120033_add_some_more_quotes.up.surql",
        "20250126_120033_add_some_more_quotes.down.surql",
    ]);
}
