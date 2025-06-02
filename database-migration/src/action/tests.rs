use super::*;
use crate::checksum::Checksum;
use crate::migration::MigrationKind;
use crate::test_dsl::{applicable_migrations, executed_migrations, key};
use assertor::*;
use chrono::DateTime;
use std::path::Path;
use std::time::Duration;

mod checks {
    #![allow(clippy::iter_on_single_items)]

    use super::*;

    #[test]
    fn checks_with_checksum_only() {
        let checks = Checks::only(Check::Checksum);

        assert_that!(checks.iter()).contains_exactly([Check::Checksum].into_iter());
    }

    #[test]
    fn checks_with_order_only() {
        let checks = Checks::only(Check::Order);

        assert_that!(checks.iter()).contains_exactly([Check::Order].into_iter());
    }

    #[test]
    fn a_check_can_be_converted_to_checks() {
        let checks = Checks::from(Check::Checksum);

        assert_that!(checks.iter()).contains_exactly([Check::Checksum].into_iter());
    }

    #[test]
    fn none_checks_contains_no_check() {
        let checks = Checks::none();

        assert_that!(checks.iter()).is_empty();
    }

    #[test]
    fn all_checks_contains_all_check_variants() {
        let checks = Checks::all();

        assert_that!(checks.iter()).contains_exactly([Check::Checksum, Check::Order].into_iter());
    }

    #[test]
    fn check_variants_can_be_added() {
        let checks = Check::Checksum + Check::Order;

        assert_that!(checks.iter()).contains_exactly([Check::Checksum, Check::Order].into_iter());
    }

    #[test]
    fn a_check_can_be_added_to_existing_checks() {
        let mut checks = Checks::from(Check::Order);

        checks += Check::Checksum;

        assert_that!(checks.into_iter())
            .contains_exactly([Check::Checksum, Check::Order].into_iter());
    }
}

mod verify {
    use super::*;

