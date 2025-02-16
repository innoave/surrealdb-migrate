use crate::migration::{
    ApplicableMigration, Execution, Problem, ProblematicMigration, ScriptContent,
};
use chrono::NaiveDateTime;
use enumset::{EnumSet, EnumSetIter, EnumSetType};
use indexmap::IndexMap;
use std::marker::PhantomData;
use std::ops::{Add, AddAssign};

pub trait ListOutOfOrder {
    fn list_out_of_order(
        &self,
        defined_migrations: &[ScriptContent],
        executed_migrations: &IndexMap<NaiveDateTime, Execution>,
    ) -> Vec<ProblematicMigration>;
}

pub trait ListChangedAfterExecution {
    fn list_changed_after_execution(
        &self,
        defined_migrations: &[ScriptContent],
        executed_migrations: &IndexMap<NaiveDateTime, Execution>,
    ) -> Vec<ProblematicMigration>;
}

#[derive(EnumSetType, Debug)]
pub enum Check {
    Checksum,
    Order,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Checks(EnumSet<Check>);

impl Checks {
    pub const fn none() -> Self {
        Self(EnumSet::empty())
    }

    pub const fn all() -> Self {
        Self(EnumSet::all())
    }

    pub fn only(check: Check) -> Self {
        Self(EnumSet::only(check))
    }

    pub fn contains(&self, check: Check) -> bool {
        self.0.contains(check)
    }

    pub fn iter(&self) -> CheckIter {
        CheckIter {
            set_iter: self.0.iter(),
        }
    }
}

impl From<Check> for Checks {
    fn from(value: Check) -> Self {
        Self(EnumSet::from(value))
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Add<Self> for Check {
    type Output = Checks;

    fn add(self, rhs: Self) -> Self::Output {
        Checks(self | rhs)
    }
}

#[allow(clippy::suspicious_op_assign_impl)]
impl AddAssign<Check> for Checks {
    fn add_assign(&mut self, rhs: Check) {
        self.0 |= rhs;
    }
}

#[derive(Clone)]
pub struct CheckIter {
    set_iter: EnumSetIter<Check>,
}

impl Iterator for CheckIter {
    type Item = Check;

    fn next(&mut self) -> Option<Self::Item> {
        self.set_iter.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.set_iter.size_hint()
    }
}

impl IntoIterator for &Checks {
    type Item = Check;
    type IntoIter = CheckIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl IntoIterator for Checks {
    type Item = Check;
    type IntoIter = CheckIter;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
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

impl From<Checks> for Verify {
    fn from(checks: Checks) -> Self {
        Self {
            ignore_checksums: !checks.contains(Check::Checksum),
            ignore_order: !checks.contains(Check::Order),
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

impl ListOutOfOrder for Verify {
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

impl ListChangedAfterExecution for Verify {
    fn list_changed_after_execution(
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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Migrate {
    _seal: PhantomData<()>,
}

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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Revert {
    _seal: PhantomData<()>,
}

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
