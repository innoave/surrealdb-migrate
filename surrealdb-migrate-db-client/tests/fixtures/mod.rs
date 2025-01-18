pub mod db;

pub fn load_environment_variables() {
    dotenvy::dotenv().expect("failed to load .env file");
}

// workaround for false positive 'unused extern crate' warnings until
// Rust issue [#95513](https://github.com/rust-lang/rust/issues/95513) is fixed
#[cfg(test)]
mod dummy_extern_uses {
    use chrono as _;
    use serde as _;
    use surrealdb as _;
}
