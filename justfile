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
    cargo test

# run the lib tests only
test-lib:
    cargo test --lib

# run code coverage (does not include doc-tests)
test-coverage:
    cargo +nightly llvm-cov --branch --html --open --ignore-filename-regex "tests|test_dsl"

# build the crate for release
build-release:
    cargo build --release --workspace --all-features

# clean the workspace
clean:
    cargo clean

# generate and open docs locally
docl:
    cargo doc --workspace --all-features --open
