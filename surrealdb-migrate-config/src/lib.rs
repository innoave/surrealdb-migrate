use config::{Config, File, FileFormat};
use database_migration::config::{DbAuthLevel, DbClientConfig, RunnerConfig};
use database_migration::error::Error;
use serde::de::{Unexpected, Visitor};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::env;
use std::fmt::Formatter;
use std::path::Path;

pub const CONFIG_DIR_ENVIRONMENT_VAR: &str = "SURREALDB_MIGRATE_CONFIG_DIR";
pub const SETTINGS_FILENAME: &str = "surrealdb-migrate";

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
            environment_toml.push_str(&format!("{key} = \"{val}\"\n"));
        }
    }
    if !files.is_empty() {
        environment_toml.push_str("[files]\n");
        for (key, val) in files {
            environment_toml.push_str(&format!("{key} = \"{val}\"\n"));
        }
    }
    if !database.is_empty() {
        environment_toml.push_str("[database]\n");
        for (key, val) in database {
            environment_toml.push_str(&format!("{key} = \"{val}\"\n"));
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
        let config_file = path.join(SETTINGS_FILENAME);
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
            ignore_checksums: self.migration.ignore_checksum,
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
