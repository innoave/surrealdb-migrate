# SurrealDB Migrate

Evolve a [SurrealDB] database over time by applying migrations. `surrealdb-migrate` is a commandline
tool and lib to define and run migrations on the database. It provides version control for a
[SurrealDB] database in a project.

## Defining migrations

A migration is identified by a timestamp and a title and whether it is a forward migration (up) or
a backward migration (down). To make the definition of a migration complete we write some
[SurrealQL] queries that describe what is to be changed in the database.

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
* execution time
* success (yes/no)

[SurrealDB]: https://surrealdb.com

[SurrealQL]: https://surrealdb.com/docs/surrealql
