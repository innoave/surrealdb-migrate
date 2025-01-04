use crate::checksum::Checksum;
use std::path::PathBuf;
use time::{Duration, OffsetDateTime, PrimitiveDateTime};

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    #[default]
    Up,
    Down,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Migration {
    pub id: PrimitiveDateTime,
    pub title: String,
    pub direction: Direction,
    pub script_path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Execution {
    pub id: PrimitiveDateTime,
    pub applied_at: OffsetDateTime,
    pub checksum: Checksum,
    pub execution_time: Duration,
    pub success: bool,
}
