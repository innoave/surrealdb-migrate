use crate::args::RevertArgs;
use crate::runner::runner;
use chrono::NaiveDateTime;
use color_eyre::eyre::eyre;
use color_eyre::Report;
use surrealdb_migrate::config::{DbClientConfig, RunnerConfig, MIGRATION_KEY_FORMAT_STR};
use surrealdb_migrate::db_client::DbConnection;
use surrealdb_migrate::result::Reverted;

pub async fn run(
    args: RevertArgs,
    config: RunnerConfig<'_>,
    db_config: DbClientConfig<'_>,
    db: &DbConnection,
) -> Result<(), Report> {
    let runner = runner(config);

    let reverted_to = if let Some(max_remaining_key) = args.to {
        let max_key_arg = max_remaining_key.trim();
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
            r#"Reverting database "{}/{}" down to {max_key_arg}..."#,
            &db_config.namespace,
            &db_config.database,
        );
        println!();

        runner.revert_to(max_key, db).await?
    } else {
        println!();
        log::info!(
            r#"Reverting database "{}/{}"..."#,
            &db_config.namespace,
            &db_config.database,
        );
        println!();

        runner.revert(db).await?
    };

    match reverted_to {
        Reverted::Nothing => {
            log::info!(
                r#"Nothing to revert in database "{}/{}". All migrations are reverted already."#,
                &db_config.namespace,
                &db_config.database
            );
        },
        Reverted::DownTo(max_remaining) => {
            println!();
            log::info!(
                r#"Successfully reverted database "{}/{}" down to {}."#,
                &db_config.namespace,
                &db_config.database,
                &max_remaining.format(MIGRATION_KEY_FORMAT_STR).to_string()
            );
        },
        Reverted::Completely => {
            println!();
            log::info!(
                r#"Successfully reverted database "{}/{}" completely."#,
                &db_config.namespace,
                &db_config.database,
            );
        },
        Reverted::NoBackwardMigrationsFound => {},
    }
    println!();
    Ok(())
}
