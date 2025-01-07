use super::*;
use crate::migration::MigrationKind;
use crate::test_dsl::key;
use speculoos::prelude::*;

mod str {
    use super::*;

    #[test]
    fn parse_migration_from_valid_file_path() {
        let path = "migrations/20250103_140830_define_some_table.surql";

        let migration = path.parse_migration();

        assert_that!(migration).is_ok_containing(Migration {
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

        assert_that!(migration).is_ok_containing(Migration {
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

        assert_that!(migration).is_ok_containing(Migration {
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

        assert_that!(migration).is_ok_containing(Migration {
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

        assert_that!(migration).is_ok_containing(Migration {
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

        assert_that!(migration).is_ok_containing(Migration {
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

        assert_that!(migration).is_ok_containing(Migration {
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

        assert_that!(migration).is_ok_containing(Migration {
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

        assert_that!(migration).is_ok_containing(Migration {
            key: key("20250103_140830"),
            title: "define some table".into(),
            kind: MigrationKind::Up,
            script_path: path.into(),
        });
    }

    #[test]
    fn parse_migration_from_file_path_without_date() {
        let path = Path::new("migrations/140830_define_some_table.surql");

        let migration = path.parse_migration();

        assert_that!(migration)
            .is_err_containing(DefinitionError::InvalidDate("input is out of range".into()));
    }

    #[test]
    fn parse_migration_from_file_path_without_time() {
        let path = Path::new("migrations/20250103_define_some_table.surql");

        let migration = path.parse_migration();

        assert_that!(migration).is_err_containing(DefinitionError::InvalidTime(
            "input contains invalid characters".into(),
        ));
    }

    #[test]
    fn parse_migration_from_file_path_without_title() {
        let path = Path::new("migrations/20250103_140830.surql");

        let migration = path.parse_migration();

        assert_that!(migration).is_err_containing(DefinitionError::MissingTitle);
    }

    #[test]
    fn parse_migration_from_file_path_with_up_and_down() {
        let path = Path::new("migrations/20250103_140830_define_some_table.up.down.surql");

        let migration = path.parse_migration();

        assert_that!(migration).is_err_containing(DefinitionError::AmbiguousDirection);
    }

    #[test]
    fn parse_migration_from_file_path_without_a_filename() {
        let path = Path::new("migrations");

        let migration = path.parse_migration();

        assert_that!(migration).is_err_containing(DefinitionError::NoFilename);
    }

    #[test]
    fn parse_migration_from_empty_path() {
        let path = Path::new("");

        let migration = path.parse_migration();

        assert_that!(migration).is_err_containing(DefinitionError::NoFilename);
    }
}
