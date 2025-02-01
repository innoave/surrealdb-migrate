use chrono::NaiveDateTime;
use database_migration::config::{RunnerConfig, MIGRATION_KEY_FORMAT_STR};
use database_migration::error::Error;
use database_migration::logic::{
    ListChangedAfterExecution, ListOutOfOrder, Migrate, MigrationsToApply, Verify,
};
use database_migration::migration::{Execution, Migration, MigrationKind};
use database_migration::repository::{ListMigrations, ReadScriptContent};
use database_migration_files::MigrationDirectory;
use indexmap::IndexMap;
use std::collections::HashMap;
use std::path::PathBuf;
#[cfg(feature = "config")]
use surrealdb_migrate_config::Settings;
use surrealdb_migrate_db_client::{
    apply_migration_in_transaction, insert_migration_execution, select_all_executions,
    select_all_executions_sorted_by_key, DbConnection,
};

pub struct MigrationRunner {
    migrations_folder: PathBuf,
    migrations_table: String,
    ignore_checksum: bool,
    ignore_order: bool,
}

impl MigrationRunner {
    pub fn new(config: RunnerConfig<'_>) -> Self {
        Self {
            migrations_folder: config.migrations_folder.into(),
            migrations_table: config.migrations_table.into(),
            ignore_checksum: config.ignore_checksum,
            ignore_order: config.ignore_order,
        }
    }

    #[cfg(feature = "config")]
    pub fn with_settings(settings: &Settings) -> Self {
        Self::new(settings.runner_config())
    }

    pub fn list_defined_migrations<P>(&self, predicate: P) -> Result<Vec<Migration>, Error>
    where
        P: Fn(&MigrationKind) -> bool,
    {
        let mut migrations = MigrationDirectory::new(self.migrations_folder.as_path())
            .list_all_migrations()?
            .filter(|maybe_mig| maybe_mig.as_ref().map_or(true, |mig| predicate(&mig.kind)))
            .collect::<Result<Vec<_>, _>>()?;
        migrations.sort_unstable_by_key(|mig| mig.key);
        Ok(migrations)
    }

    pub async fn list_applied_migrations(
        &self,
        db: &DbConnection,
    ) -> Result<Vec<Execution>, Error> {
        select_all_executions_sorted_by_key(&self.migrations_table, db).await
    }

    pub async fn fetch_applied_migrations_dictionary(
        &self,
        db: &DbConnection,
    ) -> Result<HashMap<NaiveDateTime, Execution>, Error> {
        select_all_executions(&self.migrations_table, db).await
    }

    pub async fn migrate(&self, db: &DbConnection) -> Result<Option<NaiveDateTime>, Error> {
        let mig_dir = MigrationDirectory::new(self.migrations_folder.as_path());
        let mut migrations = mig_dir
            .list_all_migrations()?
            .filter(|maybe_mig| maybe_mig.as_ref().map_or(true, |mig| mig.kind.is_forward()))
            .collect::<Result<Vec<_>, _>>()?;
        migrations.sort_unstable_by_key(|mig| mig.key);

        self.migrate_list(mig_dir, migrations, db).await
    }

    pub async fn migrate_to(
        &self,
        max_key: NaiveDateTime,
        db: &DbConnection,
    ) -> Result<Option<NaiveDateTime>, Error> {
        let mig_dir = MigrationDirectory::new(self.migrations_folder.as_path());
        let mut migrations = mig_dir
            .list_all_migrations()?
            .filter(|maybe_mig| {
                maybe_mig
                    .as_ref()
                    .map_or(true, |mig| mig.kind.is_forward() && mig.key <= max_key)
            })
            .collect::<Result<Vec<_>, _>>()?;
        migrations.sort_unstable_by_key(|mig| mig.key);

        self.migrate_list(mig_dir, migrations, db).await
    }

    async fn migrate_list(
        &self,
        mig_dir: MigrationDirectory<'_>,
        migration_list: Vec<Migration>,
        db: &DbConnection,
    ) -> Result<Option<NaiveDateTime>, Error> {
        let script_contents = mig_dir.read_script_content_for_migrations(&migration_list)?;
        let existing_executions =
            select_all_executions_sorted_by_key(&self.migrations_table, db).await?;
        let executed_migrations = existing_executions
            .into_iter()
            .map(|exec| (exec.key, exec))
            .collect::<IndexMap<_, _>>();
        let mut migrations = migration_list
            .into_iter()
            .map(|mig| (mig.key, mig))
            .collect::<HashMap<_, _>>();

        let verify = Verify::default()
            .with_ignore_checksums(self.ignore_checksum)
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

        let mut last_applied_migration = None;
        let migrate = Migrate::default();
        let to_apply = migrate.list_migrations_to_apply(&script_contents, &executed_migrations);
        for migration in to_apply.values() {
            let definition = migrations.remove(&migration.key).expect(
                "migration to be applied not found in migrations folder - should be unreachable - please report a bug",
            );
            let migration_applied = format!(
                "{}: {} ({}) applied",
                migration.key.format(MIGRATION_KEY_FORMAT_STR),
                &definition.title,
                &migration.kind.as_str(),
            );
            let execution = apply_migration_in_transaction(
                migration,
                db.username(),
                &self.migrations_table,
                db,
            )
            .await?;
            insert_migration_execution(definition, execution, &self.migrations_table, db).await?;
            last_applied_migration = Some(migration.key);
            log::info!("{migration_applied}");
        }
        Ok(last_applied_migration)
    }
}

#[cfg(test)]
mod tests;
