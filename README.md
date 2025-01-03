# SurrealDB Migrate

Evolve a [SurrealDB] Database over time by applying migrations. `surrealdb-migrate` is a commandline
tool and lib to define and run migrations on the database in your project.

## Defining migrations

A migration is identified by a timestamp and a title and whether it is a forward migration (up) or
a backward migration (down). To make the definition of a migration complete we write some SurQL that
describes what is to be changed.

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

* timestamp
* title
* direction (up/down)
* script name
* applied at
* execution time
* success (yes/no)


[SurrealDB]: https://surrealdb.com
