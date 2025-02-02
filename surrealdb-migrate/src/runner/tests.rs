use super::*;
use assertor::*;

mod migration_runner {
    use super::*;
    use assert_fs::TempDir;
    use database_migration::test_dsl::key;
    use std::path::Path;

    #[test]
    fn can_be_instantiated_with_default_runner_config() {
        let runner_config = RunnerConfig::default();

        let migration_runner = MigrationRunner::new(runner_config);

        assert_that!(migration_runner.migrations_folder).is_equal_to(PathBuf::from("migrations"));
        assert_that!(migration_runner.migrations_table).is_equal_to("migrations".to_string());
        assert_that!(migration_runner.ignore_checksum).is_equal_to(false);
        assert_that!(migration_runner.ignore_order).is_equal_to(false);
    }

    #[cfg(feature = "config")]
    #[test]
    fn can_be_instantiated_with_default_settings() {
        use assert_fs::TempDir;
        let temp_dir = TempDir::new().expect("failed to create temp dir");
        let settings =
            Settings::load_from_dir(temp_dir.path()).expect("failed to load default settings");

        let migration_runner = MigrationRunner::with_settings(&settings);

        assert_that!(migration_runner.migrations_folder).is_equal_to(PathBuf::from("migrations"));
        assert_that!(migration_runner.migrations_table).is_equal_to("migrations".to_string());
        assert_that!(migration_runner.ignore_checksum).is_equal_to(false);
        assert_that!(migration_runner.ignore_order).is_equal_to(false);
    }

    #[test]
    fn list_defined_forward_migrations_in_basic_fixture() {
        let config = RunnerConfig::default()
            .with_migrations_folder(Path::new("../fixtures/basic/migrations"));
        let runner = MigrationRunner::new(config);

        let defined_migrations = runner
            .list_defined_migrations(MigrationKind::is_forward)
            .expect("failed to list defined migrations");

        assert_that!(defined_migrations).contains_exactly_in_order(vec![
            Migration {
                key: key("20250103_140520"),
                title: "define quote table".into(),
                kind: MigrationKind::Up,
                script_path:
                    "../fixtures/basic/migrations/20250103_140520_define_quote_table.surql".into(),
            },
            Migration {
                key: key("20250103_140521"),
                title: "create some quotes".into(),
                kind: MigrationKind::Up,
                script_path:
                    "../fixtures/basic/migrations/20250103_140521_create_some_quotes.surql".into(),
            },
        ]);
    }

    #[test]
    fn list_defined_backward_migrations_in_basic_fixture() {
        let config = RunnerConfig::default()
            .with_migrations_folder(Path::new("../fixtures/basic/migrations"));
        let runner = MigrationRunner::new(config);

        let defined_migrations = runner
            .list_defined_migrations(MigrationKind::is_backward)
            .expect("failed to list defined migrations");

        assert_that!(defined_migrations).is_empty();
    }

    #[test]
    fn list_defined_migrations_of_any_kind_in_empty_folder() {
        let temp_dir = TempDir::new().expect("failed to create temp dir");
        let config = RunnerConfig::default().with_migrations_folder(temp_dir.path());
        let runner = MigrationRunner::new(config);

        let defined_migrations = runner
            .list_defined_migrations(MigrationKind::is_any)
            .expect("failed to list defined migrations");

        assert_that!(defined_migrations).is_empty();
    }

    #[test]
    fn list_defined_migrations_of_any_kind_in_non_existing_folder() {
        let temp_dir = TempDir::new().expect("failed to create temp dir");
        let non_existing_folder = temp_dir.join("non_existing/migrations");
        let config = RunnerConfig::default().with_migrations_folder(non_existing_folder);
        let runner = MigrationRunner::new(config);

        let result = runner.list_defined_migrations(MigrationKind::is_any);

        assert_that!(matches!(result, Err(Error::ScanningMigrationDirectory(_)))).is_true();
    }
}
