pub mod db;

pub fn load_environment_variables() {
    dotenvy::dotenv().expect("failed to load .env file");
}

// workaround for false positive 'unused extern crate' warnings until
// Rust issue [#95513](https://github.com/rust-lang/rust/issues/95513) is fixed
mod dummy_extern_uses {
    use chrono as _;
    use crc32fast as _;
    use indexmap as _;
    use proptest as _;
    use serde as _;
    use serde_with as _;
    use surrealdb as _;
    use thiserror as _;
}
