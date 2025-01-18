pub mod runner;

// workaround for false positive 'unused extern crate' warnings until
// Rust issue [#95513](https://github.com/rust-lang/rust/issues/95513) is fixed
#[cfg(test)]
mod dummy_extern_uses {
    use color_eyre as _;
    use dotenvy as _;
    use testcontainers_modules as _;
    use tokio as _;
}
