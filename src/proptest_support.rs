use crate::definition::{
    DEFAULT_MIGRATIONS_FOLDER, DOWN_SCRIPT_FILE_EXTENSION, SCRIPT_FILE_EXTENSION,
    UP_SCRIPT_FILE_EXTENSION,
};
use crate::migration::{Direction, Migration};
use proptest::prelude::*;
use proptest::string::string_regex;
use std::path::PathBuf;
use time::macros::format_description;
use time::{Date, Month, PrimitiveDateTime, Time};

fn any_month() -> impl Strategy<Value = Month> {
    prop_oneof![
        Just(Month::January),
        Just(Month::February),
        Just(Month::March),
        Just(Month::April),
        Just(Month::May),
        Just(Month::June),
        Just(Month::July),
        Just(Month::August),
        Just(Month::September),
        Just(Month::October),
        Just(Month::November),
        Just(Month::December),
    ]
}

pub fn any_id() -> impl Strategy<Value = PrimitiveDateTime> {
    (1970..=9999, any_month())
        .prop_flat_map(|(year, month)| {
            (
                Just(year),
                Just(month),
                1..=time::util::days_in_month(month, year),
                1..=23_u8,
                1..=59_u8,
                1..=59_u8,
            )
        })
        .prop_map(|(year, month, day, hour, minute, second)| {
            PrimitiveDateTime::new(
                Date::from_calendar_date(year, month, day).expect("invalid calendar date"),
                Time::from_hms(hour, minute, second).expect("invalid time"),
            )
        })
}

pub fn any_title() -> impl Strategy<Value = String> {
    string_regex(r"[\w][\w\-]{0,200}").expect("invalid regex for title")
}

pub fn any_direction() -> impl Strategy<Value = Direction> {
    prop_oneof![Just(Direction::Up), Just(Direction::Down),]
}

pub fn any_filename() -> impl Strategy<Value = String> {
    (any_id(), any_title(), any_direction(), any::<bool>()).prop_map(
        |(id, title, direction, include_direction)| {
            let mut filename = id
                .format(format_description!(
                    "[year][month][day]_[hour][minute][second]"
                ))
                .expect("invalid filename");
            filename.push('_');
            filename.push_str(&title);
            match (include_direction, direction) {
                (true, Direction::Up) => filename.push_str(UP_SCRIPT_FILE_EXTENSION),
                (true, Direction::Down) => filename.push_str(DOWN_SCRIPT_FILE_EXTENSION),
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

pub fn any_migration() -> impl Strategy<Value = Migration> {
    (any_id(), any_title(), any_direction(), any_script_path()).prop_map(
        |(id, title, direction, script_path)| Migration {
            id,
            title,
            direction,
            script_path,
        },
    )
}
