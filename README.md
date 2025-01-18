# SurrealDB Migrate

Evolve a [SurrealDB] database over time by applying migrations. `surrealdb-migrate` is a commandline
tool and lib to define and run migrations on the database. It provides version control for a
[SurrealDB] database in a project.

## Features and functionality

Milestone 0.1 (first public release):

* [X] Read migrations from the filesystem
* [X] Store migration executions in the migrations table in the database
* [X] Create the migrations table if it does not exist
* [X] Apply migrations to a database
* [X] Verify order of migrations (optional: opt-out)
* [X] Verify checksum of applied migrations (optional: opt-out)
* [ ] Revert migrations using "down"-scripts
* [ ] Create scaffold for defining migrations on the filesystem
* [X] Configure lib and CLI using environment variables
* [X] Configure lib and CLI using configuration file (TOML)
* [ ] Command line application (CLI)

Milestone 1.0:

* [ ] Configure lib and CLI via a "hierarchy" of config-files (TOML) - workdir -> homedir -> appdir
* [ ] Dry run for migrate and revert
* [ ] Clean a database (remove all tables, indexes, relations, ...) (optional: opt-in)
* [ ] Create new migration definitions in the migrations folder - templates!?
* [ ] Traversing subfolders of the `migrations` directory
* [ ] Support for baseline of non-empty databases (or snapshots!?)
* [ ] Support for branching of databases for development

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

Separate up and down migrations:

```
migrations/
    down/
        20250102_142032_define_some_table.surql
        20250102_142116_add_record_user_for_some_table.surql
    up/
        20250102_142032_define_some_table.surql
        20250102_142116_add_record_user_for_some_table.surql
```

## Tracking the status of migrations

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

## Transactions

Each migration script is executed in one database transaction.

If a migration script fails and leaves the database in an inconsistent state it is up to the user
to revert the failed migration manually or by applying a down-script.

## Configuration

The lib as well as the cli application can be configured via a config file named
`surrealdb-migrate.toml` or via environment variables. Each setting has a default value. Any
setting can be overwritten with the value defined in the configuration file or via an environment
variable. The environment variables take precedence over the configuration file.

### Config file `surrealdb-migrate.toml`

This configuration file is read from the current working directory by
default. The default location of the configuration file can be set via the environment variable
`SURREALDB_MIGRATE_CONFIG_DIR`, e.g.

```
SURREALDB_MIGRATE_CONFIG_DIR=~/.config/surreal
```

The configuration file must be named `surrealdb-migrate.toml` and must be in the `TOML` format.

A complete list of configuration options can be found in the file
[
`surrealdb-migrate.default.toml`](surrealdb-migrate-config/resources/surrealdb-migrate.default.toml).
This file defines the default settings.

### Environment variables

A second option to configure the lib and the cli application is via environment variables. The list
of possible environment variables. Each environment variable overwrites a configuration value and
takes precedence over the value defined in a configuration file. For example:

```dotenv
SURMIG_MIGRATION_IGNORE_ORDER=true
SURMIG_DATABASE_ADDRESS=wss://localhost:9000
SURMIG_DATABASE_USERNAME=tester
SURMIG_DATABASE_PASSWORD=s3cr3t
```

The possible environment variables are listed in the file [
`default.env`](surrealdb-migrate-config/resources/default.env)

[SurrealDB]: https://surrealdb.com

[SurrealQL]: https://surrealdb.com/docs/surrealql
