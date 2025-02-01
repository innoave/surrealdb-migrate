pub mod db;

pub fn load_environment_variables() {
    let _env_file =
        dotenvy::from_filename("test.env").expect("failed to load environment variables");
}

// workaround for false positive 'unused extern crate' warnings until
// Rust issue [#95513](https://github.com/rust-lang/rust/issues/95513) is fixed
#[cfg(test)]
mod dummy_extern_uses {
    use assert_fs as _;
    use assertor as _;
    use chrono as _;
    use color_eyre as _;
    use database_migration_files as _;
    use indexmap as _;
    use log as _;
    #[cfg(feature = "config")]
    use surrealdb_migrate_config as _;
    use surrealdb_migrate_db_client as _;
}
