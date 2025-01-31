use crate::args::ListArgs;
use crate::runner::runner;
use crate::tables::format_migration_table;
use color_eyre::Report;
use std::cmp::{Ordering, Reverse};
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
        let mut executions = runner.list_applied_migrations(db).await?;
        let mut entries = Vec::with_capacity(migrations.len());

        match (args.up, args.down, args.applied, args.open) {
            (_, false, false, false) | (_, false, true, true) => {
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
                entries.sort_unstable_by_key(|(mig, _)| Reverse(mig.key));
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
            (true, true, false, false) | (true, true, true, true) => {
                for migration in migrations {
                    let execution = if migration.kind.is_forward() {
                        executions
                            .iter()
                            .position(|exe| exe.key == migration.key)
                            .map(|pos| executions.remove(pos))
                    } else {
                        None
                    };
                    entries.push((migration, execution));
                }
                entries.sort_unstable_by(|(mig1, _), (mig2, _)| {
                    mig1.key
                        .cmp(&mig2.key)
                        .then_with(|| display_ordering(mig1.kind, mig2.kind))
                });
                println!("\nList of migrations:");
            },
            (true, true, false, true) => {
                for migration in migrations {
                    let execution = executions.iter().find(|exe| exe.key == migration.key);
                    let applied = execution.is_some();
                    if migration.kind.is_backward() == applied {
                        let execution = execution.cloned();
                        entries.push((migration, execution));
                    }
                }
                entries.sort_unstable_by(|(mig1, _), (mig2, _)| {
                    mig1.key
                        .cmp(&mig2.key)
                        .then_with(|| display_ordering(mig1.kind, mig2.kind))
                });
                println!("\nList of open migrations:");
            },
            (true, true, true, false) => {
                for migration in migrations {
                    let execution = executions.iter().find(|exe| exe.key == migration.key);
                    let applied = execution.is_some();
                    if migration.kind.is_forward() == applied {
                        let execution = execution.cloned();
                        entries.push((migration, execution));
                    }
                }
                entries.sort_unstable_by(|(mig1, _), (mig2, _)| {
                    mig1.key
                        .cmp(&mig2.key)
                        .then_with(|| display_ordering(mig1.kind, mig2.kind))
                });
                println!("\nList of applied migrations:");
            },
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
    use assertor::*;

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
