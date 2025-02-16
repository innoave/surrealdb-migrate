use crate::args::VerifyArgs;
use crate::runner::runner;
use color_eyre::Report;
use surrealdb_migrate::action::{Check, Checks};
use surrealdb_migrate::config::{DbClientConfig, RunnerConfig, MIGRATION_KEY_FORMAT_STR};
use surrealdb_migrate::db_client::DbConnection;
use surrealdb_migrate::migration::{Problem, ProblematicMigration};
use surrealdb_migrate::result::Verified;

pub async fn run(
    args: VerifyArgs,
    config: RunnerConfig<'_>,
    db_config: DbClientConfig<'_>,
    db: &DbConnection,
) -> Result<(), Report> {
    let runner = runner(config);

    println!();
    println!(
        r#"Verifying migrations against database "{}/{}"..."#,
        &db_config.namespace, &db_config.database
    );
    println!();

    let checks = if args.checksum {
        if args.order {
            Check::Checksum + Check::Order
        } else {
            Check::Checksum.into()
        }
    } else if args.order {
        Check::Order.into()
    } else {
        Checks::all()
    };
    let verified = runner.verify_checks(checks, db).await?;

    match verified {
        Verified::NoProblemsFound => {
            println!("All migrations verified successfully.");
        },
        Verified::FoundProblems(mut problematic_migrations) => {
            problematic_migrations.sort_unstable_by_key(|pm| pm.key);
            problematic_migrations
                .iter()
                .for_each(print_problematic_migration);

            let num_problems = problematic_migrations.len();
            if num_problems == 1 {
                println!("\nFound 1 problematic migration.");
            } else {
                println!("\nFound {num_problems} problematic migrations.");
            }
        },
        Verified::NoMigrationsFound => {
            println!("No migrations defined in migrations folder.");
        },
    }
    println!();

    Ok(())
}

fn print_problematic_migration(pm: &ProblematicMigration) {
    match pm.problem {
        Problem::ChecksumMismatch {
            definition_checksum,
            execution_checksum,
        } => {
            let pm_key = pm.key.format(MIGRATION_KEY_FORMAT_STR).to_string();
            println!(
                "* migration {pm_key} has changed - current checksum: {definition_checksum}, applied checksum: {execution_checksum}",
            );
        },
        Problem::OutOfOrder {
            last_applied_key, ..
        } => {
            let pm_key = pm.key.format(MIGRATION_KEY_FORMAT_STR).to_string();
            let last_applied_key = last_applied_key
                .format(MIGRATION_KEY_FORMAT_STR)
                .to_string();
            println!(
                "* migration {pm_key} is out of order - last applied migration is {last_applied_key}",
            );
        },
    }
}
