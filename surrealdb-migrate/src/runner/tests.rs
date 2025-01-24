use super::*;
use assertor::*;

mod migration_runner {
    use super::*;

    #[test]
    fn can_be_instantiated_with_default_runner_config() {
        let runner_config = RunnerConfig::default();

        let migration_runner = MigrationRunner::new(runner_config);

        assert_that!(migration_runner.migrations_folder).is_equal_to(PathBuf::from("migrations"));
        assert_that!(migration_runner.migrations_table).is_equal_to("migrations".to_string());
        assert_that!(migration_runner.ignore_checksums).is_equal_to(false);
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
        assert_that!(migration_runner.ignore_checksums).is_equal_to(false);
        assert_that!(migration_runner.ignore_order).is_equal_to(false);
    }
}
