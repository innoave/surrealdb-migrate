use crate::args::ListArgs;
use crate::runner::runner;
use crate::tables::format_migration_table;
use color_eyre::Report;
use std::cmp::Reverse;
use surrealdb_migrate::config::RunnerConfig;
use surrealdb_migrate::db_client::DbConnection;
use surrealdb_migrate::migration::MigrationKind;

pub async fn run(
    args: ListArgs,
    config: RunnerConfig<'_>,
    db: &DbConnection,
) -> Result<(), Report> {
    let mut no_records = None;
    let runner = runner(config);
    let kind_criteria = if args.down && !args.up {
        MigrationKind::is_backward
    } else {
        MigrationKind::is_forward
    };
    let migrations = runner.list_defined_migrations(kind_criteria)?;
    if migrations.is_empty() {
        no_records = Some("No migrations defined.");
    }
    let mut executions = runner.list_applied_migrations(db).await?;
    let mut entries = Vec::with_capacity(migrations.len());
    match (args.up, args.down, args.applied, args.open) {
        (true, _, _, _) | (_, false, false, false) | (_, false, true, true) => {
            for migration in migrations {
                let execution = executions
                    .iter()
                    .position(|exe| exe.key == migration.key)
                    .map(|pos| executions.remove(pos));
                entries.push((migration, execution));
            }
            println!("\nList of migrations:");
        },
        (_, false, true, false) => {
            for migration in migrations {
                if let Some(execution) = executions
                    .iter()
                    .position(|exe| exe.key == migration.key)
                    .map(|pos| executions.remove(pos))
                {
                    entries.push((migration, Some(execution)));
                }
            }
            println!("\nList of applied migrations:");
        },
        (_, false, false, true) => {
            for migration in migrations {
                let applied = executions.iter().any(|exe| exe.key == migration.key);
                if !applied {
                    entries.push((migration, None));
                }
            }
            println!("\nList of open migrations:");
        },
        (false, true, false, false) | (false, true, true, true) => {
            for migration in migrations {
                let execution = executions
                    .iter()
                    .position(|exe| exe.key == migration.key)
                    .map(|pos| executions.remove(pos));
                entries.push((migration, execution));
            }
            entries.sort_unstable_by_key(|(mig, _)| Reverse(mig.key));
            println!("\nList of backward migrations:");
        },
        (false, true, true, false) => {
            for migration in migrations {
                let applied = executions.iter().any(|exe| exe.key == migration.key);
                if !applied {
                    entries.push((migration, None));
                }
            }
            println!("\nList of applied backward migrations:");
        },
        (false, true, false, true) => {
            for migration in migrations.into_iter().filter(|mig| mig.kind.is_backward()) {
                if let Some(execution) = executions
                    .iter()
                    .position(|exe| exe.key == migration.key)
                    .map(|pos| executions.remove(pos))
                {
                    entries.push((migration, Some(execution)));
                }
            }
            entries.sort_unstable_by_key(|(mig, _)| Reverse(mig.key));
            println!("\nList of open backward migrations:");
        },
    }
    if no_records.is_none() && entries.is_empty() {
        no_records = Some("No migrations found for the specified options.");
    }
    let migration_table = format_migration_table(entries)?;
    println!("{migration_table}");
    if let Some(message) = no_records {
        println!("  {message}\n");
    }
    Ok(())
}
