# SurrealDB-Migrate

[![crates.io][crates-badge]][crates-url]
[![docs.rs][docs-badge]][docs-url]
[![Apache-2.0 licensed][license-badge]][license-url]
![MSRV][msrv-badge]
[![code coverage][code-coverage-badge]][code-coverage-url]

Evolve a [SurrealDB] database over time by applying migrations. SurrealDB-Migrate is a commandline
tool and lib to define and apply migrations on a database. It provides version control for a
[SurrealDB] database in a project.

SurrealDB-Migrate provides two ways to deal with migrations of a database:

* `surmig`: the command line tool to manage and apply migrations on a database in the terminal.
* `surrealdb-migrate`: the crate to manage and apply migrations programmatically from within an
  application.

## `surmig`: the command line tool

Install the command line tool from [crates.io][cli-crate-url]:

```console
$ cargo install surrealdb-migrate-cli
```

Run the help command to get a list of available commands and options:

```console
$ surmig help 
Create and apply migrations for a SurrealDB database

Usage: surmig [OPTIONS] <COMMAND>

Commands:
  create   Create a new migration file
  migrate  Apply all new migrations to the database
  revert   Revert migrations on the database, running down migrations
  list     List migrations defined and/or applied to the database
  verify   Verify applied migrations against the defined ones
  help     Print this message or the help of the given subcommand(s)

Options:
      --config-dir <CONFIG_DIR>
          Path to the folder containing the surrealdb-migrate.toml config file
      --migrations-folder <MIGRATIONS_FOLDER>
          Path to the folder that contains the migration files
      --db-address <DB_ADDRESS>
          Address of the database server, e.g. "ws://localhost:8000"
  -h, --help
          Print help
  -V, --version
          Print version
```

