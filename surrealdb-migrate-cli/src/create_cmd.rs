use crate::args::CreateArgs;
use chrono::{NaiveDateTime, Utc};
use color_eyre::Report;
use color_eyre::eyre::{ContextCompat, WrapErr};
use surrealdb_migrate::config::{MIGRATION_KEY_FORMAT_STR, RunnerConfig};
use surrealdb_migrate::definition::MigrationFilenameStrategy;
use surrealdb_migrate::files::MigrationDirectory;
use surrealdb_migrate::migration::{MigrationKind, NewMigration};
use surrealdb_migrate::repository::CreateNewMigration;

#[allow(clippy::needless_pass_by_value)]
pub fn run(args: CreateArgs, config: RunnerConfig<'_>) -> Result<(), Report> {
    let mig_dir = MigrationDirectory::new(&config.migrations_folder, &config.excluded_files);
    mig_dir.create_directory_if_not_existing()?;

    let key = args
        .key
        .map_or_else(
            || Ok(Utc::now().naive_local()),
            |arg| NaiveDateTime::parse_from_str(&arg, MIGRATION_KEY_FORMAT_STR),
        )
        .wrap_err("Invalid key! Please specify a key in the format YYYYmmdd_HHMMSS.")?;
    let title = args.title.unwrap_or_default();

    let (new_migration, down_migration) = if args.down {
        (
            NewMigration {
                key,
                title: title.clone(),
                kind: MigrationKind::Up,
            },
            Some(NewMigration {
                key,
                title,
                kind: MigrationKind::Down,
            }),
        )
    } else {
        (
            NewMigration {
                key,
                title,
                kind: MigrationKind::Up,
            },
            None,
        )
    };

    let filename_strategy = MigrationFilenameStrategy::default();

    let up_migration = mig_dir
        .files(filename_strategy)
        .create_new_migration(new_migration)?;

    let up_filename = up_migration.script_path.file_name().wrap_err("migration script path does not have a filename part - should not be reachable - please report a bug!")?.to_string_lossy();

    let down_filename = if let Some(down_migration) = down_migration {
        let down_migration = mig_dir
            .files(filename_strategy)
            .create_new_migration(down_migration)?;
        let down_filename = down_migration.script_path.file_name().wrap_err("migration script path does not have a filename part - should not be reachable - please report a bug!")?.to_string_lossy().to_string();
        Some(down_filename)
    } else {
        None
    };

    println!();
    println!(
        "Migrations located at {}:",
        config.migrations_folder.display()
    );
    println!();
    println!("New migration {up_filename} created.");
    if let Some(down_filename) = down_filename {
        println!("New backward migration {down_filename} created.");
    }
    println!();

    Ok(())
}
