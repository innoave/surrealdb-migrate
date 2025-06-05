// workaround for false positive 'unused extern crate' warnings until
// Rust issue [#95513](https://github.com/rust-lang/rust/issues/95513) is fixed
#[cfg(test)]
mod dummy_extern_uses {
    use anyhow as _;
    use assert_fs as _;
    use asserting as _;
    use chrono as _;
    use color_eyre as _;
    use database_migration as _;
    use database_migration_files as _;
    use dotenvy as _;
    use indexmap as _;
    use log as _;
    use surrealdb_migrate as _;
    #[cfg(feature = "config")]
    use surrealdb_migrate_config as _;
    use surrealdb_migrate_db_client as _;
    use testcontainers_modules as _;
    use tokio as _;
}

#[test]
fn test_readme_deps() {
    version_sync::assert_markdown_deps_updated!("../README.md");
}

#[test]
fn test_html_root_url() {
    version_sync::assert_html_root_url_updated!("src/lib.rs");
}