    #[test]
    fn list_out_of_order_first_and_third_migration_out_of_four_are_applied() {
        let defined = vec![
            ScriptContent {
                key: key("20250109_115959"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250109_115959_create_name_set_one.surql").into(),
                content: r#"LET $data = ["J. Jonah Jameson", "James Earl Jones"];"#.into(),
                checksum: Checksum(0x_4D65A4BF),
            },
            ScriptContent {
                key: key("20250109_125900"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250109_125900_create_name_set_two.surql").into(),
                content: r#"LET $data = ["Clair Windsor", "Georg Anderson"];"#.into(),
                checksum: Checksum(0x_8E8B2D8B),
            },
            ScriptContent {
                key: key("20250110_090059"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250110_090059_create_name_set_three.surql").into(),
                content: r#"LET $data = ["Alice Sulton", "Tamara Jackson"];"#.into(),
                checksum: Checksum(0x_587930EA),
            },
            ScriptContent {
                key: key("20250110_090100"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250110_090100_create_name_set_four.surql").into(),
                content: r#"LET $data = ["Peter Burns", "Jennifer Carlson"];"#.into(),
                checksum: Checksum(0x_36C45A48),
            },
        ];

        let executed = executed_migrations([
            Execution {
                key: key("20250109_115959"),
                applied_rank: 1,
                applied_by: "some.user".into(),
                checksum: Checksum(0x_4D65A4BF),
                applied_at: DateTime::default(),
                execution_time: Duration::default(),
            },
            Execution {
                key: key("20250110_090059"),
                applied_rank: 2,
                applied_by: "some.user".into(),
                checksum: Checksum(0x_587930EA),
                applied_at: DateTime::default(),
                execution_time: Duration::default(),
            },
        ]);

        let verify = Verify::default();

        let problematic = verify.list_out_of_order(&defined, &executed);

        assert_that!(problematic).contains_exactly_in_order(vec![ProblematicMigration {
            key: key("20250109_125900"),
            kind: MigrationKind::Up,
            script_path: Path::new("migrations/20250109_125900_create_name_set_two.surql").into(),
            problem: Problem::OutOfOrder {
                last_applied_key: key("20250110_090059"),
            },
        }]);
    }

    #[test]
    fn list_out_of_order_second_and_fourth_migration_out_of_four_are_applied() {
        let defined = vec![
            ScriptContent {
                key: key("20250109_115959"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250109_115959_create_name_set_one.surql").into(),
                content: r#"LET $data = ["J. Jonah Jameson", "James Earl Jones"];"#.into(),
                checksum: Checksum(0x_4D65A4BF),
            },
            ScriptContent {
                key: key("20250109_125900"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250109_125900_create_name_set_two.surql").into(),
                content: r#"LET $data = ["Clair Windsor", "Georg Anderson"];"#.into(),
                checksum: Checksum(0x_8E8B2D8B),
            },
            ScriptContent {
                key: key("20250110_090059"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250110_090059_create_name_set_three.surql").into(),
                content: r#"LET $data = ["Alice Sulton", "Tamara Jackson"];"#.into(),
                checksum: Checksum(0x_587930EA),
            },
            ScriptContent {
                key: key("20250110_090100"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250110_090100_create_name_set_four.surql").into(),
                content: r#"LET $data = ["Peter Burns", "Jennifer Carlson"];"#.into(),
                checksum: Checksum(0x_36C45A48),
            },
        ];

        let executed = executed_migrations([
            Execution {
                key: key("20250109_125900"),
                applied_rank: 1,
                applied_by: "some.user".into(),
                checksum: Checksum(0x_8E8B2D8B),
                applied_at: DateTime::default(),
                execution_time: Duration::default(),
            },
            Execution {
                key: key("20250110_090100"),
                applied_rank: 2,
                applied_by: "some.user".into(),
                checksum: Checksum(0x_36C45A48),
                applied_at: DateTime::default(),
                execution_time: Duration::default(),
            },
        ]);

        let verify = Verify::default();

        let problematic = verify.list_out_of_order(&defined, &executed);

        assert_that!(problematic).contains_exactly_in_order(vec![
            ProblematicMigration {
                key: key("20250109_115959"),
                kind: MigrationKind::Up,
                script_path: Path::new("migrations/20250109_115959_create_name_set_one.surql")
                    .into(),
                problem: Problem::OutOfOrder {
                    last_applied_key: key("20250110_090100"),
                },
            },
            ProblematicMigration {
                key: key("20250110_090059"),
                kind: MigrationKind::Up,
                script_path: Path::new("migrations/20250110_090059_create_name_set_three.surql")
                    .into(),
                problem: Problem::OutOfOrder {
                    last_applied_key: key("20250110_090100"),
                },
            },
        ]);
    }

    #[test]
    fn list_out_of_order_with_ignore_order_option_second_and_fourth_migration_out_of_four_are_applied()
     {
        let defined = vec![
            ScriptContent {
                key: key("20250109_115959"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250109_115959_create_name_set_one.surql").into(),
                content: r#"LET $data = ["J. Jonah Jameson", "James Earl Jones"];"#.into(),
                checksum: Checksum(0x_4D65A4BF),
            },
            ScriptContent {
                key: key("20250109_125900"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250109_125900_create_name_set_two.surql").into(),
                content: r#"LET $data = ["Clair Windsor", "Georg Anderson"];"#.into(),
                checksum: Checksum(0x_8E8B2D8B),
            },
            ScriptContent {
                key: key("20250110_090059"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250110_090059_create_name_set_three.surql").into(),
                content: r#"LET $data = ["Alice Sulton", "Tamara Jackson"];"#.into(),
                checksum: Checksum(0x_587930EA),
            },
            ScriptContent {
                key: key("20250110_090100"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250110_090100_create_name_set_four.surql").into(),
                content: r#"LET $data = ["Peter Burns", "Jennifer Carlson"];"#.into(),
                checksum: Checksum(0x_36C45A48),
            },
        ];

        let executed = executed_migrations([
            Execution {
                key: key("20250109_125900"),
                applied_rank: 1,
                applied_by: "some.user".into(),
                checksum: Checksum(0x_8E8B2D8B),
                applied_at: DateTime::default(),
                execution_time: Duration::default(),
            },
            Execution {
                key: key("20250110_090100"),
                applied_rank: 2,
                applied_by: "some.user".into(),
                checksum: Checksum(0x_36C45A48),
                applied_at: DateTime::default(),
                execution_time: Duration::default(),
            },
        ]);

        let verify = Verify::default().with_ignore_order(true);

        let problematic = verify.list_out_of_order(&defined, &executed);

        assert_that!(problematic).is_empty();
    }

    #[test]
    fn list_out_of_order_input_not_sorted_and_first_is_applied() {
        let defined = vec![
            ScriptContent {
                key: key("20250109_125900"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250109_125900_create_name_set_two.surql").into(),
                content: r#"LET $data = ["Clair Windsor", "Georg Anderson"];"#.into(),
                checksum: Checksum(0x_8E8B2D8B),
            },
            ScriptContent {
                key: key("20250109_115959"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250109_115959_create_name_set_one.surql").into(),
                content: r#"LET $data = ["J. Jonah Jameson", "James Earl Jones"];"#.into(),
                checksum: Checksum(0x_4D65A4BF),
            },
            ScriptContent {
                key: key("20250110_090059"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250110_090059_create_name_set_three.surql").into(),
                content: r#"LET $data = ["Alice Sulton", "Tamara Jackson"];"#.into(),
                checksum: Checksum(0x_587930EA),
            },
            ScriptContent {
                key: key("20250110_090100"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250110_090100_create_name_set_four.surql").into(),
                content: r#"LET $data = ["Peter Burns", "Jennifer Carlson"];"#.into(),
                checksum: Checksum(0x_36C45A48),
            },
        ];

        let executed = executed_migrations([Execution {
            key: key("20250109_115959"),
            applied_rank: 1,
            applied_by: "some.user".into(),
            checksum: Checksum(0x_4D65A4BF),
            applied_at: DateTime::default(),
            execution_time: Duration::default(),
        }]);

        let verify = Verify::default();

        let problematic = verify.list_out_of_order(&defined, &executed);

        assert_that!(problematic).is_empty();
    }

    #[test]
    fn list_out_of_order_input_not_sorted_and_none_applied() {
        let defined = vec![
            ScriptContent {
                key: key("20250109_125900"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250109_125900_create_name_set_two.surql").into(),
                content: r#"LET $data = ["Clair Windsor", "Georg Anderson"];"#.into(),
                checksum: Checksum(0x_8E8B2D8B),
            },
            ScriptContent {
                key: key("20250109_115959"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250109_115959_create_name_set_one.surql").into(),
                content: r#"LET $data = ["J. Jonah Jameson", "James Earl Jones"];"#.into(),
                checksum: Checksum(0x_4D65A4BF),
            },
            ScriptContent {
                key: key("20250110_090059"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250110_090059_create_name_set_three.surql").into(),
                content: r#"LET $data = ["Alice Sulton", "Tamara Jackson"];"#.into(),
                checksum: Checksum(0x_587930EA),
            },
            ScriptContent {
                key: key("20250110_090100"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250110_090100_create_name_set_four.surql").into(),
                content: r#"LET $data = ["Peter Burns", "Jennifer Carlson"];"#.into(),
                checksum: Checksum(0x_36C45A48),
            },
        ];

        let executed = executed_migrations([]);

        let verify = Verify::default();

        let problematic = verify.list_out_of_order(&defined, &executed);

        assert_that!(problematic).is_empty();
    }

    #[test]
    fn list_out_of_order_all_migrations_are_applied() {
        let defined = vec![
            ScriptContent {
                key: key("20250109_115959"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250109_115959_create_name_set_one.surql").into(),
                content: r#"LET $data = ["J. Jonah Jameson", "James Earl Jones"];"#.into(),
                checksum: Checksum(0x_4D65A4BF),
            },
            ScriptContent {
                key: key("20250109_125900"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250109_125900_create_name_set_two.surql").into(),
                content: r#"LET $data = ["Clair Windsor", "Georg Anderson"];"#.into(),
                checksum: Checksum(0x_8E8B2D8B),
            },
            ScriptContent {
                key: key("20250110_090059"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250110_090059_create_name_set_three.surql").into(),
                content: r#"LET $data = ["Alice Sulton", "Tamara Jackson"];"#.into(),
                checksum: Checksum(0x_587930EA),
            },
            ScriptContent {
                key: key("20250110_090100"),
                kind: MigrationKind::Up,
                path: Path::new("migrations/20250110_090100_create_name_set_four.surql").into(),
                content: r#"LET $data = ["Peter Burns", "Jennifer Carlson"];"#.into(),
                checksum: Checksum(0x_36C45A48),
            },
        ];

        let executed = executed_migrations([
            Execution {
                key: key("20250109_115959"),
                applied_rank: 1,
                applied_by: "some.user".into(),
                checksum: Checksum(0x_4D65A4BF),
                applied_at: DateTime::default(),
                execution_time: Duration::default(),
            },
            Execution {
                key: key("20250109_125900"),
                applied_rank: 2,
                applied_by: "some.user".into(),
                checksum: Checksum(0x_8E8B2D8B),
                applied_at: DateTime::default(),
                execution_time: Duration::default(),
            },
            Execution {
                key: key("20250110_090059"),
                applied_rank: 3,
                applied_by: "some.user".into(),
                checksum: Checksum(0x_587930EA),
                applied_at: DateTime::default(),
                execution_time: Duration::default(),
            },
            Execution {
                key: key("20250110_090100"),
                applied_rank: 4,
                applied_by: "some.user".into(),
                checksum: Checksum(0x_36C45A48),
                applied_at: DateTime::default(),
                execution_time: Duration::default(),
            },
        ]);

        let verify = Verify::default();

        let problematic = verify.list_out_of_order(&defined, &executed);

        assert_that!(problematic).is_empty();
    }

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
                applied_rank: 2,
                applied_by: "some.user".into(),
                checksum: Checksum(0x_DD081E07),
                applied_at: DateTime::default(),
                execution_time: Duration::default(),
            },
        ]);

        let verify = Verify::default();

        let problematic = verify.list_changed_after_execution(&defined, &executed);

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
                applied_rank: 2,
                applied_by: "some.user".into(),
                checksum: Checksum(0x_DD081E07),
                applied_at: DateTime::default(),
                execution_time: Duration::default(),
            },
        ]);

        let verify = Verify::default().with_ignore_checksums(true);

        let problematic = verify.list_changed_after_execution(&defined, &executed);

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
                applied_rank: 2,
                applied_by: "some.user".into(),
                checksum: Checksum(0x_DD081E07),
                applied_at: DateTime::default(),
                execution_time: Duration::default(),
            },
        ]);

        let verify = Verify::default();

        let problematic = verify.list_changed_after_execution(&defined, &executed);

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

        let problematic = verify.list_changed_after_execution(&defined, &executed);

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

        let problematic = verify.list_changed_after_execution(&defined, &executed);

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
                applied_rank: 2,
                applied_by: "some.user".into(),
                checksum: Checksum(0x_DD081E07),
                applied_at: DateTime::default(),
                execution_time: Duration::default(),
            },
        ]);

        let verify = Verify::default();

        let problematic = verify.list_changed_after_execution(&defined, &executed);

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
                applied_rank: 2,
                applied_by: "some.user".into(),
                checksum: Checksum(0x_DD081E07),
                applied_at: DateTime::default(),
                execution_time: Duration::default(),
            },
        ]);

        let verify = Verify::default();

        let problematic = verify.list_changed_after_execution(&defined, &executed);

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

        let migrate = Migrate::default();
        let applicable = migrate.list_migrations_to_apply(&defined, &executed);

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

        let migrate = Migrate::default();
        let applicable = migrate.list_migrations_to_apply(&defined, &executed);

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

        let migrate = Migrate::default();
        let applicable = migrate.list_migrations_to_apply(&defined, &executed);

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

        let revert = Revert::default();
        let applicable = revert.list_migrations_to_apply(&defined, &executed);

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

        let revert = Revert::default();
        let applicable = revert.list_migrations_to_apply(&defined, &executed);

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

        let revert = Revert::default();
        let applicable = revert.list_migrations_to_apply(&defined, &executed);

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
