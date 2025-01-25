use crate::checksum::Checksum;
use crate::config::{DEFAULT_MIGRATIONS_FOLDER, MIGRATION_KEY_FORMAT_STR};
use crate::definition::{
    DOWN_SCRIPT_FILE_EXTENSION, SCRIPT_FILE_EXTENSION, UP_SCRIPT_FILE_EXTENSION,
};
use crate::migration::{Migration, MigrationKind};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use proptest::prelude::*;
use proptest::string::string_regex;
use std::path::PathBuf;

pub fn any_checksum() -> impl Strategy<Value = Checksum> {
    (0..=0x_FFFF_FFFF_u32).prop_map(Checksum)
}

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn days_in_month(year: i32, month: u32) -> u32 {
    let current_month =
        NaiveDate::from_ymd_opt(year, month, 1).expect("year or month out of range");
    let (next_year, next_month) = match month {
        12 => (year + 1, 1),
        _ => (year, month + 1),
    };
    let next_month = NaiveDate::from_ymd_opt(next_year, next_month, 1)
        .expect("next_year or next_month out of range");
    next_month.signed_duration_since(current_month).num_days() as u32
}

pub fn any_key() -> impl Strategy<Value = NaiveDateTime> {
    (1970..=9999, 1..=12_u32)
        .prop_flat_map(|(year, month)| {
            (
                Just(year),
                Just(month),
                1..=days_in_month(year, month),
                1..=23_u32,
                1..=59_u32,
                1..=59_u32,
            )
        })
        .prop_map(|(year, month, day, hour, minute, second)| {
            NaiveDateTime::new(
                NaiveDate::from_ymd_opt(year, month, day).expect("year, month or day out of range"),
                NaiveTime::from_hms_opt(hour, minute, second)
                    .expect("hour, minute or second out of range"),
            )
        })
}

pub fn any_title() -> impl Strategy<Value = String> {
    string_regex(r"[\w][\w\-_ ]{0,200}").expect("invalid regex for title")
}

pub fn any_migration_kind() -> impl Strategy<Value = MigrationKind> {
    prop_oneof![
        Just(MigrationKind::Up),
        Just(MigrationKind::Down),
        Just(MigrationKind::Baseline)
    ]
}

pub fn any_direction() -> impl Strategy<Value = MigrationKind> {
    prop_oneof![Just(MigrationKind::Up), Just(MigrationKind::Down),]
}

pub fn any_filename() -> impl Strategy<Value = String> {
    (any_key(), any_title(), any_direction(), any::<bool>()).prop_map(
        |(key, title, direction, include_direction)| {
            let mut filename = key.format(MIGRATION_KEY_FORMAT_STR).to_string();
            filename.push('_');
            filename.push_str(&title);
            match (include_direction, direction) {
                (true, MigrationKind::Down) => filename.push_str(DOWN_SCRIPT_FILE_EXTENSION),
                (true, _) => filename.push_str(UP_SCRIPT_FILE_EXTENSION),
                (false, _) => filename.push_str(SCRIPT_FILE_EXTENSION),
            }
            filename
        },
    )
}

pub fn any_script_path() -> impl Strategy<Value = PathBuf> {
    (
        string_regex(r"/?[\w][\w\-_ ]{1,50}((/[\w])?[\w\-_ ]{1,50}){1,3}")
            .expect("invalid regex for workdir path"),
        any_filename(),
        any::<bool>(),
    )
        .prop_map(|(workdir, filename, default_migrations_folder)| {
            let mut path = PathBuf::from(workdir);
            if default_migrations_folder {
                path.push(DEFAULT_MIGRATIONS_FOLDER);
            }
            path.push(filename);
            path
        })
}

pub fn any_script_content() -> impl Strategy<Value = String> {
    string_regex(r#"(([\w][\w\-_ \(\)\[\]\{\}#'"]){1,100};\n){1,12}"#)
        .expect("invalid regex for script content")
}

pub fn any_migration() -> impl Strategy<Value = Migration> {
    (any_key(), any_title(), any_direction(), any_script_path()).prop_map(
        |(key, title, kind, script_path)| Migration {
            key,
            title,
            kind,
            script_path,
        },
    )
}
