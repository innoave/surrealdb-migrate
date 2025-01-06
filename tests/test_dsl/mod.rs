use chrono::{DateTime, NaiveDateTime, Utc};
use surrealdb_migrate::config::MIGRATION_KEY_FORMAT_STR;

pub fn key(value: &str) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(value, MIGRATION_KEY_FORMAT_STR).expect("invalid migration key")
}

pub fn datetime(value: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(value)
        .expect("invalid RFC 3339 datetime")
        .to_utc()
}
