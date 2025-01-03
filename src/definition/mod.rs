use crate::migration::{Direction, Migration};
use std::path::{Path, PathBuf};
use time::error::Parse;
use time::macros::format_description;
use time::{Date, PrimitiveDateTime, Time};

pub trait ParseMigration {
    type Err;

    fn parse_migration(&self) -> Result<Migration, Self::Err>;
}

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum DefinitionError {
    #[error("direction is ambiguous")]
    AmbiguousDirection,
    #[error("invalid date: {0}")]
    InvalidDate(Parse),
    #[error("invalid time: {0}")]
    InvalidTime(Parse),
    #[error("definition contains an invalid utf-8 character")]
    InvalidUtf8Character,
    #[error("definition does not contain a date")]
    MissingDate,
    #[error("definition does not contain a time")]
    MissingTime,
    #[error("definition does not contain a title")]
    MissingTitle,
    #[error("definition does not specify a filename")]
    NoFilename,
}

const SCRIPT_FILE_EXTENSION: &str = ".surql";
const UP_SCRIPT_FILE_EXTENSION: &str = ".up.surql";
const DOWN_SCRIPT_FILE_EXTENSION: &str = ".down.surql";

fn parse_migration(path: &Path, filename: &str) -> Result<Migration, DefinitionError> {
    if !filename.ends_with(SCRIPT_FILE_EXTENSION) {
        return Err(DefinitionError::NoFilename);
    }
    let up = filename.ends_with(UP_SCRIPT_FILE_EXTENSION);
    let down = filename.ends_with(DOWN_SCRIPT_FILE_EXTENSION);
    let (direction, ext_len) = match (up, down) {
        (false, false) => (Direction::Up, SCRIPT_FILE_EXTENSION.len()),
        (true, false) => (Direction::Up, UP_SCRIPT_FILE_EXTENSION.len()),
        (false, true) => (Direction::Down, DOWN_SCRIPT_FILE_EXTENSION.len()),
        (true, true) => return Err(DefinitionError::AmbiguousDirection),
    };
    if filename.contains(".up.") && filename.contains(".down.") {
        return Err(DefinitionError::AmbiguousDirection);
    }
    let len = filename.len();
    if len < 8 + ext_len {
        return Err(DefinitionError::MissingDate);
    }
    let date_substr = &filename[0..8];
    let date = Date::parse(date_substr, format_description!("[year][month][day]"))
        .map_err(DefinitionError::InvalidDate)?;
    if len < 15 + ext_len || &filename[8..9] != "_" {
        return Err(DefinitionError::MissingTime);
    }
    let time_substr = &filename[9..15];
    let time = Time::parse(time_substr, format_description!("[hour][minute][second]"))
        .map_err(DefinitionError::InvalidTime)?;
    let id = PrimitiveDateTime::new(date, time);
    if len < 17 + ext_len || &filename[15..16] != "_" {
        return Err(DefinitionError::MissingTitle);
    }
    let title = &filename[16..len - ext_len];
    let mut script_path = PathBuf::from(path);
    script_path.push(filename);

    Ok(Migration {
        id,
        title: title.to_string(),
        direction,
        script_path,
    })
}

impl ParseMigration for str {
    type Err = DefinitionError;

    fn parse_migration(&self) -> Result<Migration, Self::Err> {
        let (path, filename) = self
            .rfind('/')
            .map_or(("", self), |index| (&self[..index], &self[index + 1..]));

        parse_migration(Path::new(path), filename)
    }
}

impl ParseMigration for Path {
    type Err = DefinitionError;

    fn parse_migration(&self) -> Result<Migration, Self::Err> {
        let path = self.parent().unwrap_or_else(|| Self::new(""));
        let filename = self.file_name().ok_or(DefinitionError::NoFilename)?;
        let filename = filename
            .to_str()
            .ok_or(DefinitionError::InvalidUtf8Character)?;

        parse_migration(path, filename)
    }
}

#[cfg(test)]
mod tests;
