use super::*;
use crate::checksum::Checksum;
use crate::migration::MigrationKind;
use crate::test_dsl::{applicable_migrations, executed_migrations, key, script_content};
use assertor::*;
use chrono::DateTime;
use std::time::Duration;

mod migrate {
    use super::*;

    #[test]
    fn list_migrations_to_apply_no_executions() {
        let defined = script_content([ScriptContent {
            key: key("20250109_125900"),
            kind: MigrationKind::Up,
            content: r#"LET $data = ["J. Jonah Jameson", "James Earl Jones"];"#.into(),
            checksum: Checksum(0x_08C11ABD),
        }]);

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
        let defined = script_content([
            ScriptContent {
                key: key("20250109_125900"),
                kind: MigrationKind::Up,
                content: r#"LET $data = ["J. Jonah Jameson", "James Earl Jones"];"#.into(),
                checksum: Checksum(0x_08C11ABD),
            },
            ScriptContent {
                key: key("20250110_090059"),
                kind: MigrationKind::Up,
                content: r#"LET $data = ["Alice Sulton", "Tamara Jackson"];"#.into(),
                checksum: Checksum(0x_DD081E07),
            },
        ]);

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
        let defined = script_content([
            ScriptContent {
                key: key("20250109_125900"),
                kind: MigrationKind::Baseline,
                content: r#"LET $data = ["J. Jonah Jameson", "James Earl Jones"];"#.into(),
                checksum: Checksum(0x_08C11ABD),
            },
            ScriptContent {
                key: key("20250109_130000"),
                kind: MigrationKind::Down,
                content: r#"LET $data = ["Alice Sulton", "Tamara Jackson"];"#.into(),
                checksum: Checksum(0x_DD081E07),
            },
            ScriptContent {
                key: key("20250110_090059"),
                kind: MigrationKind::Up,
                content: r#"LET $data = ["Simon Says", "Lucy May"];"#.into(),
                checksum: Checksum(0x_AA0137FA),
            },
        ]);

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
        let defined = script_content([ScriptContent {
            key: key("20250109_125900"),
            kind: MigrationKind::Down,
            content: r#"LET $data = ["J. Jonah Jameson", "James Earl Jones"];"#.into(),
            checksum: Checksum(0x_08C11ABD),
        }]);

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
        let defined = script_content([
            ScriptContent {
                key: key("20250109_125900"),
                kind: MigrationKind::Down,
                content: r#"LET $data = ["J. Jonah Jameson", "James Earl Jones"];"#.into(),
                checksum: Checksum(0x_08C11ABD),
            },
            ScriptContent {
                key: key("20250110_090059"),
                kind: MigrationKind::Down,
                content: r#"LET $data = ["Alice Sulton", "Tamara Jackson"];"#.into(),
                checksum: Checksum(0x_DD081E07),
            },
        ]);

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
        let defined = script_content([
            ScriptContent {
                key: key("20250109_125900"),
                kind: MigrationKind::Baseline,
                content: r#"LET $data = ["J. Jonah Jameson", "James Earl Jones"];"#.into(),
                checksum: Checksum(0x_08C11ABD),
            },
            ScriptContent {
                key: key("20250109_130000"),
                kind: MigrationKind::Down,
                content: r#"LET $data = ["Alice Sulton", "Tamara Jackson"];"#.into(),
                checksum: Checksum(0x_DD081E07),
            },
            ScriptContent {
                key: key("20250110_090059"),
                kind: MigrationKind::Up,
                content: r#"LET $data = ["Simon Says", "Lucy May"];"#.into(),
                checksum: Checksum(0x_AA0137FA),
            },
        ]);

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
