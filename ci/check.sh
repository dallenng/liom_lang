#!/usr/bin/env bash

set -e

cargo build --workspace --all-targets --all-features
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo clippy --workspace --all-targets --all-features -- -W clippy::pedantic
cargo test --workspace --all-targets --all-features
