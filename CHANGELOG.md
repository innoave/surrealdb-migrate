# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Common Changelog](https://common-changelog.org/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

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
