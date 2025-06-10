# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Common Changelog](https://common-changelog.org/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.2.0 - 2025-06-10

_Subdirectories in the migration folder_

### Added

* BREAKING: ignore configured filenames when scanning migration definitions
  [(PR #16)](https://github.com/innoave/surrealdb-migrate/pull/16)
* Verify subcommand in CLI-app
  [(PR #3)](https://github.com/innoave/surrealdb-migrate/pull/3)
* Scan directory-tree for migrations starting from the migrations-folder including any
  subdirectories
  [(PR #20)](https://github.com/innoave/surrealdb-migrate/pull/20)

### Changes

* BREAKING: remove definition key from `Problem::OutOfOrder` as it is redundant to
  `ProblematicMigration::key`
  [(PR #4)](https://github.com/innoave/surrealdb-migrate/pull/4)
* BREAKING: migrate to Rust edition 2024 - this changes MSRV to 1.86.0
  [(PR #15)](https://github.com/innoave/surrealdb-migrate/pull/15)
* upgrade dependency cli-table to version 0.5
  [(PR #12)](https://github.com/innoave/surrealdb-migrate/pull/12)
* upgrade SurrealDB to version 2.3
  [(PR #13)](https://github.com/innoave/surrealdb-migrate/pull/13)

### Development

* Switch to fluent assertions crate `asserting`
  [(PR #17)](https://github.com/innoave/surrealdb-migrate/pull/17)
* Use `fakeenv` crate for testing functionality that depends on environment variables
  [(PR #18)](https://github.com/innoave/surrealdb-migrate/pull/18)

## 0.1.0 - 2025-02-08

_First release_

### Added

* Read migrations from the filesystem
* Store migration executions in the migrations table in the database
* Create the migrations table if it does not exist
* Apply migrations to a database
* Verify order of migrations (optional: opt-out)
* Verify checksum of applied migrations (optional: opt-out)
* Revert migrations using "down"-scripts
* Create new migration definitions in the migrations folder
* Configure lib and CLI using environment variables
* Configure lib and CLI using configuration file (TOML)
* Command line application (CLI)
