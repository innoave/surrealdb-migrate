#![allow(clippy::manual_string_new)]

use super::*;
use crate::migration::MigrationKind;
use crate::proptest_support::{any_direction, any_key, any_title};
use crate::test_dsl::key;
use assertor::*;
use proptest::prelude::*;

mod str {
    use super::*;

    #[test]
    fn parse_migration_from_valid_file_path() {
        let path = "migrations/20250103_140830_define_some_table.surql";

        let migration = path.parse_migration();

        assert_that!(migration).has_ok(Migration {
            key: key("20250103_140830"),
            title: "define some table".into(),
            kind: MigrationKind::Up,
            script_path: path.into(),
        });
    }

    #[test]
    fn parse_migration_from_filename_only() {
        let path = "20250103_140830_define_some_table.surql";

        let migration = path.parse_migration();

        assert_that!(migration).has_ok(Migration {
            key: key("20250103_140830"),
            title: "define some table".into(),
            kind: MigrationKind::Up,
            script_path: path.into(),
        });
    }

    #[test]
    fn parse_migration_from_file_path_with_up_extension() {
        let path = "migrations/20250103_140830_define_some_table.up.surql";

        let migration = path.parse_migration();

        assert_that!(migration).has_ok(Migration {
            key: key("20250103_140830"),
            title: "define some table".into(),
            kind: MigrationKind::Up,
            script_path: path.into(),
        });
    }

    #[test]
    fn parse_migration_from_file_path_with_down_extension() {
        let path = "migrations/20250103_140830_define_some_table.down.surql";

        let migration = path.parse_migration();

        assert_that!(migration).has_ok(Migration {
            key: key("20250103_140830"),
            title: "define some table".into(),
            kind: MigrationKind::Down,
            script_path: path.into(),
        });
    }
}

mod string {
    use super::*;

    #[test]
    fn parse_migration_from_valid_file_path() {
        let path = String::from("migrations/20250103_140830_define_some_table.surql");

        let migration = path.parse_migration();

        assert_that!(migration).has_ok(Migration {
            key: key("20250103_140830"),
            title: "define some table".into(),
            kind: MigrationKind::Up,
            script_path: path.into(),
        });
    }
}

mod os_str {
    use super::*;
    use std::ffi::OsString;

    #[test]
    fn parse_migration_from_valid_file_path() {
        let path = OsString::from("migrations/20250103_140830_define_some_table.surql");

        let migration = path.parse_migration();

        assert_that!(migration).has_ok(Migration {
            key: key("20250103_140830"),
            title: "define some table".into(),
            kind: MigrationKind::Up,
            script_path: path.into(),
        });
    }
}

mod path {
    use super::*;
    use std::path::Path;

    #[test]
    fn parse_migration_from_valid_file_path() {
        let path = Path::new("migrations/20250103_140830_define_some_table.surql");

        let migration = path.parse_migration();

        assert_that!(migration).has_ok(Migration {
            key: key("20250103_140830"),
            title: "define some table".into(),
            kind: MigrationKind::Up,
            script_path: path.into(),
        });
    }

    #[test]
    fn parse_migration_from_file_path_with_up_extension() {
        let path = Path::new("migrations/20250103_140830_define_some_table.up.surql");

        let migration = path.parse_migration();

        assert_that!(migration).has_ok(Migration {
            key: key("20250103_140830"),
            title: "define some table".into(),
            kind: MigrationKind::Up,
            script_path: path.into(),
        });
    }

    #[test]
    fn parse_migration_from_file_path_with_down_extension() {
        let path = Path::new("migrations/20250103_140830_define_some_table.down.surql");

        let migration = path.parse_migration();

        assert_that!(migration).has_ok(Migration {
            key: key("20250103_140830"),
            title: "define some table".into(),
            kind: MigrationKind::Down,
            script_path: path.into(),
        });
    }

    #[test]
    fn parse_migration_from_filename_only() {
        let path = Path::new("20250103_140830_define_some_table.surql");

        let migration = path.parse_migration();

        assert_that!(migration).has_ok(Migration {
            key: key("20250103_140830"),
            title: "define some table".into(),
            kind: MigrationKind::Up,
            script_path: path.into(),
        });
    }

    #[test]
    fn parse_migration_from_file_path_without_title() {
        let path = Path::new("migrations/20250103_140830.surql");

        let migration = path.parse_migration();

        assert_that!(migration).has_ok(Migration {
            key: key("20250103_140830"),
            title: "".into(),
            kind: MigrationKind::Up,
            script_path: path.into(),
        });
    }

