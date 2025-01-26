pub mod db;

use snapbox::cmd::{cargo_bin, Command};

pub fn load_environment_variables() {
    let _env_file =
        dotenvy::from_filename("test.env").expect("failed to load environment variables");
}

pub fn surmig() -> Command {
    Command::new(cargo_bin!("surmig"))
}

// workaround for false positive 'unused extern crate' warnings until
// Rust issue [#95513](https://github.com/rust-lang/rust/issues/95513) is fixed
#[cfg(test)]
mod dummy_extern_uses {
    use assert_fs as _;
    use clap as _;
    use cli_table as _;
    use color_eyre as _;
    use database_migration as _;
    use surrealdb_migrate as _;
    use surrealdb_migrate_db_client as _;
    use tokio as _;
    use trycmd as _;
}
