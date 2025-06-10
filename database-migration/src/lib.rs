#![doc(html_root_url = "https://docs.rs/database-migration/0.2.0")]

pub mod action;
pub mod checksum;
pub mod config;
pub mod definition;
pub mod error;
pub mod migration;
pub mod repository;
pub mod result;

#[cfg(any(test, feature = "proptest-support"))]
pub mod proptest_support;

#[cfg(any(test, feature = "test-dsl"))]
pub mod test_dsl;

// workaround for false positive 'unused extern crate' warnings until
// Rust issue [#95513](https://github.com/rust-lang/rust/issues/95513) is fixed
#[cfg(test)]
mod dummy_extern_uses {
    use version_sync as _;
}
