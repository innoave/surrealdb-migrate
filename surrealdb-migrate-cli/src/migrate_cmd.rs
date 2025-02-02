use crate::args::MigrateArgs;
use crate::runner::runner;
use chrono::NaiveDateTime;
use color_eyre::eyre::eyre;
use color_eyre::Report;
use surrealdb_migrate::config::{DbClientConfig, RunnerConfig, MIGRATION_KEY_FORMAT_STR};
use surrealdb_migrate::db_client::DbConnection;
use surrealdb_migrate::result::Migrated;

pub async fn run(
    args: MigrateArgs,
    config: RunnerConfig<'_>,
    db_config: DbClientConfig<'_>,
    db: &DbConnection,
) -> Result<(), Report> {
    let config = apply_command_args_to_runner_config(config, &args);
    let runner = runner(config);

    let migrated_to = if let Some(max_key_arg) = args.to {
        let max_key_arg = max_key_arg.trim();
        if max_key_arg.is_empty() {
            return Err(eyre!(
                "no key specified for option '--to'. please specify a key following the '--to' option in the format yyyymmdd_HHMMSS, e.g. --to 20250103_140520"
            ));
        }
        let max_key = NaiveDateTime::parse_from_str(max_key_arg, MIGRATION_KEY_FORMAT_STR)
            .map_err(|_| {
                eyre!("the argument in option '--to {max_key_arg}' is not a valid migration key. please specify the key in the format yyyymmdd_HHMMSS, e.g. --to 20250103_140520")
            })?;

        println!();
        log::info!(
            r#"Migrating database "{}/{}" up to {max_key_arg}..."#,
            &db_config.namespace,
            &db_config.database,
        );
        println!();

        runner.migrate_to(max_key, db).await.map_err(Report::from)
    } else {
        println!();
        log::info!(
            r#"Migrating database "{}/{}"..."#,
            &db_config.namespace,
            &db_config.database
        );
        println!();

        runner.migrate(db).await.map_err(Report::from)
    }?;

    match migrated_to {
        Migrated::Nothing => {
            log::info!(
                r#"No migration applied to database "{}/{}". All migrations are applied already."#,
                &db_config.namespace,
                &db_config.database
            );
        },
        Migrated::UpTo(last_applied) => {
            println!();
            log::info!(
                r#"Successfully migrated database "{}/{}" up to {}."#,
                &db_config.namespace,
                &db_config.database,
                &last_applied.format(MIGRATION_KEY_FORMAT_STR).to_string()
            );
        },
        Migrated::NoForwardMigrationsFound => {},
    }
    println!();
    Ok(())
}

const fn apply_command_args_to_runner_config<'a>(
    runner_config: RunnerConfig<'a>,
    args: &MigrateArgs,
) -> RunnerConfig<'a> {
    let runner_config = if args.ignore_checksum {
        runner_config.with_ignore_checksum(args.ignore_checksum)
    } else {
        runner_config
    };
    let runner_config = if args.ignore_order {
        runner_config.with_ignore_order(args.ignore_order)
    } else {
        runner_config
    };
    runner_config
}
