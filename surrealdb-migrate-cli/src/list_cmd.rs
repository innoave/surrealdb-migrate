use crate::args::ListArgs;
use crate::runner::runner;
use crate::tables::format_migration_table;
use color_eyre::Report;
use std::cmp::Ordering;
use surrealdb_migrate::config::RunnerConfig;
use surrealdb_migrate::db_client::DbConnection;
use surrealdb_migrate::migration::MigrationKind;

pub async fn run(
    args: ListArgs,
    config: RunnerConfig<'_>,
    db: &DbConnection,
) -> Result<(), Report> {
    let runner = runner(config);
    let kind_criteria = match (args.up, args.down) {
        (_, false) => MigrationKind::is_forward,
        (false, true) => MigrationKind::is_backward,
        (true, true) => MigrationKind::is_any,
    };
    let migrations = runner.list_defined_migrations(kind_criteria)?;
    if migrations.is_empty() {
        println!("\nList of migrations:");
        let migration_table = format_migration_table(vec![])?;
        println!("{migration_table}");
        println!("  No migrations defined.\n");
    } else {
        let executions = runner.fetch_applied_migrations_dictionary(db).await?;
        let mut entries = Vec::with_capacity(migrations.len());

        for migration in migrations {
            let execution = executions.get(&migration.key);
            let applied = migration.kind.is_forward() == execution.is_some();
            if args.open == args.applied || args.applied && applied || args.open && !applied {
                let execution = execution.cloned();
                entries.push((migration, execution));
            }
        }
        if args.down && !args.up {
            entries.sort_unstable_by(|(mig1, _), (mig2, _)| {
                mig2.key
                    .cmp(&mig1.key)
                    .then_with(|| display_ordering(mig1.kind, mig2.kind))
            });
        } else {
            entries.sort_unstable_by(|(mig1, _), (mig2, _)| {
                mig1.key
                    .cmp(&mig2.key)
                    .then_with(|| display_ordering(mig1.kind, mig2.kind))
            });
        }

        match (args.open, args.applied, args.up, args.down) {
            (false, false, false, true) | (true, true, false, true) => {
                println!("\nList of backward migrations:");
            },
            (false, true, false, true) => println!("\nList of applied backward migrations:"),
            (true, false, false, true) => println!("\nList of open backward migrations:"),
            (false, true, _, _) => println!("\nList of applied migrations:"),
            (true, false, _, _) => println!("\nList of open migrations:"),
            (false, false, _, _) | (true, true, _, _) => println!("\nList of migrations:"),
        }
        let no_migrations_listed = entries.is_empty();
        let migration_table = format_migration_table(entries)?;
        println!("{migration_table}");
        if no_migrations_listed {
            println!("  No migrations found for the specified options.\n");
        }
    }
    Ok(())
}

#[allow(clippy::enum_glob_use)]
const fn display_ordering(mig1: MigrationKind, mig2: MigrationKind) -> Ordering {
    use MigrationKind::*;
    match (mig1, mig2) {
        (Baseline, Baseline) | (Up, Up) | (Down, Down) => Ordering::Equal,
        (Baseline, _) | (_, Down) => Ordering::Less,
        (_, Baseline) | (Down, _) => Ordering::Greater,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use asserting::prelude::*;

    mod display_ordering {
        use super::*;

        #[test]
        fn migration_kind_baseline_is_less_than_up() {
            assert_that!(display_ordering(MigrationKind::Baseline, MigrationKind::Up))
                .is_equal_to(Ordering::Less);
        }

        #[test]
        fn migration_kind_baseline_is_less_than_down() {
            assert_that!(display_ordering(
                MigrationKind::Baseline,
                MigrationKind::Down
            ))
            .is_equal_to(Ordering::Less);
        }

        #[test]
        fn migration_kind_up_is_less_than_down() {
            assert_that!(display_ordering(MigrationKind::Up, MigrationKind::Down))
                .is_equal_to(Ordering::Less);
        }

        #[test]
        fn migration_kind_up_is_greater_than_baseline() {
            assert_that!(display_ordering(MigrationKind::Up, MigrationKind::Baseline))
                .is_equal_to(Ordering::Greater);
        }

        #[test]
        fn migration_kind_down_is_greater_than_baseline() {
            assert_that!(display_ordering(
                MigrationKind::Down,
                MigrationKind::Baseline
            ))
            .is_equal_to(Ordering::Greater);
        }

        #[test]
        fn migration_kind_down_is_greater_than_up() {
            assert_that!(display_ordering(MigrationKind::Down, MigrationKind::Up))
                .is_equal_to(Ordering::Greater);
        }

        #[test]
        fn migration_kind_baseline_is_equal_baseline() {
            assert_that!(display_ordering(
                MigrationKind::Baseline,
                MigrationKind::Baseline
            ))
            .is_equal_to(Ordering::Equal);
        }

        #[test]
        fn migration_kind_up_is_equal_up() {
            assert_that!(display_ordering(MigrationKind::Up, MigrationKind::Up))
                .is_equal_to(Ordering::Equal);
        }

        #[test]
        fn migration_kind_down_is_equal_down() {
            assert_that!(display_ordering(MigrationKind::Down, MigrationKind::Down))
                .is_equal_to(Ordering::Equal);
        }
    }
}
