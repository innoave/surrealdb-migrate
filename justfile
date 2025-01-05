#!/usr/bin/env just --justfile

set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

alias b := build
alias c := check
alias l := lint
alias t := test
alias tl := test-lib
alias db := start-test-db

# list recipies
default:
    just --list

# build the crate for debugging
build:
    cargo build --all-features

# check syntax in all targets
check:
    cargo check --all-targets --all-features

# linting code using Clippy
lint:
    cargo clippy --all-targets --all-features

# run the tests
test:
    cargo test --all-features

# run the lib tests only
test-lib:
    cargo test --lib --all-features

# build the crate for release
build-release:
    cargo build --release --all-features

# clean the workspace
clean:
    cargo clean

# start the test DB in Docker container
start-test-db:
    docker compose up
