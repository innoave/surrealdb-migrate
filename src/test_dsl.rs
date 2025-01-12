use crate::config::MIGRATION_KEY_FORMAT_STR;
use crate::migration::{ApplicableMigration, Execution, Migration, ScriptContent};
use chrono::{DateTime, NaiveDateTime, Utc};
use indexmap::IndexMap;

pub fn key(value: &str) -> NaiveDateTime {
    NaiveDateTime::parse_from_str(value, MIGRATION_KEY_FORMAT_STR).expect("invalid migration key")
}

pub fn datetime(value: &str) -> DateTime<Utc> {
    DateTime::parse_from_rfc3339(value)
        .expect("invalid RFC 3339 datetime")
        .to_utc()
}

pub fn defined_migrations(
    values: impl IntoIterator<Item = Migration>,
) -> IndexMap<NaiveDateTime, Migration> {
    values
        .into_iter()
        .map(|m| (m.key, m))
        .collect::<IndexMap<_, _>>()
}

pub fn script_contents(
    values: impl IntoIterator<Item = ScriptContent>,
) -> IndexMap<NaiveDateTime, ScriptContent> {
    values
        .into_iter()
        .map(|m| (m.key, m))
        .collect::<IndexMap<_, _>>()
}

pub fn executed_migrations(
    values: impl IntoIterator<Item = Execution>,
) -> IndexMap<NaiveDateTime, Execution> {
    values
        .into_iter()
        .map(|m| (m.key, m))
        .collect::<IndexMap<_, _>>()
}

pub fn applicable_migrations(
    values: impl IntoIterator<Item = ApplicableMigration>,
) -> IndexMap<NaiveDateTime, ApplicableMigration> {
    values
        .into_iter()
        .map(|m| (m.key, m))
        .collect::<IndexMap<_, _>>()
}
