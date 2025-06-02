//! A lib to migrate a [SurrealDB] programmatically from within an application
//! or to build a custom migration tool.
//!
//! ## Usage
//!
//! For migrating or reverting a database the easiest way is to configure a
//! [`MigrationRunner`] and use its functions [`MigrationRunner::migrate()`]
//! and [`MigrationRunner::revert()`].
//!
//! ### Example
//!
//! ```no_run
//! use std::path::Path;
//! use anyhow::Context;
//! use surrealdb_migrate::{
//!     config::{DbAuthLevel, DbClientConfig, RunnerConfig},
//!     runner::MigrationRunner,
//!     db_client::connect_to_database
//! };
//!
//! #[tokio::main]
//! async fn main() -> Result<(), anyhow::Error> {
//!     // setup database connection
//!     let db_config = DbClientConfig::default()
//!         .with_address("wss://localhost:9000")
//!         .with_namespace("playground")
//!         .with_database("examples")
//!         .with_auth_level(DbAuthLevel::Database)
//!         .with_username("example.user")
//!         .with_password("s3cr3t");
//!     let db = connect_to_database(&db_config)
//!         .await
//!         .context("failed to connect to examples database")?;
//!
//!     // Instantiate the `MigrationRunner`
//!     let runner_config = RunnerConfig::default()
//!         .with_migrations_folder(Path::new("my_application/migrations"));
//!     let runner = MigrationRunner::new(runner_config);
//!
//!     // Run all forward (up) migrations
//!     runner.migrate(&db).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! For a fully working example see the [run_migrations] example.
//!
//! The configuration of the database connection and the migration runner can be
//! provided in an application specific way or by using the configuration
//! mechanism provided by this crate (see next chapter below).
//!
//! ## Configuring the DB-connection and the Migration-Runner
//!
//! Both the DB-connection and the [`MigrationRunner`] can be configured using
//! the configuration mechanism provided by this crate. The configuration
//! mechanism is gated behind the optional crate feature `config`.
//!
//! This mechanism loads the configuration settings from the configuration file
//! `surrealdb-migrate.toml` and from environment variables.
//! See the [`settings`] module for more details.
//!
//! ## Crate features
//!
//! | Feature  | Description                                                                                                 | Default |
//! |----------|-------------------------------------------------------------------------------------------------------------|:-------:|
//! | `config` | Provides a configuration mechanism for the DB-connection and the migration runner (see [`settings`] module) | no      |
//!
//! [run_migrations]: https://github.com/innoave/surrealdb-migrate/blob/main/surrealdb-migrate/examples/run_migrations.rs
//! [SurrealDB]: https://surrealdb.com
#![doc(html_root_url = "https://docs.rs/surrealdb-migrate/0.1.0")]

// imports for crate level doc
#[allow(unused_imports)]
use runner::MigrationRunner;

pub mod runner;

//
// re-export other crates of this project for a one-crate-dependency experience
// for users of this crate.
//

#[doc(inline)]
pub use database_migration::*;
#[doc(inline)]
pub use database_migration_files as files;
#[doc(inline)]
#[cfg(feature = "config")]
pub use surrealdb_migrate_config as settings;
pub mod db_client {
    #[doc(inline)]
    pub use surrealdb_migrate_db_client::DbConnection;
    #[doc(inline)]
    pub use surrealdb_migrate_db_client::DbError;
    #[doc(inline)]
    pub use surrealdb_migrate_db_client::connect_to_database;
}

// test code snippets in the README.md
#[cfg(doctest)]
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
