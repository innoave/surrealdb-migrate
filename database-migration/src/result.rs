use chrono::NaiveDateTime;

/// Result of a migration action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Migrated {
    /// No migrations have been applied. The database is fully migrated already.
    Nothing,
    /// Migrated the database to the specified migration key (version).
    UpTo(NaiveDateTime),
    /// No forward migrations found in the migrations folder.
    NoForwardMigrationsFound,
}

/// Result of a revert action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reverted {
    /// No migrations have been reverted. The database is completely reverted already.
    Nothing,
    /// Reverted the database to the specified migration key (version).
    DownTo(NaiveDateTime),
    /// No backward migrations found in the migrations folder.
    NoBackwardMigrationsFound,
}