In order to work properly `surmig` needs some configuration. See the chapter
[Configuration](#configuration) on how to configure `surmig`.

## `surrealdb-migrate`: the crate for Rust programs

Add the dependency to the `Cargo.toml` file of your project:

```toml
[dependencies]
surrealdb-migrate = "0.1"
```

Example on how to run migrations assuming they are stored in a `my_database/migrations` folder:

```rust ,no_run
use anyhow::Context;
use std::path::Path;
use surrealdb_migrate::config::{DbAuthLevel, DbClientConfig, RunnerConfig};
use surrealdb_migrate::db_client::{connect_to_database, DbConnection};
use surrealdb_migrate::runner::MigrationRunner;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    // Configure the database connection
    let db_config = DbClientConfig::default()
        .with_address("ws://localhost:8000")
        .with_namespace("playground")
        .with_database("examples")
        .with_auth_level(DbAuthLevel::Database)
        .with_username("example.user")
        .with_password("s3cr3t");

    let db = connect_to_database(&db_config)
        .await
        .context("failed to connect to examples database")?;

    // Instantiate the `MigrationRunner`
    let config = RunnerConfig::default()
        .with_migrations_folder(Path::new("my_database/migrations"));
    let runner = MigrationRunner::new(config.clone());

    // Run all forward (up) migrations
    runner.migrate(&db).await?;

    Ok(())
}
```

See the [API docs][docs-url] for more details on how to use this crate. A fully working example
can be found in the [examples](surrealdb-migrate/examples) folder of `surrealdb-migrate`.

## Features and functionality

Milestone 0.1 (first public release):

* [X] Read migrations from the filesystem
* [X] Store migration executions in the migrations table in the database
* [X] Create the migrations table if it does not exist
* [X] Apply migrations to a database
* [X] Verify order of migrations (optional: opt-out)
* [X] Verify checksum of applied migrations (optional: opt-out)
* [X] Revert migrations using "down"-scripts
* [X] Create new migration definitions in the migrations folder
* [X] Configure lib and CLI using environment variables
* [X] Configure lib and CLI using configuration file (TOML)
* [X] Command line application (CLI)

Planned features:

* [ ] CLI: Verify applied migrations against defined ones, to detect changed migrations and
  out-of-order migrations
* [ ] Traversing subfolders of the migrations-directory
* [ ] Optional `down`-subfolders for holding backward migrations
* [ ] Separated `up`- und `down`-subfolders for organizing forward- and backward-migrations
* [ ] Ignore configured filenames (pattern) when scanning the migrations-directory
* [ ] Dry run for migrate and revert
* [ ] Clean a database (remove all tables, indexes, relations, ...) (optional: opt-in)
* [ ] Additional command line options for most (maybe all) configuration settings

Further feature ideas:

* [ ] GitHub action for running `surrealdb-migrate` in CI/CD pipelines
* [ ] Docker container to run `surrealdb-migrate` as `initcontainer` for tools like Kubernetes
* [ ] Baseline of non-empty databases (or snapshots!?)
* [ ] Branching of databases for development
* [ ] Configure lib and CLI via a "hierarchy" of config-files (TOML) - workdir -> homedir -> appdir
* [ ] Templates for defining new migrations (provided ones and custom ones)

Non functional goals:

* [X] Excellent test coverage
* [X] Continues integration (CI) using GitHub Actions
* [ ] Good documentation of Lib on docs.rs
* [ ] Good documentation of CLI application in README
* [X] Complying with semantic versioning ([SemVer])
* [X] Documented Minimal Supported Rust Version (MSRV)

## Defining migrations

A migration is identified by a key and a title and whether it is a forward migration (up) or
a backward migration (down). For a complete migration definition we also need a migration script to
describe what has to be changed in the database.

The key of a migration is built from a date and a time, when the migration was created. A
migration script is any [SurrealQL] script.

Flat folder structure:

```text
migrations/
    20250102_142032_define_some_table.surql
    20250102_142032_define_some_table.down.surql
    20250102_142116_add_record_user_for_some_table.down.surql
    20250102_142116_add_record_user_for_some_table.up.surql
```

Separate up and down migrations: [planned]

```text
migrations/
    down/
        20250102_142032_define_some_table.surql
        20250102_142116_add_record_user_for_some_table.surql
    up/
        20250102_142032_define_some_table.surql
        20250102_142116_add_record_user_for_some_table.surql
```

## Applying migrations

### Order of migrations

Migrations are applied in the order of their keys (= timestamps). A migrations with an earlier
timestamp is applied before another migration with a later timestamp. If a migration with an earlier
timestamp is added after a migration with a later timestamp has been applied already, this is
considered an out-of-order migration.

SurrealDB-Migrate checks the order of migrations. By default, it does not migrate a databases if an
out-of-order migration is detected. This can be switched off by settings the configuration parameter
`ignore-order = true`, setting the environment variable `SURMIG_MIGRATION_IGNORE_ORDER=true` or
by specifying command line flag `--ignore-order`. (See [configuration](#configuration) for details.)

### Transactions

Each migration script is executed in one database transaction. This should prevent situations where
a failing migration script causes an inconsistent state of the database.

If a migration script fails and leaves the database in an inconsistent state it is up to the user
to revert the failed migration manually or by applying a down-script.

### Tracking the status of migrations

A migration is defined by:

* timestamp
* title
* kind (baseline/up/down)
* path to the script

The status of a migration is tracked by their execution:

* applied at
* applied by
* checksum
* execution time

SurrealDB-Migrate records executed migrations in a dedicated migrations-table in the database. The
default name of the migrations-table is `migrations`. The user can configure a custom name for this
migrations-table by settings the parameter `migrations-table = "schema_version"` or setting the
environment variable `SURMIG_DATABASE_MIGRATIONS_TABLE=schema_version`. If both, the parameter in
the configuration file and the environment variable, are set the value of the environment variable
overrides the value specified in the configuration file.

### Modified migrations

Before applying new migrations, SurrealDB-Migrate checks whether already applied migrations have
been changed. This is done by comparing the checksum of a migration in the migrations directory on
the filesystem with the checksum stored in the migrations table in the database for this migration
when it has been applied.

The checksum for the defined migration is calculated every time the 'migrate' operation is executed.
If this checksum does not match with the checksum stored when this checksum has been applied, the
'migrate' operation is aborted with an error message. The user can examine whether the migration
has changed accidentally or was modified on purpose and take actions to assure the database is in a
consistent state and remains consistent, when the new migrations are applied.

The check for changed migrations can be switched off by setting the parameter
`ignore-checksum = true` in the configuration file, by setting the environment variable
`SURMIG_MIGRATION_IGNORE_CHECKSUM=true` or specifying the command line flag `--ignore-checksum`.
(See [configuration](#configuration) for details.)

## Configuration

The lib as well as the cli application can be configured via a config file named
`surrealdb-migrate.toml` or via environment variables. Each setting has a default value. Any
setting can be overwritten with the value defined in the configuration file or via an environment
variable. The environment variables take precedence over the configuration file.

The CLI provides some command line options to further configure the applications. The CLI options
take the highest precedence and overwrite the related environment variables as well as the related
settings in the configuration file.

### Config file `surrealdb-migrate.toml`

This configuration file is read from the current working directory by
default. The default location of the configuration file can be set via the environment variable
`SURREALDB_MIGRATE_CONFIG_DIR`, e.g.

```dotenv
SURREALDB_MIGRATE_CONFIG_DIR=~/.config/surreal
```

The configuration file must be named `surrealdb-migrate.toml` and must be in the `TOML` format.

A complete list of configuration options can be found in the file
[
`surrealdb-migrate.default.toml`](surrealdb-migrate-config/resources/surrealdb-migrate.default.toml).
This file defines the default settings.

### Environment variables

A second option to configure the lib and the cli application is via environment variables. Each
environment variable overwrites a configuration value and takes precedence over the value defined in
a configuration file. For example:

```dotenv
SURMIG_MIGRATION_IGNORE_ORDER=true
SURMIG_DATABASE_ADDRESS=wss://localhost:9000
SURMIG_DATABASE_USERNAME=tester
SURMIG_DATABASE_PASSWORD=s3cr3t
```

The possible environment variables are listed in the file [
`default.env`](surrealdb-migrate-config/resources/default.env)

### Options of the command line tool

Options of the command line tool overwrite related settings of environment variables and in the
configuration file. There are options that are applicable for all subcommands and options that are
available only for a specific subcommand.

To get details about options that are available for all subcommands specify the `--help` option
without any subcommand like so:

```console
$ surmig --help 
```

To get details about available options for a specific subcommand specify the `--help` option after
the subcommand. For example:

```console
$ surmig migrate --help
```

<!-- Badges and related URLs --> 

[crates-badge]: https://img.shields.io/crates/v/surrealdb-migrate.svg

[crates-url]: https://crates.io/crates/surrealdb-migrate

[docs-badge]: https://docs.rs/surrealdb-migrate/badge.svg

[docs-url]: https://docs.rs/surrealdb-migrate

[license-badge]: https://img.shields.io/github/license/innoave/surrealdb-migrate?color=blue

[license-url]: https://github.com/innoave/surrealdb-migrate/blob/main/LICENSE

[msrv-badge]: https://img.shields.io/crates/msrv/surrealdb-migrate?color=chocolate

[code-coverage-badge]: https://codecov.io/github/innoave/surrealdb-migrate/graph/badge.svg?token=o0w7R7J0Op

[code-coverage-url]: https://codecov.io/github/innoave/surrealdb-migrate


<!-- External Links -->

[cli-crate-url]: https://crates.io/crates/surrealdb-migrate-cli

[SemVer]: https://semver.org

[SurrealDB]: https://surrealdb.com

[SurrealQL]: https://surrealdb.com/docs/surrealql
