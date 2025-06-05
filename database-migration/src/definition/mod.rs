use crate::config::{DEFAULT_EXCLUDED_FILES, MIGRATION_KEY_FORMAT_STR};
use crate::error::{DefinitionError, FilePatternError};
use crate::migration::{Migration, MigrationKind, NewMigration};
use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use regex::Regex;
use std::borrow::Cow;
use std::ffi::OsStr;
use std::fmt::{self, Display, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub trait ParseMigration {
    type Err;

    fn parse_migration(&self) -> Result<Migration, Self::Err>;
}

pub const SCRIPT_FILE_EXTENSION: &str = ".surql";
pub const UP_SCRIPT_FILE_EXTENSION: &str = ".up.surql";
pub const DOWN_SCRIPT_FILE_EXTENSION: &str = ".down.surql";

fn parse_migration(path: &Path, filename: &str) -> Result<Migration, DefinitionError> {
    if !filename.ends_with(SCRIPT_FILE_EXTENSION) {
        return Err(DefinitionError::InvalidFilename);
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
    let title = if len < 17 + ext_len || &filename[15..16] != "_" {
        ""
    } else {
        &filename[16..len - ext_len].replace('_', " ")
    };
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

impl ParseMigration for OsStr {
    type Err = DefinitionError;

    fn parse_migration(&self) -> Result<Migration, Self::Err> {
        let path_str = self.to_str().ok_or(DefinitionError::InvalidUtf8Character)?;
        ParseMigration::parse_migration(path_str)
    }
}

impl ParseMigration for Path {
    type Err = DefinitionError;

    fn parse_migration(&self) -> Result<Migration, Self::Err> {
        let path = self.parent().unwrap_or_else(|| Self::new(""));
        let filename = self.file_name().ok_or(DefinitionError::InvalidFilename)?;
        let filename = filename
            .to_str()
            .ok_or(DefinitionError::InvalidUtf8Character)?;

        parse_migration(path, filename)
    }
}

pub trait GetFilename {
    fn get_filename(&self, migration: &NewMigration) -> String;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[must_use]
pub struct MigrationFilenameStrategy {
    pub up_postfix: bool,
}

impl Default for MigrationFilenameStrategy {
    fn default() -> Self {
        Self { up_postfix: true }
    }
}

impl MigrationFilenameStrategy {
    pub const fn with_up_postfix(mut self, up_postfix: bool) -> Self {
        self.up_postfix = up_postfix;
        self
    }
}

impl GetFilename for MigrationFilenameStrategy {
    fn get_filename(&self, migration: &NewMigration) -> String {
        let key = migration.key.format(MIGRATION_KEY_FORMAT_STR).to_string();
        let title = migration.title.replace(' ', "_");
        let extension = match (migration.kind, self.up_postfix) {
            (MigrationKind::Up, true) => UP_SCRIPT_FILE_EXTENSION,
            (MigrationKind::Up, false) => SCRIPT_FILE_EXTENSION,
            (MigrationKind::Down, _) => DOWN_SCRIPT_FILE_EXTENSION,
            (MigrationKind::Baseline, _) => panic!("baselines do not have migration scripts"),
        };
        if title.is_empty() {
            format!("{key}{extension}")
        } else {
            format!("{key}_{title}{extension}")
        }
    }
}

#[derive(Clone, Debug)]
struct FilePattern {
    pattern: String,
    regex: Regex,
    filename_pattern: bool,
}

impl FilePattern {
    #[allow(clippy::missing_const_for_fn)]
    fn is_filename_pattern(&self) -> bool {
        self.filename_pattern
    }

    fn is_match(&self, haystack: &str) -> bool {
        self.regex.is_match(haystack)
    }
}

impl PartialEq for FilePattern {
    fn eq(&self, other: &Self) -> bool {
        self.pattern == other.pattern
    }
}

impl Display for FilePattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.pattern)
    }
}

impl FromStr for FilePattern {
    type Err = FilePatternError;

    fn from_str(pattern_str: &str) -> Result<Self, Self::Err> {
        if pattern_str.is_empty() {
            Err(FilePatternError::EmptySubPatternNotAllowed)
        } else {
            let invalid_chars = scan_for_invalid_characters(pattern_str);
            if invalid_chars.is_empty() {
                let filename_pattern = !pattern_str.contains('/');
                let mut regex_pattern = String::from("^");
                regex_pattern.push_str(
                    &pattern_str
                        .replace('.', "\\.")
                        .replace("**", ".?")
                        .replace('*', "[^/]*")
                        .replace(".?", ".*"),
                );
                regex_pattern.push('$');
                Regex::new(&regex_pattern)
                    .map_err(|err| FilePatternError::InvalidPattern(err.to_string()))
                    .map(|regex| Self {
                        pattern: pattern_str.into(),
                        regex,
                        filename_pattern,
                    })
            } else {
                Err(FilePatternError::InvalidCharacter(invalid_chars))
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ExcludedFiles {
    pattern: Vec<FilePattern>,
}

impl Default for ExcludedFiles {
    fn default() -> Self {
        DEFAULT_EXCLUDED_FILES.parse()
            .unwrap_or_else(|err| panic!("failed to create default `ExcludedFiles`: {err} -- THIS IS AN IMPLEMENTATION ERROR! Please file a bug."))
    }
}

impl ExcludedFiles {
    pub const fn empty() -> Self {
        Self {
            pattern: Vec::new(),
        }
    }

    pub fn matches(&self, path: &Path) -> bool {
        let filename = path
            .file_name()
            .map_or(Cow::Borrowed(""), OsStr::to_string_lossy);
        if filename.is_empty() {
            return false;
        }
        let path_str = path.to_string_lossy().replace('\\', "/");
        self.pattern.iter().any(|p| {
            if p.is_filename_pattern() {
                p.is_match(&filename)
            } else {
                p.is_match(&path_str)
            }
        })
    }
}

impl Display for ExcludedFiles {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for pattern in &self.pattern {
            if first {
                first = false;
            } else {
                f.write_char('|')?;
            }
            write!(f, "{pattern}")?;
        }
        Ok(())
    }
}

impl FromStr for ExcludedFiles {
    type Err = FilePatternError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Ok(Self {
                pattern: Vec::new(),
            });
        }
        let pattern = s
            .split('|')
            .map(FromStr::from_str)
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { pattern })
    }
}

fn scan_for_invalid_characters(s: &str) -> Vec<char> {
    s.chars().filter(|c| !is_valid_pattern_char(*c)).collect()
}

fn is_valid_pattern_char(c: char) -> bool {
    match c {
        '*' | ' ' | '_' | '-' | '.' | '/' => true,
        _ if c.is_alphanumeric() => true,
        _ => false,
    }
}

#[cfg(test)]
mod tests;
