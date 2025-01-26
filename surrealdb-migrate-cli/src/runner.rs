use surrealdb_migrate::config::RunnerConfig;
use surrealdb_migrate::runner::MigrationRunner;

pub fn runner(config: RunnerConfig<'_>) -> MigrationRunner {
    MigrationRunner::new(config)
}
