use time::error::Parse;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
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
    #[error("failed reading migration files: {0}")]
    ReadingMigrationFile(String),
    #[error("failed scanning migration directory: {0}")]
    ScanningMigrationDirectory(String),
}
