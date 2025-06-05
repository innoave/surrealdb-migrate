#![allow(clippy::manual_string_new)]

use super::*;
use crate::migration::MigrationKind;
use crate::proptest_support::{any_direction, any_key, any_title};
use crate::test_dsl::key;
use asserting::prelude::*;
use proptest::prelude::*;
use std::sync::LazyLock;

mod str {
    use super::*;

    #[test]
    fn parse_migration_from_valid_file_path() {
        let path = "migrations/20250103_140830_define_some_table.surql";

        let migration = path.parse_migration();

        assert_that!(migration).ok().is_equal_to(Migration {
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

        assert_that!(migration).ok().is_equal_to(Migration {
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

        assert_that!(migration).ok().is_equal_to(Migration {
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

        assert_that!(migration).ok().is_equal_to(Migration {
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

        assert_that!(migration).ok().is_equal_to(Migration {
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

        assert_that!(migration).ok().is_equal_to(Migration {
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

        assert_that!(migration).ok().is_equal_to(Migration {
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

        assert_that!(migration).ok().is_equal_to(Migration {
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

        assert_that!(migration).ok().is_equal_to(Migration {
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

        assert_that!(migration).ok().is_equal_to(Migration {
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

        assert_that!(migration).ok().is_equal_to(Migration {
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
            .err()
            .is_equal_to(DefinitionError::InvalidDate("input is out of range".into()));
    }

    #[test]
    fn parse_migration_from_file_path_without_time() {
        let path = Path::new("migrations/20250103_define_some_table.surql");

        let migration = path.parse_migration();

        assert_that!(migration)
            .err()
            .is_equal_to(DefinitionError::InvalidTime(
                "input contains invalid characters".into(),
            ));
    }

    #[test]
    fn parse_migration_from_file_path_with_up_and_down() {
        let path = Path::new("migrations/20250103_140830_define_some_table.up.down.surql");

        let migration = path.parse_migration();

        assert_that!(migration)
            .err()
            .is_equal_to(DefinitionError::AmbiguousDirection);
    }

    #[test]
    fn parse_migration_from_file_path_without_a_filename() {
        let path = Path::new("migrations");

        let migration = path.parse_migration();

        assert_that!(migration)
            .err()
            .is_equal_to(DefinitionError::InvalidFilename);
    }

    #[test]
    fn parse_migration_from_empty_path() {
        let path = Path::new("");

        let migration = path.parse_migration();

        assert_that!(migration)
            .err()
            .is_equal_to(DefinitionError::InvalidFilename);
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

mod file_pattern {
    use super::*;

    #[test]
    fn parse_from_empty_string() {
        let result = FilePattern::from_str("");

        assert_that!(result)
            .err()
            .is_equal_to(FilePatternError::EmptySubPatternNotAllowed);
    }

    #[test]
    fn parse_from_pattern_without_wildcards_and_without_pipechar() {
        let file_pattern = FilePattern::from_str("README.md")
            .unwrap_or_else(|err| panic!("failed to parse FilePattern: {err}"));

        assert_that!(&file_pattern.pattern).is_equal_to(&"README.md".to_string());
        assert_that!(file_pattern.filename_pattern).is_true();
        assert_that!(file_pattern.regex.as_str()).is_equal_to("^README\\.md$");
    }

    #[test]
    fn parse_from_pattern_with_dot_env() {
        let file_pattern = FilePattern::from_str(".env")
            .unwrap_or_else(|err| panic!("failed to parse FilePattern: {err}"));

        assert_that!(&file_pattern.pattern).is_equal_to(&".env".to_string());
        assert_that!(file_pattern.filename_pattern).is_true();
        assert_that!(file_pattern.regex.as_str()).is_equal_to("^\\.env$");
    }

    #[test]
    fn parse_from_pattern_for_all_dot_files() {
        let file_pattern = FilePattern::from_str(".*")
            .unwrap_or_else(|err| panic!("failed to parse FilePattern: {err}"));

        assert_that!(&file_pattern.pattern).is_equal_to(&".*".to_string());
        assert_that!(file_pattern.filename_pattern).is_true();
        assert_that!(file_pattern.regex.as_str()).is_equal_to("^\\.[^/]*$");
    }

    #[test]
    fn parse_from_pattern_for_files_in_all_subdirs() {
        let file_pattern = FilePattern::from_str("**/README*")
            .unwrap_or_else(|err| panic!("failed to parse FilePattern: {err}"));

        assert_that!(&file_pattern.pattern).is_equal_to(&"**/README*".to_string());
        assert_that!(file_pattern.filename_pattern).is_false();
        assert_that!(file_pattern.regex.as_str()).is_equal_to("^.*/README[^/]*$");
    }

    #[test]
    fn parse_from_pattern_for_all_dot_files_in_all_subdirs() {
        let file_pattern = FilePattern::from_str("**/.*")
            .unwrap_or_else(|err| panic!("failed to parse FilePattern: {err}"));

        assert_that!(&file_pattern.pattern).is_equal_to(&"**/.*".to_string());
        assert_that!(file_pattern.filename_pattern).is_false();
        assert_that!(file_pattern.regex.as_str()).is_equal_to("^.*/\\.[^/]*$");
    }

    #[test]
    fn parse_from_pattern_with_invalid_char() {
        let result = ExcludedFiles::from_str("README:");

        match result {
            Ok(val) => {
                panic!("expected error, but got {:?}", val.pattern);
            },
            Err(error) => {
                assert_that!(error).is_equal_to(FilePatternError::InvalidCharacter(vec![':']));
            },
        }
    }

    proptest! {
        #[test]
        fn parse_pattern_for_filename(
            pattern_string in r"[\p{Alphabetic}\p{gc=Number} _\-.*]+\.[\p{Alphabetic}\p{gc=Number} _\-*]*",
        ) {
            let file_pattern = FilePattern::from_str(&pattern_string)
                .unwrap_or_else(|err| panic!("failed to parse FilePattern: {err}"));

            prop_assert!(file_pattern.is_filename_pattern());
        }

        #[test]
        fn parse_pattern_for_filepath(
            pattern_string in r"((\.\./)|(/))?([\p{Alphabetic}\p{gc=Number} _\-.*]+/)+[\p{Alphabetic}\p{gc=Number} _\-.*]*\.[\p{Alphabetic}\p{gc=Number} _\-*]*",
        ) {
            let file_pattern = FilePattern::from_str(&pattern_string)
                .unwrap_or_else(|err| panic!("failed to parse FilePattern: {err}"));

            prop_assert!(!file_pattern.is_filename_pattern());
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(50))]

        #[test]
        fn parse_from_pattern_string_and_convert_back_to_string(
            pattern_string in r"[\p{Alphabetic}\p{gc=Number} _\-./*]+",
        ) {
            let file_pattern = FilePattern::from_str(&pattern_string)
                .unwrap_or_else(|err| panic!("failed to parse FilePattern: {err}"));
            let display_string = file_pattern.to_string();

            prop_assert_eq!(display_string, pattern_string);
        }

        #[test]
        fn two_file_pattern_parsed_from_the_same_pattern_string_are_equal(
            pattern_string in r"[\p{Alphabetic}\p{gc=Number} _\-./*]+",
        ) {
            let file_pattern1 = FilePattern::from_str(&pattern_string)
                .unwrap_or_else(|err| panic!("failed to parse FilePattern: {err}"));
            let file_pattern2 = FilePattern::from_str(&pattern_string)
                .unwrap_or_else(|err| panic!("failed to parse FilePattern: {err}"));

            prop_assert_eq!(file_pattern1, file_pattern2);
        }

        #[test]
        fn file_pattern_matches_filepath(
            pattern_string in FILEPATH_PATTERN,
            file_wildcard_string in r"[\p{Alphabetic}\p{gc=Number} _\-]+",
            dir_wildcard_string in r"[\p{Alphabetic}\p{gc=Number} _\-]+/",
        ) {
            let filepath = pattern_string.replace("**", &dir_wildcard_string).replace('*', &file_wildcard_string);

            let file_pattern = FilePattern::from_str(&pattern_string)
                .unwrap_or_else(|err| panic!("failed to parse FilePattern: {err}"));

            prop_assert!(file_pattern.is_match(&filepath));
        }
    }
}

const FILEPATH_PATTERN: &str = r"((\.\./)|(/))?([\p{Alphabetic}\p{gc=Number} _\-.*]+/)*[\p{Alphabetic}\p{gc=Number} _\-.*]+\.[\p{Alphabetic}\p{gc=Number} _\-*]*";
static EXCLUDED_FILES_PATTERN: LazyLock<String> =
    LazyLock::new(|| String::from(FILEPATH_PATTERN) + "(\\|" + FILEPATH_PATTERN + ")*");

mod excluded_files {
    use super::*;

    proptest! {
        #[test]
        fn valid_file_pattern_chars(
            chars in r"[\p{Alphabetic}\p{gc=Number} _\-./*]*",
        ) {
            let invalid = scan_for_invalid_characters(&chars);

            prop_assert!(invalid.is_empty());
        }

        #[test]
        fn invalid_file_pattern_chars(
            chars in r"[^\p{Alphabetic}\p{gc=Number} _\-./*]*",
        ) {
            let invalid = scan_for_invalid_characters(&chars);

            prop_assert_eq!(invalid, chars.chars().collect::<Vec<_>>());
        }
    }

    #[test]
    fn parse_from_empty_string() {
        let excluded_files = ExcludedFiles::from_str("")
            .unwrap_or_else(|err| panic!("failed to parse ExcludedFilenames: {err}"))
            .pattern
            .into_iter()
            .map(|p| p.regex.to_string())
            .collect::<Vec<_>>();

        assert_that!(excluded_files).is_empty();
    }

    #[test]
    fn parse_from_pattern_with_pipechar() {
        let excluded_files = ExcludedFiles::from_str(".env|TODO.txt")
            .unwrap_or_else(|err| panic!("failed to parse ExcludedFilenames: {err}"))
            .pattern
            .into_iter()
            .map(|p| p.regex.to_string())
            .collect::<Vec<_>>();

        assert_that!(excluded_files)
            .contains_exactly(vec!["^\\.env$".to_string(), "^TODO\\.txt$".to_string()]);
    }

    #[test]
    fn parse_from_pattern_with_wildcards() {
        let excluded_files = ExcludedFiles::from_str(".*|TODO*")
            .unwrap_or_else(|err| panic!("failed to parse ExcludedFilenames: {err}"))
            .pattern
            .into_iter()
            .map(|p| p.regex.to_string())
            .collect::<Vec<_>>();

        assert_that!(excluded_files)
            .contains_exactly(vec!["^\\.[^/]*$".to_string(), "^TODO[^/]*$".to_string()]);
    }

    #[test]
    fn parse_from_pattern_with_wildcards_and_slash() {
        let excluded_files = ExcludedFiles::from_str("**/.keep|README*|TODO.txt")
            .unwrap_or_else(|err| panic!("failed to parse ExcludedFilenames: {err}"))
            .pattern
            .into_iter()
            .map(|p| p.regex.to_string())
            .collect::<Vec<_>>();

        assert_that!(excluded_files).contains_exactly(vec![
            "^.*/\\.keep$".to_string(),
            "^README[^/]*$".to_string(),
            "^TODO\\.txt$".to_string(),
        ]);
    }

    #[test]
    fn parse_from_pattern_with_pipechar_at_start() {
        let result = ExcludedFiles::from_str("|README*|TODO.txt");

        match result {
            Ok(val) => {
                panic!("expected error, but got {:?}", val.pattern);
            },
            Err(error) => {
                assert_that!(error).is_equal_to(FilePatternError::EmptySubPatternNotAllowed);
            },
        }
    }

    #[test]
    fn parse_from_pattern_with_pipechar_at_end() {
        let result = ExcludedFiles::from_str(".*|README*|TODO.txt|");

        match result {
            Ok(val) => {
                panic!("expected error, but got {:?}", val.pattern);
            },
            Err(error) => {
                assert_that!(error).is_equal_to(FilePatternError::EmptySubPatternNotAllowed);
            },
        }
    }

    #[test]
    fn parse_from_pattern_with_two_consecutive_pipechars() {
        let result = ExcludedFiles::from_str(".*||TODO.txt");

        match result {
            Ok(val) => {
                panic!("expected error, but got {:?}", val.pattern);
            },
            Err(error) => {
                assert_that!(error).is_equal_to(FilePatternError::EmptySubPatternNotAllowed);
            },
        }
    }

    #[test]
    fn parse_from_pattern_with_invalid_char() {
        let result = ExcludedFiles::from_str(".*|README:*|TODO.txt");

        match result {
            Ok(val) => {
                panic!("expected error, but got {:?}", val.pattern);
            },
            Err(error) => {
                assert_that!(error).is_equal_to(FilePatternError::InvalidCharacter(vec![':']));
            },
        }
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(50))]

        #[test]
        fn parse_from_pattern_string_and_converted_back_to_pattern_string(
            pattern_string in EXCLUDED_FILES_PATTERN.as_str(),
        ) {
            let excluded_files = ExcludedFiles::from_str(&pattern_string)
                .unwrap_or_else(|err| panic!("failed to parse ExcludedFilenames: {err}"));
            let display_string = excluded_files.to_string();

            prop_assert_eq!(display_string, pattern_string);
        }

        #[test]
        fn two_file_pattern_parsed_from_the_same_pattern_string_are_equal(
            pattern_string in EXCLUDED_FILES_PATTERN.as_str(),
        ) {
            let excluded_files1 = ExcludedFiles::from_str(&pattern_string)
                .unwrap_or_else(|err| panic!("failed to parse ExcludedFilenames: {err}"));
            let excluded_files2 = ExcludedFiles::from_str(&pattern_string)
                .unwrap_or_else(|err| panic!("failed to parse ExcludedFilenames: {err}"));

            prop_assert_eq!(excluded_files1, excluded_files2);
        }
    }

    #[test]
    fn parsed_from_empty_string_is_equal_to_empty_excluded_files() {
        let excluded_files = ExcludedFiles::from_str("")
            .unwrap_or_else(|err| panic!("failed to parse ExcludedFilenames: {err}"));

        assert_that!(excluded_files).is_equal_to(ExcludedFiles::empty());
    }

    #[test]
    fn empty_pattern_does_not_match_empty_path() {
        let excluded_files = ExcludedFiles::from_str("")
            .unwrap_or_else(|err| panic!("failed to parse ExcludedFilenames: {err}"));

        let matches = excluded_files.matches(Path::new(""));

        assert_that!(matches).is_false();
    }

    proptest! {
        #[test]
        fn empty_pattern_does_not_match_any_path(
            path_str in r"((\.\./)|(/))?([\p{Alphabetic}\p{gc=Number} _\-.]+/)*[\p{Alphabetic}\p{gc=Number} _\-]+\.[\p{Alphabetic}\p{gc=Number} _\-]*",
        ) {
            let excluded_files = ExcludedFiles::from_str("")
                .unwrap_or_else(|err| panic!("failed to parse ExcludedFilenames: {err}"));

            let matches = excluded_files.matches(Path::new(&path_str));

            prop_assert!(!matches);
        }

        #[test]
        fn single_subpattern_without_wildcards_matches_same_path(
            path_str in r"((\.\./)|(/))?([\p{Alphabetic}\p{gc=Number} _\-.]+/)*[\p{Alphabetic}\p{gc=Number} _\-]+\.[\p{Alphabetic}\p{gc=Number} _\-]*",
        ) {
            let excluded_files = ExcludedFiles::from_str(&path_str)
                .unwrap_or_else(|err| panic!("failed to parse ExcludedFilenames: {err}"));

            let matches = excluded_files.matches(Path::new(&path_str));

            prop_assert!(matches);
        }

        #[test]
        fn multiple_subpattern_without_wildcards_matches_path(
            path_str in r"((\.\./)|(/))?([\p{Alphabetic}\p{gc=Number} _\-.]+/)*[\p{Alphabetic}\p{gc=Number} _\-]+\.[\p{Alphabetic}\p{gc=Number} _\-]*",
            pattern1 in r"((\.\./)|(/))?([\p{Alphabetic}\p{gc=Number} _\-.]+/)*[\p{Alphabetic}\p{gc=Number} _\-]+\.[\p{Alphabetic}\p{gc=Number} _\-]*",
            pattern2 in r"((\.\./)|(/))?([\p{Alphabetic}\p{gc=Number} _\-.]+/)*[\p{Alphabetic}\p{gc=Number} _\-]+\.[\p{Alphabetic}\p{gc=Number} _\-]*",
        ) {
            let mut pattern_str = pattern1;
            pattern_str.push('|');
            pattern_str.push_str(&path_str);
            pattern_str.push('|');
            pattern_str.push_str(&pattern2);

            let excluded_files = ExcludedFiles::from_str(&pattern_str)
                .unwrap_or_else(|err| panic!("failed to parse ExcludedFilenames: {err}"));

            let matches = excluded_files.matches(Path::new(&path_str));

            prop_assert!(matches);
        }

        #[test]
        fn pattern_with_slashes_matches_path_with_backslashes(
            pattern in r"[\p{Alphabetic}\p{gc=Number} _\-.]{1,30}(/[\p{Alphabetic}\p{gc=Number} _\-.]{1,30}){1,4}",
        ) {
            let path_str = pattern.replace('/', "\\");
            let excluded_files = ExcludedFiles::from_str(&pattern)
                .unwrap_or_else(|err| panic!("failed to parse ExcludedFilenames: {err}"));

            let matches = excluded_files.matches(Path::new(&path_str));

            prop_assert!(matches);
        }
    }

    #[test]
    fn pattern_with_wildcard_at_the_end_matches_filename() {
        let pattern = "README*";
        let excluded_files = ExcludedFiles::from_str(pattern)
            .unwrap_or_else(|err| panic!("failed to parse ExcludedFilenames: {err}"));

        let matches = excluded_files.matches(Path::new("migrations/README.md"));

        assert_that!(matches).is_true();
    }

    #[test]
    fn pattern_for_filename_does_not_match_directory() {
        let pattern = "TODO*";
        let excluded_files = ExcludedFiles::from_str(pattern)
            .unwrap_or_else(|err| panic!("failed to parse ExcludedFilenames: {err}"));

        let matches = excluded_files.matches(Path::new("TODO/NOTE.md"));

        assert_that!(matches).is_false();
    }

    #[test]
    fn default_excluded_files() {
        let excluded_files = ExcludedFiles::default();

        assert_that!(excluded_files.to_string()).is_equal_to(".*|README*|TODO*".to_string());
    }
}
