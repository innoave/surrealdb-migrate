#!/usr/bin/env just --justfile

set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

alias b := build
alias c := check
alias cc := code-coverage
alias l := lint
alias la := lint-all-features
alias ld := lint-default
alias t := test
alias ta := test-all-features
alias td := test-default
alias tl := test-lib
alias tla := test-lib-all-features
alias tld := test-lib-default

# list recipies
default:
    just --list

# build the crate for debugging
build:
    cargo build --workspace --all-features

# check syntax in all targets
check:
    cargo check --workspace --all-targets --all-features

# linting code using Clippy
lint:
    just lint-default
    just lint-all-features

# linting code using Clippy with default features
lint-default:
    cargo clippy --workspace --all-targets

# linting code using Clippy with all features enabled
lint-all-features:
    cargo clippy --workspace --all-targets --all-features

# run all tests
test:
    just test-default
    just test-all-features

# run all tests with default features
test-default:
    cargo test

# run all tests for all features
test-all-features:
    cargo test --all-features

# run unit tests only
test-lib:
    just test-lib-default
    just test-lib-all-features

# run unit tests with default features
test-lib-default:
    cargo test --lib --bins

# run unit tests for all features
test-lib-all-features:
    cargo test --all-features --lib --bins

# run the cli-tests only
test-cli:
    cargo test --package surrealdb-migrate-cli --all-features

# run tests for the `surrealdb-migrate-db-client` package
test-db-client:
    cargo test --package surrealdb-migrate-db-client --all-features

# run tests for the `surrealdb-migrate` package
test-migrate:
    cargo test --package surrealdb-migrate --all-features

# run code coverage (does not include doc-tests)
code-coverage:
    cargo +nightly llvm-cov clean --workspace
    cargo +nightly llvm-cov --branch --all-features --no-report
    cargo +nightly llvm-cov report --html --open --ignore-filename-regex "tests|test_dsl"

# build the crate for release
build-release:
    cargo build --release --workspace

# clean the workspace
clean:
    cargo clean

# generate and open docs locally
doc:
    cargo doc --workspace --all-features --no-deps --open
