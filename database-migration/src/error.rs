use crate::migration::ProblematicMigration;
use indexmap::IndexMap;

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum Error {
    #[error("migrations changed after execution: {0:?}")]
    ChangedAfterExecution(Vec<ProblematicMigration>),
    #[error("failed to load settings: {0}")]
    Configuration(String),
    #[error("failed to create migrations folder: {0}")]
    CreatingMigrationsFolder(String),
    #[error("failed to create script file: {0}")]
    CreatingScriptFile(String),
    #[error("database query failed: {0}")]
    DbQuery(String),
    #[error("db script execution failed: {0:?}")]
    DbScript(IndexMap<usize, String>),
    #[error(transparent)]
    Definition(#[from] DefinitionError),
    #[error("failed to insert the migration execution for key={0} into the migrations table")]
    ExecutionNotInserted(String),
    #[error("failed to delete the migration execution for key={0} from the migrations table")]
    ExecutionNotDeleted(String),
    #[error("failed to query table definitions: {0}")]
    FetchingTableDefinitions(String),
    #[error("migrations out of order: {0:?}")]
    OutOfOrder(Vec<ProblematicMigration>),
    #[error("failed reading migration files: {0}")]
    ReadingMigrationFile(String),
    #[error("failed scanning migration directory: {0}")]
    ScanningMigrationDirectory(String),
}

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum DefinitionError {
    #[error("direction is ambiguous")]
    AmbiguousDirection,
    #[error("invalid date: {0}")]
    InvalidDate(String),
    #[error("invalid time: {0}")]
    InvalidTime(String),
    #[error("definition contains an invalid utf-8 character")]
    InvalidUtf8Character,
    #[error("definition does not contain a date")]
    MissingDate,
    #[error("definition does not contain a time")]
    MissingTime,
    #[error("filename of definition is invalid")]
    InvalidFilename,
}

#[derive(thiserror::Error, Debug, Clone, PartialEq, Eq)]
pub enum FilePatternError {
    #[error("invalid characters '{0:?}' in filename pattern")]
    InvalidCharacter(Vec<char>),
    #[error("invalid pattern: {0}")]
    InvalidPattern(String),
    #[error("an empty pattern together with OR ('|') is not allowed")]
    EmptySubPatternNotAllowed,
}

impl From<FilePatternError> for Error {
    fn from(value: FilePatternError) -> Self {
        Self::Configuration(value.to_string())
    }
}
