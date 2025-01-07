use crate::error::DefinitionError;
use crate::migration::{Migration, MigrationKind};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use std::ffi::OsString;
use std::path::{Path, PathBuf};

pub trait ParseMigration {
    type Err;

    fn parse_migration(&self) -> Result<Migration, Self::Err>;
}

pub const SCRIPT_FILE_EXTENSION: &str = ".surql";
pub const UP_SCRIPT_FILE_EXTENSION: &str = ".up.surql";
pub const DOWN_SCRIPT_FILE_EXTENSION: &str = ".down.surql";

fn parse_migration(path: &Path, filename: &str) -> Result<Migration, DefinitionError> {
    if !filename.ends_with(SCRIPT_FILE_EXTENSION) {
        return Err(DefinitionError::NoFilename);
    }
    let up = filename.ends_with(UP_SCRIPT_FILE_EXTENSION);
    let down = filename.ends_with(DOWN_SCRIPT_FILE_EXTENSION);
    let (kind, ext_len) = match (up, down) {
        (false, false) => (MigrationKind::Up, SCRIPT_FILE_EXTENSION.len()),
        (true, false) => (MigrationKind::Up, UP_SCRIPT_FILE_EXTENSION.len()),
        (false, true) => (MigrationKind::Down, DOWN_SCRIPT_FILE_EXTENSION.len()),
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
    let date = NaiveDate::parse_from_str(date_substr, "%Y%m%d")
        .map_err(|err| DefinitionError::InvalidDate(err.to_string()))?;
    if len < 15 + ext_len || &filename[8..9] != "_" {
        return Err(DefinitionError::MissingTime);
    }
    let time_substr = &filename[9..15];
    let time = NaiveTime::parse_from_str(time_substr, "%H%M%S")
        .map_err(|err| DefinitionError::InvalidTime(err.to_string()))?;
    let key = NaiveDateTime::new(date, time);
    if len < 17 + ext_len || &filename[15..16] != "_" {
        return Err(DefinitionError::MissingTitle);
    }
    let title = &filename[16..len - ext_len];
    let mut script_path = PathBuf::from(path);
    script_path.push(filename);

    Ok(Migration {
        key,
        title: title.to_string(),
        kind,
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

impl ParseMigration for OsString {
    type Err = DefinitionError;

    fn parse_migration(&self) -> Result<Migration, Self::Err> {
        let path = "";
        let filename = self.to_str().ok_or(DefinitionError::InvalidUtf8Character)?;

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
