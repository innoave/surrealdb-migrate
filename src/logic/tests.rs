use super::*;
use crate::checksum::Checksum;
use crate::migration::MigrationKind;
use crate::test_dsl::{applicable_migrations, executed_migrations, key};
use assertor::*;
use chrono::DateTime;
use std::path::Path;
use std::time::Duration;

mod verify {
    use super::*;

    #[test]
    fn list_changed_migrations_one_of_two_has_different_checksum() {
        let defined = vec![
            ScriptContent {
                key: key("20250109_125900"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250109_125900_create_name_set_one.surql").into(),
                content: r#"LET $data = ["J. Jonah Jameson", "James Earl Jones"];"#.into(),
                checksum: Checksum(0x_08C11ABD),
            },
            ScriptContent {
                key: key("20250110_090059"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250110_090059_create_name_set_two.surql").into(),
                content: r#"LET $data = ["Alice Sulton", "Tamara Jackson"];"#.into(),
                checksum: Checksum(0x_DD081E07),
            },
        ];

        let executed = executed_migrations([
            Execution {
                key: key("20250109_125900"),
                applied_rank: 1,
                applied_by: "some.user".into(),
                checksum: Checksum(0x_18C11ABD),
                applied_at: DateTime::default(),
                execution_time: Duration::default(),
            },
            Execution {
                key: key("20250110_090059"),
                applied_rank: 1,
                applied_by: "some.user".into(),
                checksum: Checksum(0x_DD081E07),
                applied_at: DateTime::default(),
                execution_time: Duration::default(),
            },
        ]);

        let verify = Verify::default();

        let problematic = verify.list_changed_migrations(&defined, &executed);

        assert_that!(problematic).contains_exactly_in_order(vec![ProblematicMigration {
            key: key("20250109_125900"),
            kind: MigrationKind::Up,
            script_path: Path::new("migrations/20250109_125900_create_name_set_one.surql").into(),
            problem: Problem::ChecksumMismatch {
                definition_checksum: Checksum(0x_08C11ABD),
                execution_checksum: Checksum(0x_18C11ABD),
            },
        }]);
    }

    #[test]
    fn list_changed_migrations_with_option_ignore_checksums_one_of_two_has_different_checksum() {
        let defined = vec![
            ScriptContent {
                key: key("20250109_125900"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250109_125900_create_name_set_one.surql").into(),
                content: r#"LET $data = ["J. Jonah Jameson", "James Earl Jones"];"#.into(),
                checksum: Checksum(0x_08C11ABD),
            },
            ScriptContent {
                key: key("20250110_090059"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250110_090059_create_name_set_two.surql").into(),
                content: r#"LET $data = ["Alice Sulton", "Tamara Jackson"];"#.into(),
                checksum: Checksum(0x_DD081E07),
            },
        ];

        let executed = executed_migrations([
            Execution {
                key: key("20250109_125900"),
                applied_rank: 1,
                applied_by: "some.user".into(),
                checksum: Checksum(0x_18C11ABD),
                applied_at: DateTime::default(),
                execution_time: Duration::default(),
            },
            Execution {
                key: key("20250110_090059"),
                applied_rank: 1,
                applied_by: "some.user".into(),
                checksum: Checksum(0x_DD081E07),
                applied_at: DateTime::default(),
                execution_time: Duration::default(),
            },
        ]);

        let verify = Verify::default().with_ignore_checksums(true);

        let problematic = verify.list_changed_migrations(&defined, &executed);

        assert_that!(problematic).is_empty();
    }

    #[test]
    fn list_changed_migrations_none_of_two_has_different_checksum() {
        let defined = vec![
            ScriptContent {
                key: key("20250109_125900"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250109_125900_create_name_set_one.surql").into(),
                content: r#"LET $data = ["J. Jonah Jameson", "James Earl Jones"];"#.into(),
                checksum: Checksum(0x_08C11ABD),
            },
            ScriptContent {
                key: key("20250110_090059"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250110_090059_create_name_set_two.surql").into(),
                content: r#"LET $data = ["Alice Sulton", "Tamara Jackson"];"#.into(),
                checksum: Checksum(0x_DD081E07),
            },
        ];

        let executed = executed_migrations([
            Execution {
                key: key("20250109_125900"),
                applied_rank: 1,
                applied_by: "some.user".into(),
                checksum: Checksum(0x_08C11ABD),
                applied_at: DateTime::default(),
                execution_time: Duration::default(),
            },
            Execution {
                key: key("20250110_090059"),
                applied_rank: 1,
                applied_by: "some.user".into(),
                checksum: Checksum(0x_DD081E07),
                applied_at: DateTime::default(),
                execution_time: Duration::default(),
            },
        ]);

        let verify = Verify::default();

        let problematic = verify.list_changed_migrations(&defined, &executed);

        assert_that!(problematic).is_empty();
    }

    #[test]
    fn list_changed_migrations_both_of_two_have_different_checksum_but_one_is_not_executed() {
        let defined = vec![
            ScriptContent {
                key: key("20250109_125900"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250109_125900_create_name_set_one.surql").into(),
                content: r#"LET $data = ["J. Jonah Jameson", "James Earl Jones"];"#.into(),
                checksum: Checksum(0x_08C11ABD),
            },
            ScriptContent {
                key: key("20250110_090059"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250110_090059_create_name_set_two.surql").into(),
                content: r#"LET $data = ["Alice Sulton", "Tamara Jackson"];"#.into(),
                checksum: Checksum(0x_AD081E07),
            },
        ];

        let executed = executed_migrations([Execution {
            key: key("20250109_125900"),
            applied_rank: 1,
            applied_by: "some.user".into(),
            checksum: Checksum(0x_18C11ABD),
            applied_at: DateTime::default(),
            execution_time: Duration::default(),
        }]);

        let verify = Verify::default();

        let problematic = verify.list_changed_migrations(&defined, &executed);

        assert_that!(problematic).contains_exactly_in_order(vec![ProblematicMigration {
            key: key("20250109_125900"),
            kind: MigrationKind::Up,
            script_path: Path::new("migrations/20250109_125900_create_name_set_one.surql").into(),
            problem: Problem::ChecksumMismatch {
                definition_checksum: Checksum(0x_08C11ABD),
                execution_checksum: Checksum(0x_18C11ABD),
            },
        }]);
    }

    #[test]
    fn list_changed_migrations_both_of_two_have_different_checksum_but_none_is_executed() {
        let defined = vec![
            ScriptContent {
                key: key("20250109_125900"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250109_125900_create_name_set_one.surql").into(),
                content: r#"LET $data = ["J. Jonah Jameson", "James Earl Jones"];"#.into(),
                checksum: Checksum(0x_08C11ABD),
            },
            ScriptContent {
                key: key("20250110_090059"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250110_090059_create_name_set_two.surql").into(),
                content: r#"LET $data = ["Alice Sulton", "Tamara Jackson"];"#.into(),
                checksum: Checksum(0x_AD081E07),
            },
        ];

        let executed = executed_migrations([]);

        let verify = Verify::default();

        let problematic = verify.list_changed_migrations(&defined, &executed);

        assert_that!(problematic).is_empty();
    }

    #[test]
    fn list_changed_migrations_no_migrations_defined() {
        let defined = vec![];

        let executed = executed_migrations([
            Execution {
                key: key("20250109_125900"),
                applied_rank: 1,
                applied_by: "some.user".into(),
                checksum: Checksum(0x_18C11ABD),
                applied_at: DateTime::default(),
                execution_time: Duration::default(),
            },
            Execution {
                key: key("20250110_090059"),
                applied_rank: 1,
                applied_by: "some.user".into(),
                checksum: Checksum(0x_DD081E07),
                applied_at: DateTime::default(),
                execution_time: Duration::default(),
            },
        ]);

        let verify = Verify::default();

        let problematic = verify.list_changed_migrations(&defined, &executed);

        assert_that!(problematic).is_empty();
    }

    #[test]
    fn list_changed_migrations_both_of_two_have_different_checksum_but_one_is_a_backward_migration()
    {
        let defined = vec![
            ScriptContent {
                key: key("20250109_125900"),
                kind: MigrationKind::Down,
                path: Path::new("migrations/20250109_125900_create_name_set_one.surql").into(),
                content: r#"LET $data = ["J. Jonah Jameson", "James Earl Jones"];"#.into(),
                checksum: Checksum(0x_18C11ABD),
            },
            ScriptContent {
                key: key("20250110_090059"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250110_090059_create_name_set_two.surql").into(),
                content: r#"LET $data = ["Alice Sulton", "Tamara Jackson"];"#.into(),
                checksum: Checksum(0x_AD081E07),
            },
        ];

        let executed = executed_migrations([
            Execution {
                key: key("20250109_125900"),
                applied_rank: 1,
                applied_by: "some.user".into(),
                checksum: Checksum(0x_08C11ABD),
                applied_at: DateTime::default(),
                execution_time: Duration::default(),
            },
            Execution {
                key: key("20250110_090059"),
                applied_rank: 1,
                applied_by: "some.user".into(),
                checksum: Checksum(0x_DD081E07),
                applied_at: DateTime::default(),
                execution_time: Duration::default(),
            },
        ]);

        let verify = Verify::default();

        let problematic = verify.list_changed_migrations(&defined, &executed);

        assert_that!(problematic).contains_exactly_in_order(vec![ProblematicMigration {
            key: key("20250110_090059"),
            kind: MigrationKind::Up,
            script_path: Path::new("migrations/20250110_090059_create_name_set_two.surql").into(),
            problem: Problem::ChecksumMismatch {
                definition_checksum: Checksum(0x_AD081E07),
                execution_checksum: Checksum(0x_DD081E07),
            },
        }]);
    }
}

mod migrate {
    use super::*;

    #[test]
    fn list_migrations_to_apply_no_executions() {
        let defined = vec![ScriptContent {
            key: key("20250109_125900"),
            kind: MigrationKind::Up,
            path: Path::new("migrations/20250109_125900_create_name_set_one.surql").into(),
            content: r#"LET $data = ["J. Jonah Jameson", "James Earl Jones"];"#.into(),
            checksum: Checksum(0x_08C11ABD),
        }];

        let executed = executed_migrations([]);

        let applicable = Migrate.list_migrations_to_apply(&defined, &executed);

        assert_that!(applicable.iter()).contains_exactly_in_order(
            applicable_migrations([ApplicableMigration {
                key: key("20250109_125900"),
                kind: MigrationKind::Up,
                script_content: r#"LET $data = ["J. Jonah Jameson", "James Earl Jones"];"#.into(),
                checksum: Checksum(0x_08C11ABD),
            }])
            .iter(),
        );
    }

    #[test]
    fn list_migrations_to_apply_one_of_two_migrations_applied() {
        let defined = vec![
            ScriptContent {
                key: key("20250109_125900"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250109_125900_create_name_set_one.surql").into(),
                content: r#"LET $data = ["J. Jonah Jameson", "James Earl Jones"];"#.into(),
                checksum: Checksum(0x_08C11ABD),
            },
            ScriptContent {
                key: key("20250110_090059"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250110_090059_create_name_set_two.surql").into(),
                content: r#"LET $data = ["Alice Sulton", "Tamara Jackson"];"#.into(),
                checksum: Checksum(0x_DD081E07),
            },
        ];

        let executed = executed_migrations([Execution {
            key: key("20250109_125900"),
            applied_rank: 1,
            applied_by: "some.user".into(),
            checksum: Checksum(0x_08C11ABD),
            applied_at: DateTime::default(),
            execution_time: Duration::default(),
        }]);

        let applicable = Migrate.list_migrations_to_apply(&defined, &executed);

        assert_that!(applicable.iter()).contains_exactly_in_order(
            applicable_migrations([ApplicableMigration {
                key: key("20250110_090059"),
                kind: MigrationKind::Up,
                script_content: r#"LET $data = ["Alice Sulton", "Tamara Jackson"];"#.into(),
                checksum: Checksum(0x_DD081E07),
            }])
            .iter(),
        );
    }

    #[test]
    fn list_migrations_to_apply_lists_forward_scripts_only() {
        let defined = vec![
            ScriptContent {
                key: key("20250109_125900"),
                kind: MigrationKind::Baseline,
                path: Path::new("migrations/20250109_125900_create_name_set_one.surql").into(),
                content: r#"LET $data = ["J. Jonah Jameson", "James Earl Jones"];"#.into(),
                checksum: Checksum(0x_08C11ABD),
            },
            ScriptContent {
                key: key("20250109_130000"),
                kind: MigrationKind::Down,
                path: Path::new("migrations/20250109_130000_create_name_set_two.surql").into(),
                content: r#"LET $data = ["Alice Sulton", "Tamara Jackson"];"#.into(),
                checksum: Checksum(0x_DD081E07),
            },
            ScriptContent {
                key: key("20250110_090059"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250110_090059_create_name_set_three.surql").into(),
                content: r#"LET $data = ["Simon Says", "Lucy May"];"#.into(),
                checksum: Checksum(0x_AA0137FA),
            },
        ];

        let executed = executed_migrations([]);

        let applicable = Migrate.list_migrations_to_apply(&defined, &executed);

        assert_that!(applicable.iter()).contains_exactly_in_order(
            applicable_migrations([
                ApplicableMigration {
                    key: key("20250109_125900"),
                    kind: MigrationKind::Baseline,
                    script_content: r#"LET $data = ["J. Jonah Jameson", "James Earl Jones"];"#
                        .into(),
                    checksum: Checksum(0x_08C11ABD),
                },
                ApplicableMigration {
                    key: key("20250110_090059"),
                    kind: MigrationKind::Up,
                    script_content: r#"LET $data = ["Simon Says", "Lucy May"];"#.into(),
                    checksum: Checksum(0x_AA0137FA),
                },
            ])
            .iter(),
        );
    }
}

mod revert {
    use super::*;

    #[test]
    fn list_migrations_to_apply_up_migration_applied() {
        let defined = vec![ScriptContent {
            key: key("20250109_125900"),
            kind: MigrationKind::Down,
            path: Path::new("migrations/20250109_125900_create_name_set_one.surql").into(),
            content: r#"LET $data = ["J. Jonah Jameson", "James Earl Jones"];"#.into(),
            checksum: Checksum(0x_08C11ABD),
        }];

        let executed = executed_migrations([Execution {
            key: key("20250109_125900"),
            applied_rank: 1,
            applied_by: "some.user".into(),
            checksum: Checksum(0x_08C11ABD),
            applied_at: DateTime::default(),
            execution_time: Duration::default(),
        }]);

        let applicable = Revert.list_migrations_to_apply(&defined, &executed);

        assert_that!(applicable.iter()).contains_exactly_in_order(
            applicable_migrations([ApplicableMigration {
                key: key("20250109_125900"),
                kind: MigrationKind::Down,
                script_content: r#"LET $data = ["J. Jonah Jameson", "James Earl Jones"];"#.into(),
                checksum: Checksum(0x_08C11ABD),
            }])
            .iter(),
        );
    }

    #[test]
    fn list_migrations_to_apply_one_of_two_migrations_applied() {
        let defined = vec![
            ScriptContent {
                key: key("20250109_125900"),
                kind: MigrationKind::Down,
                path: Path::new("migrations/20250109_125900_create_name_set_one.surql").into(),
                content: r#"LET $data = ["J. Jonah Jameson", "James Earl Jones"];"#.into(),
                checksum: Checksum(0x_08C11ABD),
            },
            ScriptContent {
                key: key("20250110_090059"),
                kind: MigrationKind::Down,
                path: Path::new("migrations/20250110_090059_create_name_set_two.surql").into(),
                content: r#"LET $data = ["Alice Sulton", "Tamara Jackson"];"#.into(),
                checksum: Checksum(0x_DD081E07),
            },
        ];

        let executed = executed_migrations([Execution {
            key: key("20250109_125900"),
            applied_rank: 1,
            applied_by: "some.user".into(),
            checksum: Checksum(0x_08C11ABD),
            applied_at: DateTime::default(),
            execution_time: Duration::default(),
        }]);

        let applicable = Revert.list_migrations_to_apply(&defined, &executed);

        assert_that!(applicable.iter()).contains_exactly_in_order(
            applicable_migrations([ApplicableMigration {
                key: key("20250109_125900"),
                kind: MigrationKind::Down,
                script_content: r#"LET $data = ["J. Jonah Jameson", "James Earl Jones"];"#.into(),
                checksum: Checksum(0x_08C11ABD),
            }])
            .iter(),
        );
    }

    #[test]
    fn list_migrations_to_apply_lists_backward_scripts_only() {
        let defined = vec![
            ScriptContent {
                key: key("20250109_125900"),
                kind: MigrationKind::Baseline,
                path: Path::new("migrations/20250109_125900_create_name_set_one.surql").into(),
                content: r#"LET $data = ["J. Jonah Jameson", "James Earl Jones"];"#.into(),
                checksum: Checksum(0x_08C11ABD),
            },
            ScriptContent {
                key: key("20250109_130000"),
                kind: MigrationKind::Down,
                path: Path::new("migrations/20250109_130000_create_name_set_two.surql").into(),
                content: r#"LET $data = ["Alice Sulton", "Tamara Jackson"];"#.into(),
                checksum: Checksum(0x_DD081E07),
            },
            ScriptContent {
                key: key("20250110_090059"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250110_090059_create_name_set_three.surql").into(),
                content: r#"LET $data = ["Simon Says", "Lucy May"];"#.into(),
                checksum: Checksum(0x_AA0137FA),
            },
        ];

        let executed = executed_migrations([
            Execution {
                key: key("20250109_125900"),
                applied_rank: 1,
                applied_by: "some.user".into(),
                checksum: Checksum(0x_08C11ABD),
                applied_at: DateTime::default(),
                execution_time: Duration::default(),
            },
            Execution {
                key: key("20250109_130000"),
                applied_rank: 2,
                applied_by: "some.user".into(),
                checksum: Checksum(0x_DD081E07),
                applied_at: DateTime::default(),
                execution_time: Duration::default(),
            },
            Execution {
                key: key("20250110_090059"),
                applied_rank: 3,
                applied_by: "some.user".into(),
                checksum: Checksum(0x_AA0137FA),
                applied_at: DateTime::default(),
                execution_time: Duration::default(),
            },
        ]);

        let applicable = Revert.list_migrations_to_apply(&defined, &executed);

        assert_that!(applicable.iter()).contains_exactly_in_order(
            applicable_migrations([ApplicableMigration {
                key: key("20250109_130000"),
                kind: MigrationKind::Down,
                script_content: r#"LET $data = ["Alice Sulton", "Tamara Jackson"];"#.into(),
                checksum: Checksum(0x_DD081E07),
            }])
            .iter(),
        );
    }
}
