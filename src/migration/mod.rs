use crate::checksum::Checksum;
use chrono::{DateTime, NaiveDateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

#[derive(Default, Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    #[default]
    Up,
    Down,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Migration {
    pub key: NaiveDateTime,
    pub title: String,
    pub direction: Direction,
    pub script_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Execution {
    pub key: NaiveDateTime,
    pub applied_at: DateTime<Utc>,
    pub checksum: Checksum,
    pub execution_time: Duration,
    pub success: bool,
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
