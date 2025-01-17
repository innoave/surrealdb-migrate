use crate::config::RunnerConfig;
use crate::db::{
    apply_migration_in_transaction, insert_migration_execution,
    select_all_executions_sorted_by_key, DbConnection,
};
use crate::error::Error;
use crate::fs::{read_script_content_for_migrations, ListMigrations, MigrationDirectory};
use crate::logic::{ListChangedAfterExecution, ListOutOfOrder, Migrate, MigrationsToApply, Verify};
use indexmap::IndexMap;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct MigrationRunner {
    migrations_folder: PathBuf,
    migrations_table: String,
    ignore_checksums: bool,
    ignore_order: bool,
}

impl Default for MigrationRunner {
    fn default() -> Self {
        Self::new(RunnerConfig::default())
    }
}

impl MigrationRunner {
    // pass `RunnerConfig` by value as we may add non-copy type parameters to
    // the config later and the API should not break without needing to clone
    // those parameters.
    #[allow(clippy::needless_pass_by_value)]
    pub fn new(config: RunnerConfig<'_>) -> Self {
        Self {
            migrations_folder: config.migrations_folder.into(),
            migrations_table: config.migrations_table.into(),
            ignore_checksums: config.ignore_checksums,
            ignore_order: config.ignore_order,
        }
    }

    pub async fn migrate(&self, db: &DbConnection) -> Result<(), Error> {
        let mig_dir = MigrationDirectory::new(self.migrations_folder.as_path());
        let mut migrations = mig_dir
            .list_all_migrations()?
            .filter(|maybe_mig| maybe_mig.as_ref().map_or(true, |mig| mig.kind.is_forward()))
            .collect::<Result<Vec<_>, _>>()?;
        migrations.sort_unstable_by_key(|mig| mig.key);
        let migrations = migrations;

        let script_contents = read_script_content_for_migrations(&migrations)?;
        let existing_executions =
            select_all_executions_sorted_by_key(&self.migrations_table, db).await?;
        let executed_migrations = existing_executions
            .into_iter()
            .map(|exec| (exec.key, exec))
            .collect::<IndexMap<_, _>>();
        let mut migrations = migrations
            .into_iter()
            .map(|mig| (mig.key, mig))
            .collect::<HashMap<_, _>>();

        let verify = Verify::default()
            .with_ignore_checksums(self.ignore_checksums)
            .with_ignore_order(self.ignore_order);
        let changed_after_execution =
            verify.list_changed_after_execution(&script_contents, &executed_migrations);
        if !changed_after_execution.is_empty() {
            return Err(Error::ChangedAfterExecution(changed_after_execution));
        }
        let out_of_order = verify.list_out_of_order(&script_contents, &executed_migrations);
        if !out_of_order.is_empty() {
            return Err(Error::OutOfOrder(out_of_order));
        }

        let migrate = Migrate::default();
        let to_apply = migrate.list_migrations_to_apply(&script_contents, &executed_migrations);
        for migration in to_apply.values() {
            let definition = migrations
                .remove(&migration.key)
                .expect("migration to applied not found in migrations - should be unreachable");
            let execution = apply_migration_in_transaction(
                migration,
                db.username(),
                &self.migrations_table,
                db,
            )
            .await?;
            insert_migration_execution(definition, execution, &self.migrations_table, db).await?;
        }
        Ok(())
    }
}
