use crate::migration::{ApplicableMigration, Execution, ScriptContent};
use chrono::NaiveDateTime;
use indexmap::IndexMap;

pub trait MigrationsToApply {
    fn list_migrations_to_apply(
        &self,
        defined_migrations: &IndexMap<NaiveDateTime, ScriptContent>,
        executed_migrations: &IndexMap<NaiveDateTime, Execution>,
    ) -> IndexMap<NaiveDateTime, ApplicableMigration>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Migrate;

impl MigrationsToApply for Migrate {
    fn list_migrations_to_apply(
        &self,
        defined_migrations: &IndexMap<NaiveDateTime, ScriptContent>,
        executed_migrations: &IndexMap<NaiveDateTime, Execution>,
    ) -> IndexMap<NaiveDateTime, ApplicableMigration> {
        defined_migrations
            .iter()
            .filter(|(key, mig)| mig.kind.is_forward() && !executed_migrations.contains_key(*key))
            .map(to_applicable_migration)
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Revert;

impl MigrationsToApply for Revert {
    fn list_migrations_to_apply(
        &self,
        defined_migrations: &IndexMap<NaiveDateTime, ScriptContent>,
        executed_migrations: &IndexMap<NaiveDateTime, Execution>,
    ) -> IndexMap<NaiveDateTime, ApplicableMigration> {
        defined_migrations
            .iter()
            .filter(|(key, mig)| mig.kind.is_backward() && executed_migrations.contains_key(*key))
            .map(to_applicable_migration)
            .collect()
    }
}

fn to_applicable_migration(
    (key, mig): (&NaiveDateTime, &ScriptContent),
) -> (NaiveDateTime, ApplicableMigration) {
    (
        *key,
        ApplicableMigration {
            key: *key,
            kind: mig.kind,
            script_content: mig.content.clone(),
            checksum: mig.checksum,
        },
    )
}

#[cfg(test)]
mod tests;
