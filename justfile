#!/usr/bin/env just --justfile

set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

alias b := build
alias c := check
alias l := lint
alias t := test

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

# build the crate for release
build-release:
    cargo build --release --all-features
