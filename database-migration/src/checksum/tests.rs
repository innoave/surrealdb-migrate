use super::*;
use proptest::collection::vec;
use proptest::prelude::*;

proptest! {
    #[test]
    fn hasher_update_and_combine_give_same_result(
        byte_set1 in vec(any::<u8>(), 0..100),
        byte_set2 in vec(any::<u8>(), 0..100),
        byte_set3 in vec(any::<u8>(), 0..100),
    ) {
        let mut hasher_a = Hasher::new();
        hasher_a.update(&byte_set1);
        hasher_a.update(&byte_set2);
        hasher_a.update(&byte_set3);
        let hash_a = hasher_a.finalize();

        let mut hasher1 = Hasher::new();
        hasher1.update(&byte_set1);
        let mut hasher2 = Hasher::new();
        hasher2.update(&byte_set2);
        let mut hasher3 = Hasher::new();
        hasher3.update(&byte_set3);
        let mut hasher_b = Hasher::new();
        hasher_b.combine(&hasher1);
        hasher_b.combine(&hasher2);
        hasher_b.combine(&hasher3);
        let hash_b = hasher_b.finalize();

        prop_assert_eq!(hash_a, hash_b);
    }

    #[test]
    fn hasher_update_once_gives_different_result_than_update_twice(
        byte_set1 in vec(any::<u8>(), 1..100),
        byte_set2 in vec(any::<u8>(), 1..100),
    ) {
        let mut hasher_a1 = Hasher::new();
        hasher_a1.update(&byte_set1);
        let hash_a1 = hasher_a1.finalize();

        let mut hasher_a2 = Hasher::new();
        hasher_a2.update(&byte_set2);
        let hash_a2 = hasher_a2.finalize();

        let mut hasher_b = Hasher::new();
        hasher_b.update(&byte_set1);
        hasher_b.update(&byte_set2);
        let hash_b = hasher_b.finalize();

        prop_assert_ne!(hash_a1, hash_b);
        prop_assert_ne!(hash_a2, hash_b);
    }
}

mod hash_migration_script {
    use super::*;
    use crate::proptest_support::{any_migration, any_script_content};

    proptest! {
        #[test]
        fn can_hash_any_migration_script(
            migration in any_migration(),
            script_content in any_script_content(),
        ) {
            let checksum = hash_migration_script(&migration, &script_content);

            prop_assert_ne!(checksum, Checksum(0));
        }
    }
}
