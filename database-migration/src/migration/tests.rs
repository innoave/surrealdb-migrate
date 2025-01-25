use super::*;
use crate::proptest_support::any_migration_kind;
use assertor::*;
use proptest::prelude::*;

mod migration_kind {
    use super::*;

    proptest! {
        #[test]
        fn any_migration_kind_is_any(
            migration_kind in any_migration_kind()
        ) {
            assert_that!(migration_kind.is_any()).is_true();
        }
    }

    proptest! {
        #[test]
        fn any_migration_kind_but_down_is_forward(
            migration_kind in any_migration_kind().prop_filter("is not down", |kind| *kind != MigrationKind::Down)
        ) {
            assert_that!(migration_kind.is_forward()).is_true();
        }
    }

    #[test]
    fn down_migration_is_backward() {
        assert_that!(MigrationKind::Down.is_backward()).is_true();
    }
}
