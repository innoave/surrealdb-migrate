// workaround for false positive 'unused extern crate' warnings until
// Rust issue [#95513](https://github.com/rust-lang/rust/issues/95513) is fixed
#[cfg(test)]
mod dummy_extern_uses {
    use asserting as _;
    use chrono as _;
    use crc32fast as _;
    use database_migration as _;
    use enumset as _;
    use indexmap as _;
    use proptest as _;
    use regex as _;
    use serde as _;
    use serde_with as _;
    use thiserror as _;
}

#[test]
fn test_readme_deps() {
    version_sync::assert_markdown_deps_updated!("README.md");
}

#[test]
fn test_html_root_url() {
    version_sync::assert_html_root_url_updated!("src/lib.rs");
}
