# SurrealDB Migrate

Evolve a [SurrealDB] database over time by applying migrations. `surrealdb-migrate` is a commandline
tool and lib to define and run migrations on the database. It provides version control for a
[SurrealDB] database in a project.

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
* direction (up/down)
* path to the script

The status of a migration is tracked by their execution:

* applied at
* checksum
* execution time
* success (yes/no)

## Transactions

Each migration script is executed in one database transaction.

If a migration script fails and leaves the database in an inconsistent state it is up to the user
to revert the failed migration manually or by applying a down-script.

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
* [ ] Configure lib and CLI using environment variables
* [ ] Command line application (CLI)

Milestone 1.0:

* [ ] Configure lib and CLI via a "hierarchy" of config-files (TOML) - workdir -> homedir -> appdir
* [ ] Dry run for migrate and revert
* [ ] Clean a database (remove all tables, indexes, relations, ...) (optional: opt-in)
* [ ] Create new migration definitions in the migrations folder - templates!?
* [ ] Traversing subfolders of the `migrations` directory
* [ ] Support for baseline of non-empty databases (or snapshots!?)
* [ ] Support for branching of databases for development

[SurrealDB]: https://surrealdb.com

[SurrealQL]: https://surrealdb.com/docs/surrealql
