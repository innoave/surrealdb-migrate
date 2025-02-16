//! A command line tool to manage and apply migrations for a [SurrealDB] database.
//!
//! ## Installation
//!
//! Currently only installation from source via [crates.io] is supported.
//!
//! ### From crates.io (Linux, Mac-OS, Windows)
//!
//! ```console
//! $ cargo install surrealdb-migrate-cli
//! ```
//!
//! This command will install the executable `surmig` on Linux and Mac-OS and
//! `surmig.exe` on Windows.
//!
//! ## Usage
//!
//! Surmig provides several subcommands for different tasks. To get an overview
//! of the available subcommands type:
//!
//! ```console
//! $ surmig help
//! ```
//!
//! or
//!
//! ```console
//! $ surmig --help
//! ```
//!
//! There are general options applicable to most/all subcommands and subcommand
//! specific options. To get a list of options available for a specific
//! subcommand use the `--help` option after the subcommand. For example:
//!
//! ```console
//! $ surmig migrate --help
//! ```
//!
//! ## Configuration
//!
//! In order to work properly with `surmig` it needs some configuration. It can
//! be configured by providing a config-file `surrealdb-migrate.toml` and/or by
//! settings some environment variables. For details on how to configure
//! `surmig` see the [configuration documentation][surrealdb_migrate::settings].
//!
//! Some settings can be provided via command-line options. To get the available
//! command line options specify the `--help` option.
//!
//! When a command line option is specified it overwrites the related
//! environment variable and the related setting in the configuration file.
//!
//! [crates.io]: https://crates.io/crates/surrealdb-migrate-cli
//! [SurrealDB]: https://surrealdb.com

mod args;
mod create_cmd;
mod list_cmd;
mod migrate_cmd;
mod revert_cmd;
mod runner;
mod tables;
mod verify_cmd;

use crate::args::{Args, Command};
use clap::Parser;
use color_eyre::eyre::WrapErr;
use color_eyre::Report;
use simplelog::{ConfigBuilder, LevelFilter, SimpleLogger};
use std::path::Path;
use surrealdb_migrate::config::{DbClientConfig, RunnerConfig};
use surrealdb_migrate::db_client::connect_to_database;
use surrealdb_migrate::settings::Settings;

#[tokio::main]
async fn main() -> Result<(), Report> {
    color_eyre::install()?;

    let args = Args::parse();

    let settings = args.config_dir.map_or_else(Settings::load, |dir| {
        Settings::load_from_dir(Path::new(&dir))
    })?;

    let runner_config = args
        .migrations_folder
        .map_or(settings.runner_config(), |mfd| {
            settings.runner_config().with_migrations_folder(mfd)
        });
    let db_config = args.db_address.map_or(settings.db_client_config(), |dba| {
        settings.db_client_config().with_address(dba)
    });

    run_command(args.command, runner_config, db_config).await?;

    Ok(())
}

async fn run_command(
    command: Command,
    runner_config: RunnerConfig<'_>,
    db_config: DbClientConfig<'_>,
) -> Result<(), Report> {
    match command {
        Command::Create(args) => create_cmd::run(args, runner_config),
        Command::Migrate(args) => {
            SimpleLogger::init(LevelFilter::Info, logger_config())
                .wrap_err("failed to initialize terminal logger")?;
            let db = connect_to_database(&db_config).await?;
            migrate_cmd::run(args, runner_config, db_config, &db).await
        },
        Command::Revert(args) => {
            SimpleLogger::init(LevelFilter::Info, logger_config())
                .wrap_err("failed to initialize terminal logger")?;
            let db = connect_to_database(&db_config).await?;
            revert_cmd::run(args, runner_config, db_config, &db).await
        },
        Command::List(args) => {
            let db = connect_to_database(&db_config).await?;
            list_cmd::run(args, runner_config, &db).await
        },
        Command::Verify(args) => {
            let db = connect_to_database(&db_config).await?;
            verify_cmd::run(args, runner_config, db_config, &db).await
        },
    }
}

fn logger_config() -> simplelog::Config {
    ConfigBuilder::new()
        .set_location_level(LevelFilter::Off)
        .set_max_level(LevelFilter::Off)
        .set_target_level(LevelFilter::Off)
        .set_thread_level(LevelFilter::Off)
        .set_time_level(LevelFilter::Off)
        .build()
}

// workaround for false positive 'unused extern crate' warnings until
// Rust issue [#95513](https://github.com/rust-lang/rust/issues/95513) is fixed
#[cfg(test)]
mod dummy_extern_uses {
    use assert_fs as _;
    use assertor as _;
    use database_migration as _;
    use dotenvy as _;
    use snapbox as _;
    use surrealdb_migrate_db_client as _;
    use testcontainers_modules as _;
}
