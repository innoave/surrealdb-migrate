use database_migration::checksum::hash_migration_script;
use database_migration::definition::{GetFilename, ParseMigration};
use database_migration::error::Error;
use database_migration::migration::{Migration, NewMigration, ScriptContent};
use database_migration::repository::{CreateNewMigration, ListMigrations, ReadScriptContent};
use std::fs;
use std::fs::File;
#[cfg(target_family = "windows")]
use std::os::windows::fs::FileTypeExt;
use std::path::Path;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MigrationDirectory<'a> {
    path: &'a Path,
}

impl<'a> MigrationDirectory<'a> {
    pub const fn new(path: &'a Path) -> Self {
        Self { path }
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

impl<'a> ListMigrations for MigrationDirectory<'a> {
    type Iter = MigDirIter<'a>;

    fn list_all_migrations(&self) -> Result<Self::Iter, Error> {
        fs::read_dir(self.path)
            .map_err(|err| Error::ScanningMigrationDirectory(err.to_string()))
            .map(|dir_iter| MigDirIter {
                base_dir: self.path,
                dir_iter,
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
pub struct MigDirIter<'a> {
    base_dir: &'a Path,
    dir_iter: fs::ReadDir,
}

impl Iterator for MigDirIter<'_> {
    type Item = Result<Migration, Error>;

    fn next(&mut self) -> Option<Self::Item> {
        for dir_entry in &mut self.dir_iter {
            return match dir_entry {
                Ok(entry) => {
                    match entry.file_type() {
                        Ok(file_type) => {
                            #[cfg(target_family = "windows")]
                            if file_type.is_dir() || file_type.is_symlink_dir() {
                                continue;
                            }
                            #[cfg(not(target_family = "windows"))]
                            if file_type.is_dir() {
                                continue;
                            }
                        },
                        Err(err) => {
                            return Some(Err(Error::ScanningMigrationDirectory(err.to_string())))
                        },
                    }
                    let file_path = self.base_dir.join(entry.file_name());
                    Some(file_path.parse_migration().map_err(Error::from))
                },
                Err(err) => Some(Err(Error::ScanningMigrationDirectory(err.to_string()))),
            };
        }
        None
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.dir_iter.size_hint()
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
