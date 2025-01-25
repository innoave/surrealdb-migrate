#!/usr/bin/env just --justfile

set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

alias b := build
alias c := check
alias l := lint
alias t := test
alias tc := test-coverage
alias tl := test-lib

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
    cargo clippy --workspace --all-targets --all-features

# run all tests
test:
    cargo test --all-features

# run the lib tests only
test-lib:
    cargo test --lib --all-features

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
test-coverage:
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
docl:
    cargo doc --workspace --all-features --open
