mod args;
mod create_cmd;
mod list_cmd;
mod migrate_cmd;
mod runner;
mod tables;

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
        Command::Revert(_) => {
            todo!()
        },
        Command::List(args) => {
            let db = connect_to_database(&db_config).await?;
            list_cmd::run(args, runner_config, &db).await
        },
        Command::Verify(_) => {
            todo!()
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
