use crate::checksum::Checksum;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MigrationKind {
    Baseline,
    Up,
    Down,
}

impl MigrationKind {
    pub fn is_backward(&self) -> bool {
        *self == Self::Down
    }

    pub fn is_forward(&self) -> bool {
        !self.is_backward()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Migration {
    pub key: NaiveDateTime,
    pub title: String,
    pub kind: MigrationKind,
    pub script_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptContent {
    pub key: NaiveDateTime,
    pub kind: MigrationKind,
    pub path: PathBuf,
    pub content: String,
    pub checksum: Checksum,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApplicableMigration {
    pub key: NaiveDateTime,
    pub kind: MigrationKind,
    pub script_content: String,
    pub checksum: Checksum,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Execution {
    pub key: NaiveDateTime,
    pub applied_rank: i64,
    pub applied_by: String,
    pub applied_at: DateTime<Utc>,
    pub checksum: Checksum,
    pub execution_time: Duration,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProblematicMigration {
    pub key: NaiveDateTime,
    pub kind: MigrationKind,
    pub script_path: PathBuf,
    pub problem: Problem,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Problem {
    ChecksumMismatch {
        definition_checksum: Checksum,
        execution_checksum: Checksum,
    },
    OutOfOrder {
        definition_key: NaiveDateTime,
        last_applied_key: NaiveDateTime,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MigrationsTableInfo {
    NoTables,
    Missing,
    Table {
        name: String,
        version: Option<String>,
        definition: String,
    },
}
