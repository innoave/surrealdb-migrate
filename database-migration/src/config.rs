use crate::definition::ExcludedFiles;
use std::borrow::Cow;
use std::path::Path;

pub const DEFAULT_MIGRATIONS_FOLDER: &str = "migrations";
pub const DEFAULT_MIGRATIONS_TABLE: &str = "migrations";
pub const DEFAULT_EXCLUDED_FILES: &str = ".*|README*|TODO*";

pub const MIGRATION_KEY_FORMAT_STR: &str = "%Y%m%d_%H%M%S";

#[must_use]
#[derive(Debug, Clone, PartialEq)]
pub struct RunnerConfig<'a> {
    pub migrations_folder: Cow<'a, Path>,
    pub excluded_files: ExcludedFiles,
    pub migrations_table: Cow<'a, str>,
    pub ignore_checksum: bool,
    pub ignore_order: bool,
}

impl Default for RunnerConfig<'_> {
    fn default() -> Self {
        let excluded_files = ExcludedFiles::default();

        Self {
            migrations_folder: Path::new(DEFAULT_MIGRATIONS_FOLDER).into(),
            excluded_files,
            migrations_table: DEFAULT_MIGRATIONS_TABLE.into(),
            ignore_checksum: false,
            ignore_order: false,
        }
    }
}

impl<'a> RunnerConfig<'a> {
    pub fn with_migrations_folder(mut self, migrations_folder: impl Into<Cow<'a, Path>>) -> Self {
        self.migrations_folder = migrations_folder.into();
        self
    }

    pub fn with_migrations_table(mut self, migrations_table: impl Into<Cow<'a, str>>) -> Self {
        self.migrations_table = migrations_table.into();
        self
    }

    pub const fn with_ignore_checksum(mut self, ignore_checksum: bool) -> Self {
        self.ignore_checksum = ignore_checksum;
        self
    }

    pub const fn with_ignore_order(mut self, ignore_order: bool) -> Self {
        self.ignore_order = ignore_order;
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DbAuthLevel {
    Root,
    Namespace,
    Database,
}

#[must_use]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DbClientConfig<'a> {
    /// Address of the database instance.
    ///
    /// Examples:
    /// - `"ws://localhost:8000"`
    /// - `"wss://cloud.surrealdb.com"`
    ///
    /// Default: `"ws://localhost:8000"`
    pub address: Cow<'a, str>,

    /// Namespace to use on the database instance.
    ///
    /// Default: `"test"`
    pub namespace: Cow<'a, str>,

    /// Database to use inside the database instance.
    ///
    /// Default: `"test"`
    pub database: Cow<'a, str>,

    /// The kind of the system user used for authentication.
    ///
    /// Default: `Root`
    pub auth_level: DbAuthLevel,

    /// Username used to authenticate to the database instance.
    ///
    /// Default: `"root"`
    pub username: Cow<'a, str>,

    /// Password used to authenticate to the database instance.
    ///
    /// Default: `"root"`
    pub password: Cow<'a, str>,

    /// Capacity of the channels to the database.
    ///
    /// Example:
    /// - `0` (= unbounded)
    /// - `200`
    ///
    /// Default: `20`
    pub capacity: usize,
}

impl Default for DbClientConfig<'_> {
    fn default() -> Self {
        Self {
            address: "ws://localhost:8000".into(),
            namespace: "test".into(),
            database: "test".into(),
            auth_level: DbAuthLevel::Root,
            username: "root".into(),
            password: "root".into(),
            capacity: 20,
        }
    }
}

impl<'a> DbClientConfig<'a> {
    pub fn with_address(mut self, address: impl Into<Cow<'a, str>>) -> Self {
        self.address = address.into();
        self
    }

    pub fn with_namespace(mut self, namespace: impl Into<Cow<'a, str>>) -> Self {
        self.namespace = namespace.into();
        self
    }

    pub fn with_database(mut self, database: impl Into<Cow<'a, str>>) -> Self {
        self.database = database.into();
        self
    }

    pub const fn with_auth_level(mut self, auth_level: DbAuthLevel) -> Self {
        self.auth_level = auth_level;
        self
    }

    pub fn with_username(mut self, username: impl Into<Cow<'a, str>>) -> Self {
        self.username = username.into();
        self
    }

    pub fn with_password(mut self, password: impl Into<Cow<'a, str>>) -> Self {
        self.password = password.into();
        self
    }

    pub const fn with_capacity(mut self, capacity: usize) -> Self {
        self.capacity = capacity;
        self
    }
}
