use database_migration::checksum::hash_migration_script;
use database_migration::definition::ParseMigration;
use database_migration::error::Error;
use database_migration::migration::{Migration, ScriptContent};
use std::fs;
#[cfg(target_family = "windows")]
use std::os::windows::fs::FileTypeExt;
use std::path::Path;

pub trait ListMigrations {
    type Iter: Iterator<Item = Result<Migration, Error>>;

    fn list_all_migrations(&self) -> Result<Self::Iter, Error>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MigrationDirectory<'a> {
    path: &'a Path,
}

impl<'a> MigrationDirectory<'a> {
    pub const fn new(path: &'a Path) -> Self {
        Self { path }
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

pub fn migration_directory(path: &str) -> MigrationDirectory<'_> {
    MigrationDirectory::new(Path::new(path))
}

pub fn read_script_content_for_migrations(
    migrations: &[Migration],
) -> Result<Vec<ScriptContent>, Error> {
    let mut script_contents = Vec::with_capacity(migrations.len());
    for migration in migrations {
        let content = fs::read_to_string(&migration.script_path)
            .map_err(|err| Error::ReadingMigrationFile(err.to_string()))?;
        let checksum = hash_migration_script(migration, &content);
        script_contents.push(ScriptContent {
            key: migration.key,
            kind: migration.kind,
            path: migration.script_path.clone(),
            content,
            checksum,
        });
    }
    Ok(script_contents)
}

#[cfg(test)]
mod tests;
