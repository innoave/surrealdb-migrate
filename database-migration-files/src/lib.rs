#![doc(html_root_url = "https://docs.rs/database-migration-files/0.2.0")]

use database_migration::checksum::hash_migration_script;
use database_migration::definition::{ExcludedFiles, GetFilename, ParseMigration};
use database_migration::error::Error;
use database_migration::migration::{Migration, NewMigration, ScriptContent};
use database_migration::repository::{CreateNewMigration, ListMigrations, ReadScriptContent};
use std::fs;
use std::fs::File;
#[cfg(target_family = "windows")]
use std::os::windows::fs::FileTypeExt;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Clone)]
pub struct MigrationDirectory<'a> {
    path: &'a Path,
    excluded_files: &'a ExcludedFiles,
}

impl<'a> MigrationDirectory<'a> {
    pub const fn new(path: &'a Path, excluded_files: &'a ExcludedFiles) -> Self {
        Self {
            path,
            excluded_files,
        }
    }

    pub fn create_directory_if_not_existing(&self) -> Result<(), Error> {
        if self.path.exists() {
            return Ok(());
        }
        fs::create_dir_all(self.path)
            .map_err(|err| Error::CreatingMigrationsFolder(err.to_string()))
    }

    pub const fn files<S>(&self, filename_strategy: S) -> MigrationFiles<'a, S> {
        MigrationFiles::new(self.path, filename_strategy)
    }
}

impl ListMigrations for MigrationDirectory<'_> {
    type Iter = MigDirIter;

    fn list_all_migrations(&self) -> Result<Self::Iter, Error> {
        if !self.path.exists() {
            return Err(Error::ScanningMigrationDirectory(format!(
                r#"migrations folder "{}" does not exist"#,
                self.path.display()
            )));
        }
        let walk_dir = WalkDir::new(self.path);
        Ok(MigDirIter {
            walker: walk_dir.into_iter(),
            excluded_files: self.excluded_files.clone(),
        })
    }
}

impl ReadScriptContent for MigrationDirectory<'_> {
    fn read_script_content(&self, migration: &Migration) -> Result<ScriptContent, Error> {
        let content = fs::read_to_string(&migration.script_path)
            .map_err(|err| Error::ReadingMigrationFile(err.to_string()))?;
        let checksum = hash_migration_script(migration, &content);
        Ok(ScriptContent {
            key: migration.key,
            kind: migration.kind,
            path: migration.script_path.clone(),
            content,
            checksum,
        })
    }
}

#[derive(Debug)]
pub struct MigDirIter {
    walker: walkdir::IntoIter,
    excluded_files: ExcludedFiles,
}

impl Iterator for MigDirIter {
    type Item = Result<Migration, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        for dir_entry in &mut self.walker {
            return match dir_entry {
                Ok(entry) => {
                    let file_type = entry.file_type();
                    #[cfg(target_family = "windows")]
                    if file_type.is_dir() || file_type.is_symlink_dir() {
                        continue;
                    }
                    #[cfg(not(target_family = "windows"))]
                    if file_type.is_dir() {
                        continue;
                    }
                    let file_path = entry.path();
                    if self.excluded_files.matches(file_path) {
                        continue;
                    }
                    Some(file_path.parse_migration().map_err(Error::from))
                },
                Err(err) => Some(Err(Error::ScanningMigrationDirectory(err.to_string()))),
            };
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.walker.size_hint()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MigrationFiles<'a, S> {
    path: &'a Path,
    filename_strategy: S,
}

impl<'a, S> MigrationFiles<'a, S> {
    pub const fn new(path: &'a Path, filename_strategy: S) -> Self {
        Self {
            path,
            filename_strategy,
        }
    }
}

impl<S> CreateNewMigration for MigrationFiles<'_, S>
where
    S: GetFilename,
{
    fn create_new_migration(&self, new_migration: NewMigration) -> Result<Migration, Error> {
        let filename = self.filename_strategy.get_filename(&new_migration);
        let script_path = self.path.join(&filename);
        File::create_new(&script_path).map_err(|err| Error::CreatingScriptFile(err.to_string()))?;
        Ok(Migration {
            key: new_migration.key,
            title: new_migration.title,
            kind: new_migration.kind,
            script_path,
        })
    }
}

#[cfg(test)]
mod tests;

// workaround for false positive 'unused extern crate' warnings until
// Rust issue [#95513](https://github.com/rust-lang/rust/issues/95513) is fixed
#[cfg(test)]
mod dummy_extern_uses {
    use version_sync as _;
}
