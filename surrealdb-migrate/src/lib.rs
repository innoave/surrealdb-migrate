pub mod runner;

//
// re-export other crates of the project for a one-crate-dependency experience
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
}
