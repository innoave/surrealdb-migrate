//! Configuration mechanism for the [`surrealdb-migrate`] crate.
//!
//! To be able to use the `MigrationRunner` of the [`surrealdb-migrate`] crate
//! some configuration settings are needed for the runner itself and the
//! DB-connection to the database the migrations shall be applied to. These
//! configuration settings can be provided by an application through its own
//! configuration mechanism or this crate is used to load the settings from
//! a configuration file and/or some environment variables.
//!
//! ## Concept
//!
//! All settings have default values. The user needs to provide only those
//! settings which shall get a value different from the default value and omit
//! those settings where the default value is suitable for the application.
//!
//! First the settings are loaded from a configuration file with the name
//! `surrealdb-migrate.toml`. By default, this configuration file must be
//! located in the current working directory. To load the configuration file
//! from a different directory, the environment variable
//! `SURREALDB_MIGRATE_CONFIG_DIR` can be set to point to the directory where
//! the configuration file is located. For example:
//!
//! ```dotenv
//! SURREALDB_MIGRATE_CONFIG_DIR="my_application/config"
//! ```
//!
//! The configuration does not need to define all available settings. If a
//! setting is not present in the configuration file the default value is used.
//! If the configuration file is not present or can not be found, the default
//! values for all settings are used.
//!
//! All available settings with their default values are listed in the example
//! configuration file [surrealdb-migrate.default.toml].
//!
//! In a second step the settings are loaded from the environment. Each
//! specified environment variable that defines a settings overwrites this
//! setting in the resulting configuration.
//!
//! If an environment variable is not set, the value from the configuration file
//! is used. If a setting is neither specified in the configuration file nor set
//! via an environment variable, the default value is used.
//!
//! All available environment variables that define configuration settings are
//! listed in the example dotenv file [default.env].
//!
//! ## Usage
//!
//! To load the settings via the mechanism described in the previous chapter,
//! the [`load()`](Settings::load) function of the [`Settings`] struct is used.
//!
//! ```no_run
//! use database_migration::error::Error;
//! use surrealdb_migrate_config::Settings;
//!
//! fn main() -> Result<(), Error> {
//!     let _settings = Settings::load()?;
//!
//!     Ok(())
//! }
//! ```
//!
//! The [`load()`](Settings::load) function searches for the configuration file
//! in the directory specified by the environment variable
//! `SURREALDB_MIGRATE_CONFIG_DIR` if set or in the current working directory
//! otherwise.
//!
//! The directory where the configuration file is located can also be specified
//! when using the [`load_from_dir()`](Settings::load_from_dir()) function
//! instead.
//!
//! ```no_run
//! use std::path::Path;
//! use database_migration::error::Error;
//! use surrealdb_migrate_config::Settings;
//!
//! fn main() -> Result<(), Error> {
//!     let _settings = Settings::load_from_dir(Path::new("my_application/config"))?;
//!
//!     Ok(())
//! }
//! ```
//!
//! The loaded settings can then be used to get a [`DbClientConfig`] and a
//! [`RunnerConfig`].
//!
//! ```no_run
//! # use database_migration::error::Error;
//! # use surrealdb_migrate_config::Settings;
//!
//! fn main() -> Result<(), Error> {
//!     let settings = Settings::load()?;
//!
//!     let _db_config = settings.db_client_config();
//!     let _runner_config = settings.runner_config();
//!
//!     Ok(())
//! }
//! ```
//!
//! [default.env]: https://github.com/innoave/surrealdb-migrate/blob/main/surrealdb-migrate-config/resources/default.env
//! [surrealdb-migrate.default.toml]: https://github.com/innoave/surrealdb-migrate/blob/main/surrealdb-migrate-config/resources/surrealdb-migrate.default.toml
//! [`surrealdb-migrate`]: https://docs.rs/surrealdb-migrate/0.1.0

mod env;

use config::{Config, File, FileFormat};
use database_migration::config::{DbAuthLevel, DbClientConfig, RunnerConfig};
use database_migration::error::Error;
use serde::de::{Unexpected, Visitor};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt::{Formatter, Write as _};
use std::path::Path;

pub const CONFIG_DIR_ENVIRONMENT_VAR: &str = "SURREALDB_MIGRATE_CONFIG_DIR";
pub const CONFIG_FILENAME: &str = "surrealdb-migrate";

