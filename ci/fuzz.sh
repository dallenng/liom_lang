#!/usr/bin/env bash

set -e

cd fuzz

cargo fuzz build
cargo clippy --all-targets --all-features -- -D warnings
cargo clippy --all-targets --all-features -- -W clippy::pedantic
cargo fuzz run main -- -max_total_time=1800
