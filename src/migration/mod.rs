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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Migration {
    pub key: NaiveDateTime,
    pub title: String,
    pub kind: MigrationKind,
    pub script_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Execution {
    pub key: NaiveDateTime,
    pub applied_rank: i64,
    pub applied_by: String,
    pub applied_at: DateTime<Utc>,
    pub checksum: Checksum,
    pub execution_time: Duration,
    pub successful: bool,
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
