# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Common Changelog](https://common-changelog.org/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## 0.2.0 - 2025-06-10

_Subdirectories in the migration folder_

### Added

* BREAKING: ignore configured filenames when scanning migration definitions
  [(PR #16)](https://github.com/innoave/surrealdb-migrate/pull/16)
* Scan directory-tree for migrations starting from the migrations-folder including any
  subdirectories
  [(PR #20)](https://github.com/innoave/surrealdb-migrate/pull/20)

### Changes

* BREAKING: migrate to Rust edition 2024 - this changes MSRV to 1.86.0
  [(PR #15)](https://github.com/innoave/surrealdb-migrate/pull/15)

### Development

* Switch to fluent assertions crate `asserting`
  [(PR #17)](https://github.com/innoave/surrealdb-migrate/pull/17)

## 0.1.0 - 2025-02-08

_First release_