    #[test]
    fn parse_migration_from_file_path_without_date() {
        let path = Path::new("migrations/140830_define_some_table.surql");

        let migration = path.parse_migration();

        assert_that!(migration)
            .has_err(DefinitionError::InvalidDate("input is out of range".into()));
    }

    #[test]
    fn parse_migration_from_file_path_without_time() {
        let path = Path::new("migrations/20250103_define_some_table.surql");

        let migration = path.parse_migration();

        assert_that!(migration).has_err(DefinitionError::InvalidTime(
            "input contains invalid characters".into(),
        ));
    }

    #[test]
    fn parse_migration_from_file_path_with_up_and_down() {
        let path = Path::new("migrations/20250103_140830_define_some_table.up.down.surql");

        let migration = path.parse_migration();

        assert_that!(migration).has_err(DefinitionError::AmbiguousDirection);
    }

    #[test]
    fn parse_migration_from_file_path_without_a_filename() {
        let path = Path::new("migrations");

        let migration = path.parse_migration();

        assert_that!(migration).has_err(DefinitionError::NoFilename);
    }

    #[test]
    fn parse_migration_from_empty_path() {
        let path = Path::new("");

        let migration = path.parse_migration();

        assert_that!(migration).has_err(DefinitionError::NoFilename);
    }
}

mod migration_filename_strategy {
    use super::*;

    #[test]
    fn get_filename_with_default_strategy_for_up_migration() {
        let filename_strategy = MigrationFilenameStrategy::default();

        let migration = NewMigration {
            key: key("20250114_092042"),
            title: "create some table".to_string(),
            kind: MigrationKind::Up,
        };

        let filename = filename_strategy.get_filename(&migration);

        assert_that!(filename)
            .is_equal_to("20250114_092042_create_some_table.up.surql".to_string());
    }

    #[test]
    fn get_filename_with_default_strategy_for_down_migration() {
        let filename_strategy = MigrationFilenameStrategy::default();

        let migration = NewMigration {
            key: key("20250101_235959"),
            title: "create some table".to_string(),
            kind: MigrationKind::Down,
        };

        let filename = filename_strategy.get_filename(&migration);

        assert_that!(filename)
            .is_equal_to("20250101_235959_create_some_table.down.surql".to_string());
    }

    #[test]
    #[should_panic(expected = "baselines do not have migration scripts")]
    fn get_filename_with_default_strategy_for_baseline_migration() {
        let filename_strategy = MigrationFilenameStrategy::default().with_up_postfix(false);

        let migration = NewMigration {
            key: key("20250114_092042"),
            title: "create some table".to_string(),
            kind: MigrationKind::Baseline,
        };

        _ = filename_strategy.get_filename(&migration);
    }

    #[test]
    fn get_filename_with_no_up_postfix_strategy_for_up_migration() {
        let filename_strategy = MigrationFilenameStrategy::default().with_up_postfix(false);

        let migration = NewMigration {
            key: key("20250101_235959"),
            title: "create some table".to_string(),
            kind: MigrationKind::Up,
        };

        let filename = filename_strategy.get_filename(&migration);

        assert_that!(filename).is_equal_to("20250101_235959_create_some_table.surql".to_string());
    }

    #[test]
    fn get_filename_with_no_up_postfix_strategy_for_down_migration() {
        let filename_strategy = MigrationFilenameStrategy::default().with_up_postfix(false);

        let migration = NewMigration {
            key: key("20250114_092042"),
            title: "create some table".to_string(),
            kind: MigrationKind::Down,
        };

        let filename = filename_strategy.get_filename(&migration);

        assert_that!(filename)
            .is_equal_to("20250114_092042_create_some_table.down.surql".to_string());
    }
}

proptest! {
    #[test]
    fn any_filename_created_by_the_strategy_can_be_parsed_as_migration(
        key in any_key(),
        title in any_title(),
        direction in any_direction(),
        up_postfix in any::<bool>(),
    ) {
        let filename_strategy = MigrationFilenameStrategy {
            up_postfix,
        };

        let new_migration = NewMigration {
            key,
            title: title.clone(),
            kind: direction,
        };
        let filename = filename_strategy.get_filename(&new_migration);

        let parsed_migration = filename.parse_migration();

        prop_assert_eq!(parsed_migration, Ok(Migration {
            key,
            title: title.replace('_', " "),
            kind: direction,
            script_path: PathBuf::from(&filename),
        }));
    }
}
