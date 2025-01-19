mod args;

use crate::args::Args;
use clap::Parser;
use color_eyre::Report;

// dependency is needed to re-export the features of the `surrealdb` crate
use surrealdb_migrate as _;

#[tokio::main]
async fn main() -> Result<(), Report> {
    color_eyre::install()?;

    let _args = Args::parse();

    Ok(())
}

// workaround for false positive 'unused extern crate' warnings until
// Rust issue [#95513](https://github.com/rust-lang/rust/issues/95513) is fixed
#[cfg(test)]
mod dummy_extern_uses {
    use assert_cmd as _;
    use assert_fs as _;
    use trycmd as _;
}
