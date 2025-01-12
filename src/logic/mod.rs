use crate::migration::{
    ApplicableMigration, Execution, Problem, ProblematicMigration, ScriptContent,
};
use chrono::NaiveDateTime;
use indexmap::IndexMap;

pub trait OutOfOrder {
    fn list_out_of_order(
        &self,
        defined_migrations: &[ScriptContent],
        executed_migrations: &IndexMap<NaiveDateTime, Execution>,
    ) -> Vec<ProblematicMigration>;
}

pub trait ChangedMigrations {
    fn list_changed_migrations(
        &self,
        defined_migrations: &[ScriptContent],
        executed_migrations: &IndexMap<NaiveDateTime, Execution>,
    ) -> Vec<ProblematicMigration>;
}

#[must_use]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Verify {
    ignore_checksums: bool,
    ignore_order: bool,
}

#[allow(clippy::derivable_impls)]
impl Default for Verify {
    fn default() -> Self {
        Self {
            ignore_checksums: false,
            ignore_order: false,
        }
    }
}

impl Verify {
    pub const fn with_ignore_checksums(mut self, ignore_checksums: bool) -> Self {
        self.ignore_checksums = ignore_checksums;
        self
    }

    pub const fn ignore_checksums(&self) -> bool {
        self.ignore_checksums
    }

    pub const fn with_ignore_order(mut self, ignore_order: bool) -> Self {
        self.ignore_order = ignore_order;
        self
    }

    pub const fn ignore_order(&self) -> bool {
        self.ignore_order
    }
}

impl OutOfOrder for Verify {
    fn list_out_of_order(
        &self,
        defined_migrations: &[ScriptContent],
        executed_migrations: &IndexMap<NaiveDateTime, Execution>,
    ) -> Vec<ProblematicMigration> {
        if self.ignore_order {
            return Vec::new();
        }
        if let Some(&last_applied_key) = executed_migrations.keys().max_by_key(|key| **key) {
            defined_migrations
                .iter()
                .filter_map(|mig| {
                    if last_applied_key > mig.key && !executed_migrations.contains_key(&mig.key) {
                        Some(ProblematicMigration {
                            key: mig.key,
                            kind: mig.kind,
                            script_path: mig.path.clone(),
                            problem: Problem::OutOfOrder {
                                definition_key: mig.key,
                                last_applied_key,
                            },
                        })
                    } else {
                        None
                    }
                })
                .collect()
        } else {
            Vec::new()
        }
    }
}

impl ChangedMigrations for Verify {
    fn list_changed_migrations(
        &self,
        defined_migrations: &[ScriptContent],
        executed_migrations: &IndexMap<NaiveDateTime, Execution>,
    ) -> Vec<ProblematicMigration> {
        if self.ignore_checksums {
            return Vec::new();
        }
        defined_migrations
            .iter()
            .filter_map(|mig| {
                if mig.kind.is_forward() {
                    executed_migrations.get(&mig.key).and_then(|exec| {
                        if exec.checksum != mig.checksum {
                            Some(ProblematicMigration {
                                key: mig.key,
                                kind: mig.kind,
                                script_path: mig.path.clone(),
                                problem: Problem::ChecksumMismatch {
                                    definition_checksum: mig.checksum,
                                    execution_checksum: exec.checksum,
                                },
                            })
                        } else {
                            None
                        }
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}

pub trait MigrationsToApply {
    fn list_migrations_to_apply(
        &self,
        defined_migrations: &[ScriptContent],
        executed_migrations: &IndexMap<NaiveDateTime, Execution>,
    ) -> IndexMap<NaiveDateTime, ApplicableMigration>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Migrate;

impl MigrationsToApply for Migrate {
    fn list_migrations_to_apply(
        &self,
        defined_migrations: &[ScriptContent],
        executed_migrations: &IndexMap<NaiveDateTime, Execution>,
    ) -> IndexMap<NaiveDateTime, ApplicableMigration> {
        defined_migrations
            .iter()
            .filter(|mig| mig.kind.is_forward() && !executed_migrations.contains_key(&mig.key))
            .map(to_applicable_migration)
            .collect()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Revert;

impl MigrationsToApply for Revert {
    fn list_migrations_to_apply(
        &self,
        defined_migrations: &[ScriptContent],
        executed_migrations: &IndexMap<NaiveDateTime, Execution>,
    ) -> IndexMap<NaiveDateTime, ApplicableMigration> {
        defined_migrations
            .iter()
            .filter(|mig| mig.kind.is_backward() && executed_migrations.contains_key(&mig.key))
            .map(to_applicable_migration)
            .collect()
    }
}

fn to_applicable_migration(mig: &ScriptContent) -> (NaiveDateTime, ApplicableMigration) {
    (
        mig.key,
        ApplicableMigration {
            key: mig.key,
            kind: mig.kind,
            script_content: mig.content.clone(),
            checksum: mig.checksum,
        },
    )
}

#[cfg(test)]
mod tests;
