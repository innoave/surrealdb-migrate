use assert_cmd::Command;

pub fn surmig() -> Command {
    Command::cargo_bin("surmig").expect("surmig command not found")
}

// workaround for false positive 'unused extern crate' warnings until
// Rust issue [#95513](https://github.com/rust-lang/rust/issues/95513) is fixed
#[cfg(test)]
mod dummy_extern_uses {
    use assert_fs as _;
    use clap as _;
    use color_eyre as _;
    use database_migration as _;
    use surrealdb_migrate as _;
    use tokio as _;
    use trycmd as _;
}
