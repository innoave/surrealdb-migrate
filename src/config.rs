use std::borrow::Cow;

pub const DEFAULT_MIGRATIONS_FOLDER: &str = "migrations";
pub const DEFAULT_MIGRATIONS_TABLE: &str = "migrations";

pub const MIGRATION_KEY_FORMAT_STR: &str = "%Y%m%d_%H%M%S";

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
    pub address: Option<Cow<'a, str>>,

    /// Namespace to use on the database instance.
    ///
    /// Default: `"test"`
    pub namespace: Option<Cow<'a, str>>,

    /// Database to use inside the database instance.
    ///
    /// Default: `"test"`
    pub database: Option<Cow<'a, str>>,

    /// The kind of the system user used for authentication.
    ///
    /// Default: `Root`
    pub auth_level: DbAuthLevel,

    /// Username used to authenticate to the database instance.
    ///
    /// Default: `"root"`
    pub username: Option<Cow<'a, str>>,

    /// Password used to authenticate to the database instance.
    ///
    /// Default: `"root"`
    pub password: Option<Cow<'a, str>>,

    /// Capacity of the channels to the database.
    ///
    /// Example:
    /// - `0` (= unbounded)
    /// - `200`
    ///
    /// Default: `20`
    pub capacity: Option<usize>,
}

impl Default for DbClientConfig<'_> {
    fn default() -> Self {
        Self {
            address: None,
            namespace: None,
            database: None,
            auth_level: DbAuthLevel::Root,
            username: None,
            password: None,
            capacity: None,
        }
    }
}

#[allow(clippy::missing_const_for_fn)]
impl<'a> DbClientConfig<'a> {
    pub fn with_address(mut self, address: impl Into<Cow<'a, str>>) -> Self {
        self.address = Some(address.into());
        self
    }

    pub fn with_namespace(mut self, namespace: impl Into<Cow<'a, str>>) -> Self {
        self.namespace = Some(namespace.into());
        self
    }

    pub fn with_database(mut self, database: impl Into<Cow<'a, str>>) -> Self {
        self.database = Some(database.into());
        self
    }

    pub fn with_auth_level(mut self, auth_level: DbAuthLevel) -> Self {
        self.auth_level = auth_level;
        self
    }

    pub fn with_username(mut self, username: impl Into<Cow<'a, str>>) -> Self {
        self.username = Some(username.into());
        self
    }

    pub fn with_password(mut self, password: impl Into<Cow<'a, str>>) -> Self {
        self.password = Some(password.into());
        self
    }

    pub fn with_capacity(mut self, capacity: usize) -> Self {
        self.capacity = Some(capacity);
        self
    }

    pub fn address_or_default(&self) -> &str {
        self.address
            .as_ref()
            .map_or("ws://localhost:8000", |v| v.as_ref())
    }

    pub fn namespace_or_default(&self) -> &str {
        self.namespace.as_ref().map_or("test", |v| v.as_ref())
    }

    pub fn database_or_default(&self) -> &str {
        self.database.as_ref().map_or("test", |v| v.as_ref())
    }

    pub fn auth_level(&self) -> DbAuthLevel {
        self.auth_level
    }

    pub fn username_or_default(&self) -> &str {
        self.username.as_ref().map_or("root", |v| v.as_ref())
    }

    pub fn password_or_default(&self) -> &str {
        self.password.as_ref().map_or("root", |v| v.as_ref())
    }

    pub fn capacity_or_default(&self) -> usize {
        self.capacity.unwrap_or(20)
    }
}
