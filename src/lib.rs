pub mod checksum;
pub mod config;
pub mod db;
pub mod definition;
mod error;
pub mod io;
pub mod migration;

#[cfg(any(test, feature = "proptest-support"))]
pub mod proptest_support;

pub use error::Error;

// workaround for false positive 'unused extern crate' warnings until
// Rust issue [#95513](https://github.com/rust-lang/rust/issues/95513) is fixed
#[cfg(test)]
mod dummy_extern_uses {
    use dotenvy as _;
    use tokio as _;
}
