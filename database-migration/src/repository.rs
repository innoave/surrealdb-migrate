use crate::error::Error;
use crate::migration::{Migration, NewMigration, ScriptContent};

pub trait ListMigrations {
    type Iter: Iterator<Item = Result<Migration, Error>>;

    fn list_all_migrations(&self) -> Result<Self::Iter, Error>;
}

pub trait ReadScriptContent {
    fn read_script_content(&self, migration: &Migration) -> Result<ScriptContent, Error>;

    fn read_script_content_for_migrations(
        &self,
        migrations: &[Migration],
    ) -> Result<Vec<ScriptContent>, Error> {
        migrations
            .iter()
            .map(|mig| self.read_script_content(mig))
            .collect()
    }
}

pub trait CreateNewMigration {
    fn create_new_migration(&self, new_migration: NewMigration) -> Result<Migration, Error>;
}
