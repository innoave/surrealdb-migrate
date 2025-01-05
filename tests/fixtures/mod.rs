pub mod db;

// workaround for false positive 'unused extern crate' warnings until
// Rust issue [#95513](https://github.com/rust-lang/rust/issues/95513) is fixed
mod dummy_extern_uses {
    use crc32fast as _;
    use proptest as _;
    use surrealdb as _;
    use thiserror as _;
    use time as _;
}
