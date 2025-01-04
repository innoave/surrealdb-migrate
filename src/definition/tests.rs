use super::*;
use crate::migration::Direction;
use speculoos::prelude::*;
use time::macros::datetime;

mod str {
    use super::*;

    #[test]
    fn parse_migration_from_valid_file_path() {
        let path = "migrations/20250103_140830_define_some_table.surql";

        let migration = path.parse_migration();

        assert_that!(migration).is_ok_containing(Migration {
            id: datetime!(2025-01-03 14:08:30),
            title: "define_some_table".to_string(),
            direction: Direction::Up,
            script_path: path.into(),
        });
    }

    #[test]
    fn parse_migration_from_filename_only() {
        let path = "20250103_140830_define_some_table.surql";

        let migration = path.parse_migration();

        assert_that!(migration).is_ok_containing(Migration {
            id: datetime!(2025-01-03 14:08:30),
            title: "define_some_table".to_string(),
            direction: Direction::Up,
            script_path: path.into(),
        });
    }

    #[test]
    fn parse_migration_from_file_path_with_up_extension() {
        let path = "migrations/20250103_140830_define_some_table.up.surql";

        let migration = path.parse_migration();

        assert_that!(migration).is_ok_containing(Migration {
            id: datetime!(2025-01-03 14:08:30),
            title: "define_some_table".to_string(),
            direction: Direction::Up,
            script_path: path.into(),
        });
    }

    #[test]
    fn parse_migration_from_file_path_with_down_extension() {
        let path = "migrations/20250103_140830_define_some_table.down.surql";

        let migration = path.parse_migration();

        assert_that!(migration).is_ok_containing(Migration {
            id: datetime!(2025-01-03 14:08:30),
            title: "define_some_table".to_string(),
            direction: Direction::Down,
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
            id: datetime!(2025-01-03 14:08:30),
            title: "define_some_table".to_string(),
            direction: Direction::Up,
            script_path: path.into(),
        });
    }
}

mod path {
    use super::*;
    use std::path::Path;
    use time::error::{Parse, ParseFromDescription};

    #[test]
    fn parse_migration_from_valid_file_path() {
        let path = Path::new("migrations/20250103_140830_define_some_table.surql");

        let migration = path.parse_migration();

        assert_that!(migration).is_ok_containing(Migration {
            id: datetime!(2025-01-03 14:08:30),
            title: "define_some_table".to_string(),
            direction: Direction::Up,
            script_path: path.into(),
        });
    }

    #[test]
    fn parse_migration_from_file_path_with_up_extension() {
        let path = Path::new("migrations/20250103_140830_define_some_table.up.surql");

        let migration = path.parse_migration();

        assert_that!(migration).is_ok_containing(Migration {
            id: datetime!(2025-01-03 14:08:30),
            title: "define_some_table".to_string(),
            direction: Direction::Up,
            script_path: path.into(),
        });
    }

    #[test]
    fn parse_migration_from_file_path_with_down_extension() {
        let path = Path::new("migrations/20250103_140830_define_some_table.down.surql");

        let migration = path.parse_migration();

        assert_that!(migration).is_ok_containing(Migration {
            id: datetime!(2025-01-03 14:08:30),
            title: "define_some_table".to_string(),
            direction: Direction::Down,
            script_path: path.into(),
        });
    }

    #[test]
    fn parse_migration_from_filename_only() {
        let path = Path::new("20250103_140830_define_some_table.surql");

        let migration = path.parse_migration();

        assert_that!(migration).is_ok_containing(Migration {
            id: datetime!(2025-01-03 14:08:30),
            title: "define_some_table".to_string(),
            direction: Direction::Up,
            script_path: path.into(),
        });
    }

    #[test]
    fn parse_migration_from_file_path_without_date() {
        let path = Path::new("migrations/140830_define_some_table.surql");

        let migration = path.parse_migration();

        assert_that!(migration).is_err_containing(Error::InvalidDate(Parse::ParseFromDescription(
            ParseFromDescription::InvalidComponent("month"),
        )));
    }

    #[test]
    fn parse_migration_from_file_path_without_time() {
        let path = Path::new("migrations/20250103_define_some_table.surql");

        let migration = path.parse_migration();

        assert_that!(migration).is_err_containing(Error::InvalidTime(Parse::ParseFromDescription(
            ParseFromDescription::InvalidComponent("hour"),
        )));
    }

    #[test]
    fn parse_migration_from_file_path_without_title() {
        let path = Path::new("migrations/20250103_140830.surql");

        let migration = path.parse_migration();

        assert_that!(migration).is_err_containing(Error::MissingTitle);
    }

    #[test]
    fn parse_migration_from_file_path_with_up_and_down() {
        let path = Path::new("migrations/20250103_140830_define_some_table.up.down.surql");

        let migration = path.parse_migration();

        assert_that!(migration).is_err_containing(Error::AmbiguousDirection);
    }

    #[test]
    fn parse_migration_from_file_path_without_a_filename() {
        let path = Path::new("migrations");

        let migration = path.parse_migration();

        assert_that!(migration).is_err_containing(Error::NoFilename);
    }

    #[test]
    fn parse_migration_from_empty_path() {
        let path = Path::new("");

        let migration = path.parse_migration();

        assert_that!(migration).is_err_containing(Error::NoFilename);
    }
}
