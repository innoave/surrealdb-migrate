use super::*;
use assertor::*;
use database_migration::config::{
    DEFAULT_EXCLUDED_FILES, DbAuthLevel, DbClientConfig, RunnerConfig,
};

#[test]
fn default_settings_are_as_defined() {
    let settings = Settings::load_from_dir(Path::new("does_not_exist"));

    assert_that!(settings).is_equal_to(Ok(Settings {
        migration: MigrationSettings {
            ignore_checksum: false,
            ignore_order: false,
        },
        files: FilesSettings {
            migrations_folder: "migrations".into(),
            script_extension: ".surql".into(),
            up_script_extension: ".up.surql".into(),
            down_script_extension: ".down.surql".into(),
            exclude: DEFAULT_EXCLUDED_FILES.into(),
        },
        database: DatabaseSettings {
            migrations_table: "migrations".into(),
            address: "ws://localhost:8000".into(),
            username: "root".into(),
            password: "root".into(),
            auth_level: DbAuthLevel::Root,
            namespace: "test".into(),
            database: "test".into(),
            capacity: 20,
        },
    }));
}

#[test]
fn overwrite_settings_from_environment_variables() {
    env::set_var("SURREALDB_MIGRATE_CONFIG_DIR", "fixtures/custom_config_dir");

    env::set_var("SURMIG_MIGRATION_IGNORE_CHECKSUM", "true");
    env::set_var("SURMIG_MIGRATION_IGNORE_ORDER", "false");
    env::set_var(
        "SURMIG_FILES_MIGRATIONS_FOLDER",
        "environment/migration/scripts",
    );
    env::set_var("SURMIG_FILES_EXCLUDE", ".keep|.gitignore|TODO.md");
    env::set_var("SURMIG_FILES_UP_SCRIPT_EXTENSION", ".surql");
    env::set_var("SURMIG_DATABASE_ADDRESS", "wss://localhost:8000");
    env::set_var("SURMIG_DATABASE_AUTH_LEVEL", "Namespace");
    env::set_var("SURMIG_DATABASE_NAMESPACE", "playground");
    env::set_var("SURMIG_DATABASE_CAPACITY", "101");

    let settings = Settings::load();

    assert_that!(settings).is_equal_to(Ok(Settings {
        migration: MigrationSettings {
            ignore_checksum: true,
            ignore_order: false,
        },
        files: FilesSettings {
            migrations_folder: "environment/migration/scripts".into(),
            script_extension: ".surql".into(),
            up_script_extension: ".surql".into(),
            down_script_extension: ".down.surql".into(),
            exclude: ".keep|.gitignore|TODO.md".into(),
        },
        database: DatabaseSettings {
            migrations_table: "schema_version".into(),
            address: "wss://localhost:8000".into(),
            username: "tester".into(),
            password: "s3cr3t".into(),
            auth_level: DbAuthLevel::Namespace,
            namespace: "playground".into(),
            database: "test".into(),
            capacity: 101,
        },
    }));

    env::remove_var("SURMIG_MIGRATION_IGNORE_CHECKSUM");
    env::remove_var("SURMIG_MIGRATION_IGNORE_ORDER");
    env::remove_var("SURMIG_FILES_MIGRATIONS_FOLDER");
    env::remove_var("SURMIG_FILES_EXCLUDE");
    env::remove_var("SURMIG_FILES_UP_SCRIPT_EXTENSION");
    env::remove_var("SURMIG_DATABASE_ADDRESS");
    env::remove_var("SURMIG_DATABASE_AUTH_LEVEL");
    env::remove_var("SURMIG_DATABASE_NAMESPACE");
    env::remove_var("SURMIG_DATABASE_CAPACITY");

    env::remove_var("SURREALDB_MIGRATE_CONFIG_DIR");
}

#[test]
fn load_settings_from_custom_config_directory() {
    env::set_var("SURREALDB_MIGRATE_CONFIG_DIR", "fixtures/custom_config_dir");

    let settings = Settings::load();

    assert_that!(settings).is_equal_to(Ok(Settings {
        migration: MigrationSettings {
            ignore_checksum: false,
            ignore_order: true,
        },
        files: FilesSettings {
            migrations_folder: "database_migration/scripts".into(),
            script_extension: ".surql".into(),
            up_script_extension: ".up.surql".into(),
            down_script_extension: ".down.surql".into(),
            exclude: "**/.*".into(),
        },
        database: DatabaseSettings {
            migrations_table: "schema_version".into(),
            address: "ws://localhost:8000".into(),
            username: "tester".into(),
            password: "s3cr3t".into(),
            auth_level: DbAuthLevel::Database,
            namespace: "test".into(),
            database: "test".into(),
            capacity: 99,
        },
    }));

    env::remove_var("SURREALDB_MIGRATE_CONFIG_DIR");
}

#[test]
fn get_runner_config_from_settings() {
    let settings = Settings::load_from_dir(Path::new("fixtures/runner_config"))
        .expect("failed to load settings");

    let runner_config = settings.runner_config();

    assert_that!(runner_config).is_equal_to(RunnerConfig {
        migrations_folder: Path::new("database_migration/migrations").into(),
        excluded_files: ".keep|.*ignore|README*|TODO*|FIXME*"
            .parse()
            .unwrap_or_else(|err| panic!("invalid excluded files string: {err}")),
        migrations_table: "migration_executions".into(),
        ignore_checksum: true,
        ignore_order: false,
    });
}

#[test]
fn get_db_client_config_from_settings() {
    let settings = Settings::load_from_dir(Path::new("fixtures/db_client_config"))
        .expect("failed to load settings");

    let db_client_config = settings.db_client_config();

    assert_that!(db_client_config).is_equal_to(DbClientConfig {
        address: "wss://localhost:9000".into(),
        namespace: "playground".into(),
        database: "thegame".into(),
        auth_level: DbAuthLevel::Database,
        username: "player".into(),
        password: "s3cr3t".into(),
        capacity: 150,
    });
}
