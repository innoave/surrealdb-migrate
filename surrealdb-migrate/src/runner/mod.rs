use database_migration::config::RunnerConfig;
use database_migration::error::Error;
use database_migration::logic::{
    ListChangedAfterExecution, ListOutOfOrder, Migrate, MigrationsToApply, Verify,
};
use database_migration::repository::{ListMigrations, ReadScriptContent};
use database_migration_files::MigrationDirectory;
use indexmap::IndexMap;
use std::collections::HashMap;
use std::path::PathBuf;
#[cfg(feature = "config")]
use surrealdb_migrate_config::Settings;
use surrealdb_migrate_db_client::{
    apply_migration_in_transaction, insert_migration_execution,
    select_all_executions_sorted_by_key, DbConnection,
};

pub struct MigrationRunner {
    migrations_folder: PathBuf,
    migrations_table: String,
    ignore_checksums: bool,
    ignore_order: bool,
}

impl MigrationRunner {
    pub fn new(config: RunnerConfig<'_>) -> Self {
        Self {
            migrations_folder: config.migrations_folder.into(),
            migrations_table: config.migrations_table.into(),
            ignore_checksums: config.ignore_checksums,
            ignore_order: config.ignore_order,
        }
    }

    #[cfg(feature = "config")]
    pub fn from_settings(settings: &Settings) -> Self {
        Self::new(settings.runner_config())
    }

    pub async fn migrate(&self, db: &DbConnection) -> Result<(), Error> {
        let mig_dir = MigrationDirectory::new(self.migrations_folder.as_path());
        let mut migrations = mig_dir
            .list_all_migrations()?
            .filter(|maybe_mig| maybe_mig.as_ref().map_or(true, |mig| mig.kind.is_forward()))
            .collect::<Result<Vec<_>, _>>()?;
        migrations.sort_unstable_by_key(|mig| mig.key);
        let migrations = migrations;

        let script_contents = mig_dir.read_script_content_for_migrations(&migrations)?;
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
            let definition = migrations.remove(&migration.key).expect(
                "migration to be applied not found in migrations folder - should be unreachable",
            );
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
