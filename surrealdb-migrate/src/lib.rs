#![doc(html_root_url = "https://docs.rs/surrealdb-migrate/0.1")]

pub mod runner;

//
// re-export other crates of this project for a one-crate-dependency experience
// for users of this crate.
//

pub use database_migration::*;
pub use database_migration_files as files;
#[cfg(feature = "config")]
pub use surrealdb_migrate_config as settings;
pub mod db_client {
    pub use surrealdb_migrate_db_client::connect_to_database;
    pub use surrealdb_migrate_db_client::DbConnection;
    pub use surrealdb_migrate_db_client::DbError;
}

// test code snippets in the README.md
#[doc = include_str!("../../README.md")]
#[allow(dead_code)]
type TestExamplesInReadme = ();

// workaround for false positive 'unused extern crate' warnings until
// Rust issue [#95513](https://github.com/rust-lang/rust/issues/95513) is fixed
#[cfg(test)]
mod dummy_extern_uses {
    use anyhow as _;
    use assert_fs as _;
    use color_eyre as _;
    use dotenvy as _;
    use testcontainers_modules as _;
    use tokio as _;
    use version_sync as _;
}
