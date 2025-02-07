#![allow(clippy::doc_markdown, clippy::struct_excessive_bools)]

use std::path::PathBuf;

/// Create and apply migrations for a SurrealDB database.
#[derive(clap::Parser, Debug, Clone)]
#[clap(name = "surmig", version)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
    /// Path to the folder containing the surrealdb-migrate.toml config file
    #[clap(long)]
    pub config_dir: Option<PathBuf>,
    /// Path to the folder that contains the migration files
    #[clap(long)]
    pub migrations_folder: Option<PathBuf>,
    /// Address of the database server, e.g. "ws://localhost:8000"
    #[clap(long)]
    pub db_address: Option<String>,
}

#[derive(clap::Subcommand, Debug, Clone)]
pub enum Command {
    /// Create a new migration file.
    #[clap(aliases = &["cr"])]
    Create(CreateArgs),
    /// Apply all new migrations to the database.
    #[clap(aliases = &["m"])]
    Migrate(MigrateArgs),
    /// Revert migrations on the database, running down migrations.
    #[clap(aliases = &["r"])]
    Revert(RevertArgs),
    /// List migrations defined and/or applied to the database.
    #[clap(aliases = &["ls"])]
    List(ListArgs),
    /// Verify applied migrations against the defined ones.
    Verify(VerifyArgs),
}

#[derive(clap::Args, Debug, Clone)]
pub struct CreateArgs {
    /// The key of the new migration in the format YYYYMMDD_HHMMSS. Default: &lt;current date and time&gt;.
    #[clap(long, short)]
    pub key: Option<String>,
    /// The title of the new migration. Default: &lt;no title&gt;.
    pub title: Option<String>,
    /// Also create a new down migration file.
    #[clap(long, short, action)]
    pub down: bool,
}

#[derive(clap::Args, Debug, Clone)]
pub struct MigrateArgs {
    /// Only applies new migrations up to the migration with the given key (inclusive).
    #[clap(long)]
    pub to: Option<String>,
    /// Do not verify checksum of already applied migrations.
    #[clap(long, action)]
    pub ignore_checksum: bool,
    /// Do not verify the order of migrations to be applied.
    #[clap(long, action)]
    pub ignore_order: bool,
}

#[derive(clap::Args, Debug, Clone)]
pub struct RevertArgs {
    /// Only reverts migrations down to the migration with the given key (exclusive).
    #[clap(long)]
    pub to: Option<String>,
}

#[derive(clap::Args, Debug, Clone)]
pub struct ListArgs {
    /// list all forward migrations (default).
    #[clap(long, short, action)]
    pub up: bool,
    /// only lists backward migrations.
    #[clap(long, short, action)]
    pub down: bool,
    /// only lists applied migrations.
    #[clap(long, short = 'x', action)]
    pub applied: bool,
    /// only lists defined but not yet applied migrations.
    #[clap(long, short, action)]
    pub open: bool,
}

#[derive(clap::Args, Debug, Clone)]
pub struct VerifyArgs {
    /// only verify the checksum
    #[clap(long, short, action)]
    pub checksum: bool,
    /// only verify the order
    #[clap(long, short, action)]
    pub order: bool,
}
