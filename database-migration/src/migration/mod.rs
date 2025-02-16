use crate::checksum::Checksum;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
use std::time::Duration;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum MigrationKind {
    Baseline,
    Up,
    Down,
}

impl MigrationKind {
    pub const fn as_str(&self) -> &'static str {
        match self {
            Self::Baseline => "baseline",
            Self::Up => "up",
            Self::Down => "down",
        }
    }

    pub fn is_backward(&self) -> bool {
        *self == Self::Down
    }

    pub fn is_forward(&self) -> bool {
        !self.is_backward()
    }

    pub const fn is_any(&self) -> bool {
        true
    }
}

impl Display for MigrationKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewMigration {
    pub key: NaiveDateTime,
    pub title: String,
    pub kind: MigrationKind,
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
pub struct Reversion {
    pub key: NaiveDateTime,
    pub reverted_by: String,
    pub reverted_at: DateTime<Utc>,
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

#[cfg(test)]
mod tests;
