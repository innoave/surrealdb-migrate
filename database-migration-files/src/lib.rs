use database_migration::checksum::hash_migration_script;
use database_migration::definition::{GetFilename, ParseMigration};
use database_migration::error::Error;
use database_migration::migration::{Migration, NewMigration, ScriptContent};
use std::fs;
use std::fs::File;
#[cfg(target_family = "windows")]
use std::os::windows::fs::FileTypeExt;
use std::path::{Path, PathBuf};

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

pub fn create_migrations_folder_if_not_existing(
    path: &Path,
    folder_name: &str,
) -> Result<PathBuf, Error> {
    let migrations_folder = path.join(folder_name);
    if migrations_folder.exists() {
        return Ok(migrations_folder);
    }
    fs::create_dir_all(&migrations_folder)
        .map_err(|err| Error::CreatingMigrationsFolder(err.to_string()))?;
    Ok(migrations_folder)
}

pub fn create_migration_file(
    filename_strategy: &impl GetFilename,
    migrations_folder: &Path,
    new_migration: &NewMigration,
) -> Result<PathBuf, Error> {
    let filename = filename_strategy.get_filename(new_migration);
    let script_path = migrations_folder.join(&filename);
    File::create_new(&script_path).map_err(|err| Error::CreatingScriptFile(err.to_string()))?;
    Ok(script_path)
}

#[cfg(test)]
mod tests;
