use crate::migration::{Migration, MigrationKind};
use crc32fast::Hasher;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::borrow::Borrow;
use std::ffi::OsStr;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::ops::Deref;
use std::str::FromStr;

#[derive(SerializeDisplay, DeserializeFromStr, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Checksum(u32);

impl Display for Checksum {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Checksum {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        u32::from_str(s).map(Self)
    }
}

impl Deref for Checksum {
    type Target = u32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<u32> for Checksum {
    fn as_ref(&self) -> &u32 {
        &self.0
    }
}

impl Borrow<u32> for Checksum {
    fn borrow(&self) -> &u32 {
        &self.0
    }
}

pub fn hash_migration_script(migration: &Migration, script_content: &[u8]) -> Checksum {
    let mut hasher = Hasher::new();
    hasher.update(
        migration
            .script_path
            .file_name()
            .unwrap_or_else(|| OsStr::new(""))
            .as_encoded_bytes(),
    );
    hasher.update(match migration.kind {
        MigrationKind::Baseline => &[0],
        MigrationKind::Up => &[1],
        MigrationKind::Down => &[2],
    });
    hasher.update(script_content);
    Checksum(hasher.finalize())
}

#[cfg(test)]
mod tests;