const DEFAULT_SETTINGS: &str = include_str!("../resources/surrealdb-migrate.default.toml");

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct Settings {
    pub migration: MigrationSettings,
    pub files: FilesSettings,
    pub database: DatabaseSettings,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct MigrationSettings {
    pub ignore_checksum: bool,
    pub ignore_order: bool,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct FilesSettings {
    pub migrations_folder: String,
    pub script_extension: String,
    pub up_script_extension: String,
    pub down_script_extension: String,
}

#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub struct DatabaseSettings {
    pub migrations_table: String,
    pub address: String,
    pub username: String,
    pub password: String,
    #[serde(deserialize_with = "db_auth_level_from_string")]
    pub auth_level: DbAuthLevel,
    pub namespace: String,
    pub database: String,
    pub capacity: usize,
}

fn read_environment() -> String {
    const MIGRATION_PREFIX: &str = "SURMIG_MIGRATION_";
    const FILES_PREFIX: &str = "SURMIG_FILES_";
    const DATABASE_PREFIX: &str = "SURMIG_DATABASE_";

    let mut migration = HashMap::new();
    let mut files = HashMap::new();
    let mut database = HashMap::new();

    for (key, val) in env::vars() {
        if key.starts_with(MIGRATION_PREFIX) {
            let offset = MIGRATION_PREFIX.len();
            migration.insert(to_kebab_case(&key, offset), val);
        } else if key.starts_with(DATABASE_PREFIX) {
            let offset = DATABASE_PREFIX.len();
            database.insert(to_kebab_case(&key, offset), val);
        } else if key.starts_with(FILES_PREFIX) {
            let offset = FILES_PREFIX.len();
            files.insert(to_kebab_case(&key, offset), val);
        }
    }

    let mut environment_toml = String::new();
    if !migration.is_empty() {
        environment_toml.push_str("[migration]\n");
        for (key, val) in migration {
            let _ = writeln!(environment_toml, "{key} = \"{val}\"");
        }
    }
    if !files.is_empty() {
        environment_toml.push_str("[files]\n");
        for (key, val) in files {
            let _ = writeln!(environment_toml, "{key} = \"{val}\"");
        }
    }
    if !database.is_empty() {
        environment_toml.push_str("[database]\n");
        for (key, val) in database {
            let _ = writeln!(environment_toml, "{key} = \"{val}\"");
        }
    }
    environment_toml
}

fn to_kebab_case(s: &str, offset: usize) -> String {
    s.chars()
        .skip(offset)
        .map(|c| {
            if c == '_' {
                '-'
            } else {
                c.to_ascii_lowercase()
            }
        })
        .collect()
}

fn db_auth_level_from_string<'de, D>(deserializer: D) -> Result<DbAuthLevel, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_str(DbAuthLevelVisitor)
}

struct DbAuthLevelVisitor;

impl Visitor<'_> for DbAuthLevelVisitor {
    type Value = DbAuthLevel;

    fn expecting(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        formatter
            .write_str("expecting a string containing one of 'Root', 'Namespace' or 'Database'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match &v.to_ascii_lowercase()[..] {
            "root" => Ok(DbAuthLevel::Root),
            "namespace" => Ok(DbAuthLevel::Namespace),
            "database" => Ok(DbAuthLevel::Database),
            _ => Err(serde::de::Error::invalid_value(
                Unexpected::Str(v),
                &"Root, Namespace or Database",
            )),
        }
    }
}

impl Settings {
    pub fn load() -> Result<Self, Error> {
        let config_dir = env::var(CONFIG_DIR_ENVIRONMENT_VAR).unwrap_or_else(|_| "./".into());
        Self::load_from_dir(Path::new(&config_dir))
    }

    pub fn load_from_dir(path: &Path) -> Result<Self, Error> {
        let environment = read_environment();
        let config_file = path.join(CONFIG_FILENAME);
        let config = Config::builder()
            .add_source(File::from_str(DEFAULT_SETTINGS, FileFormat::Toml))
            .add_source(File::from(config_file).required(false))
            .add_source(File::from_str(&environment, FileFormat::Toml))
            .build()
            .map_err(|err| Error::Configuration(err.to_string()))?;

        config
            .try_deserialize()
            .map_err(|err| Error::Configuration(err.to_string()))
    }

    pub fn runner_config(&self) -> RunnerConfig<'_> {
        RunnerConfig {
            migrations_folder: Path::new(&self.files.migrations_folder).into(),
            migrations_table: (&self.database.migrations_table).into(),
            ignore_checksum: self.migration.ignore_checksum,
            ignore_order: self.migration.ignore_order,
        }
    }

    pub fn db_client_config(&self) -> DbClientConfig<'_> {
        DbClientConfig {
            address: (&self.database.address).into(),
            namespace: (&self.database.namespace).into(),
            database: (&self.database.database).into(),
            auth_level: self.database.auth_level,
            username: (&self.database.username).into(),
            password: (&self.database.password).into(),
            capacity: self.database.capacity,
        }
    }
}

#[cfg(test)]
mod tests;
