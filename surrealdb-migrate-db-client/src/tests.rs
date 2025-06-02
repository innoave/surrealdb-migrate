use super::*;
use assertor::*;

mod extract_table_definition_version {
    use super::*;

    #[test]
    fn from_comment_version_1_0() {
        let definition =
            "DEFINE TABLE migrations SCHEMAFULL TYPE NORMAL COMMENT 'version:1.0' PERMISSIONS FULL";

        let version = extract_table_definition_version(definition);

        assert_that!(version).is_equal_to(Some("1.0".to_string()));
    }

    #[test]
    fn from_comment_version_10_42() {
        let definition = "DEFINE TABLE migrations SCHEMAFULL TYPE NORMAL COMMENT 'version:10.42' PERMISSIONS FULL";

        let version = extract_table_definition_version(definition);

        assert_that!(version).is_equal_to(Some("10.42".to_string()));
    }

    #[test]
    fn from_comment_with_no_version_attribute() {
        let definition = "DEFINE TABLE migrations SCHEMAFULL TYPE NORMAL COMMENT 'some comment:10.42' PERMISSIONS FULL";

        let version = extract_table_definition_version(definition);

        assert_that!(version).is_none();
    }

    #[test]
    fn table_definition_without_comment() {
        let definition = "DEFINE TABLE migrations SCHEMAFULL TYPE NORMAL PERMISSIONS FULL";

        let version = extract_table_definition_version(definition);

        assert_that!(version).is_none();
    }

    #[test]
    fn empty_table_definition() {
        let definition = "";

        let version = extract_table_definition_version(definition);

        assert_that!(version).is_none();
    }
}
