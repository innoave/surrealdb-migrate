use chrono::NaiveDateTime;
use database_migration::action::{
    Checks, ListChangedAfterExecution, ListOutOfOrder, Migrate, MigrationsToApply, Revert, Verify,
};
use database_migration::config::{MIGRATION_KEY_FORMAT_STR, RunnerConfig};
use database_migration::definition::ExcludedFiles;
use database_migration::error::Error;
use database_migration::migration::{Execution, Migration, MigrationKind};
use database_migration::repository::{ListMigrations, ReadScriptContent};
use database_migration::result::{Migrated, Reverted, Verified};
use database_migration_files::MigrationDirectory;
use indexmap::IndexMap;
use std::cmp::Reverse;
use std::collections::HashMap;
use std::path::PathBuf;
#[cfg(feature = "config")]
use surrealdb_migrate_config::Settings;
use surrealdb_migrate_db_client::{
    DbConnection, apply_migration_in_transaction, delete_migration_execution,
    find_max_applied_migration_key, insert_migration_execution, revert_migration_in_transaction,
    select_all_executions, select_all_executions_sorted_by_key,
};

pub struct MigrationRunner {
    migrations_folder: PathBuf,
    excluded_files: ExcludedFiles,
    migrations_table: String,
    ignore_checksum: bool,
    ignore_order: bool,
}

impl MigrationRunner {
    pub fn new(config: RunnerConfig<'_>) -> Self {
        Self {
            migrations_folder: config.migrations_folder.into(),
            excluded_files: config.excluded_files,
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
        let mut migrations =
            MigrationDirectory::new(self.migrations_folder.as_path(), &self.excluded_files)
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

    pub async fn migrate(&self, db: &DbConnection) -> Result<Migrated, Error> {
        let mig_dir =
            MigrationDirectory::new(self.migrations_folder.as_path(), &self.excluded_files);
        let mut migrations = mig_dir
            .list_all_migrations()?
            .filter(|maybe_mig| maybe_mig.as_ref().map_or(true, |mig| mig.kind.is_forward()))
            .collect::<Result<Vec<_>, _>>()?;
        if migrations.is_empty() {
            return Ok(Migrated::NoForwardMigrationsFound);
        }
        migrations.sort_unstable_by_key(|mig| mig.key);

        self.migrate_list(mig_dir, migrations, db).await
    }

    pub async fn migrate_to(
        &self,
        max_key: NaiveDateTime,
        db: &DbConnection,
    ) -> Result<Migrated, Error> {
        let mig_dir =
            MigrationDirectory::new(self.migrations_folder.as_path(), &self.excluded_files);
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
    ) -> Result<Migrated, Error> {
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

        let migrate = Migrate::default();
        let to_apply = migrate.list_migrations_to_apply(&script_contents, &executed_migrations);

        let mut last_applied_migration = None;
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

        Ok(last_applied_migration.map_or(Migrated::Nothing, Migrated::UpTo))
    }

    pub async fn revert(&self, db: &DbConnection) -> Result<Reverted, Error> {
        let mig_dir =
            MigrationDirectory::new(self.migrations_folder.as_path(), &self.excluded_files);
        let mut migrations = mig_dir
            .list_all_migrations()?
            .filter(|maybe_mig| {
                maybe_mig
                    .as_ref()
                    .map_or(true, |mig| mig.kind.is_backward())
            })
            .collect::<Result<Vec<_>, _>>()?;
        if migrations.is_empty() {
            return Ok(Reverted::NoBackwardMigrationsFound);
        }
        migrations.sort_unstable_by_key(|mig| Reverse(mig.key));

        self.revert_list(mig_dir, migrations, db).await
    }

    pub async fn revert_to(
        &self,
        max_key: NaiveDateTime,
        db: &DbConnection,
    ) -> Result<Reverted, Error> {
        let mig_dir =
            MigrationDirectory::new(self.migrations_folder.as_path(), &self.excluded_files);
        let mut migrations = mig_dir
            .list_all_migrations()?
            .filter(|maybe_mig| {
                maybe_mig
                    .as_ref()
                    .map_or(true, |mig| mig.kind.is_backward() && mig.key > max_key)
            })
            .collect::<Result<Vec<_>, _>>()?;
        migrations.sort_unstable_by_key(|mig| Reverse(mig.key));

        self.revert_list(mig_dir, migrations, db).await
    }

    async fn revert_list(
        &self,
        mig_dir: MigrationDirectory<'_>,
        migration_list: Vec<Migration>,
        db: &DbConnection,
    ) -> Result<Reverted, Error> {
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

        let revert = Revert::default();
        let to_apply = revert.list_migrations_to_apply(&script_contents, &executed_migrations);

        for migration in to_apply.values() {
            let definition = migrations.remove(&migration.key).expect(
                "down migration to be applied not found in migrations folder - should be unreachable - please report a bug",
            );
            let migration_reverted = format!(
                "{}: {} ({}) applied",
                migration.key.format(MIGRATION_KEY_FORMAT_STR),
                &definition.title,
                &migration.kind.as_str(),
            );
            let reversion = revert_migration_in_transaction(migration, db.username(), db).await?;
            delete_migration_execution(reversion, &self.migrations_table, db).await?;
            log::info!("{migration_reverted}");
        }
        let max_remaining_migration =
            find_max_applied_migration_key(&self.migrations_table, db).await?;

        let completely_or_nothing = || {
            if executed_migrations.is_empty() {
                Reverted::Nothing
            } else {
                Reverted::Completely
            }
        };

        Ok(max_remaining_migration.map_or_else(completely_or_nothing, Reverted::DownTo))
    }

    pub async fn verify(&self, db: &DbConnection) -> Result<Verified, Error> {
        self.verify_checks(Checks::all(), db).await
    }

    pub async fn verify_checks(
        &self,
        checks: Checks,
        db: &DbConnection,
    ) -> Result<Verified, Error> {
        let mig_dir =
            MigrationDirectory::new(self.migrations_folder.as_path(), &self.excluded_files);
        let mut migrations = mig_dir
            .list_all_migrations()?
            .filter(|maybe_mig| maybe_mig.as_ref().map_or(true, |mig| mig.kind.is_forward()))
            .collect::<Result<Vec<_>, _>>()?;
        if migrations.is_empty() {
            return Ok(Verified::NoMigrationsFound);
        }
        migrations.sort_unstable_by_key(|mig| mig.key);
        let script_contents = mig_dir.read_script_content_for_migrations(&migrations)?;

        let existing_executions =
            select_all_executions_sorted_by_key(&self.migrations_table, db).await?;
        let executed_migrations = existing_executions
            .into_iter()
            .map(|exec| (exec.key, exec))
            .collect::<IndexMap<_, _>>();

        let verify = Verify::from(checks);
        let out_of_order_migrations =
            verify.list_out_of_order(&script_contents, &executed_migrations);
        let changed_migrations =
            verify.list_changed_after_execution(&script_contents, &executed_migrations);

        let mut problematic_migrations = out_of_order_migrations;
        problematic_migrations.extend(changed_migrations);

        if problematic_migrations.is_empty() {
            Ok(Verified::NoProblemsFound)
        } else {
            Ok(Verified::FoundProblems(problematic_migrations))
        }
    }
}

#[cfg(test)]
mod tests;
