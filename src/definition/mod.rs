use crate::migration::{Direction, Migration};
use crate::Error;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use time::macros::format_description;
use time::{Date, PrimitiveDateTime, Time};

pub trait ParseMigration {
    type Err;

    fn parse_migration(&self) -> Result<Migration, Self::Err>;
}

const SCRIPT_FILE_EXTENSION: &str = ".surql";
const UP_SCRIPT_FILE_EXTENSION: &str = ".up.surql";
const DOWN_SCRIPT_FILE_EXTENSION: &str = ".down.surql";

fn parse_migration(path: &Path, filename: &str) -> Result<Migration, Error> {
    if !filename.ends_with(SCRIPT_FILE_EXTENSION) {
        return Err(Error::NoFilename);
    }
    let up = filename.ends_with(UP_SCRIPT_FILE_EXTENSION);
    let down = filename.ends_with(DOWN_SCRIPT_FILE_EXTENSION);
    let (direction, ext_len) = match (up, down) {
        (false, false) => (Direction::Up, SCRIPT_FILE_EXTENSION.len()),
        (true, false) => (Direction::Up, UP_SCRIPT_FILE_EXTENSION.len()),
        (false, true) => (Direction::Down, DOWN_SCRIPT_FILE_EXTENSION.len()),
        (true, true) => return Err(Error::AmbiguousDirection),
    };
    if filename.contains(".up.") && filename.contains(".down.") {
        return Err(Error::AmbiguousDirection);
    }
    let len = filename.len();
    if len < 8 + ext_len {
        return Err(Error::MissingDate);
    }
    let date_substr = &filename[0..8];
    let date = Date::parse(date_substr, format_description!("[year][month][day]"))
        .map_err(Error::InvalidDate)?;
    if len < 15 + ext_len || &filename[8..9] != "_" {
        return Err(Error::MissingTime);
    }
    let time_substr = &filename[9..15];
    let time = Time::parse(time_substr, format_description!("[hour][minute][second]"))
        .map_err(Error::InvalidTime)?;
    let id = PrimitiveDateTime::new(date, time);
    if len < 17 + ext_len || &filename[15..16] != "_" {
        return Err(Error::MissingTitle);
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
    type Err = Error;

    fn parse_migration(&self) -> Result<Migration, Self::Err> {
        let (path, filename) = self
            .rfind('/')
            .map_or(("", self), |index| (&self[..index], &self[index + 1..]));

        parse_migration(Path::new(path), filename)
    }
}

impl ParseMigration for OsString {
    type Err = Error;

    fn parse_migration(&self) -> Result<Migration, Self::Err> {
        let path = "";
        let filename = self.to_str().ok_or(Error::InvalidUtf8Character)?;

        parse_migration(Path::new(path), filename)
    }
}

impl ParseMigration for Path {
    type Err = Error;

    fn parse_migration(&self) -> Result<Migration, Self::Err> {
        let path = self.parent().unwrap_or_else(|| Self::new(""));
        let filename = self.file_name().ok_or(Error::NoFilename)?;
        let filename = filename.to_str().ok_or(Error::InvalidUtf8Character)?;

        parse_migration(path, filename)
    }
}

#[cfg(test)]
mod tests;
