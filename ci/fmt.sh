#!/usr/bin/env bash

set -e

cargo fmt --all --check
cargo sort --workspace --check --check-format

cd fuzz

cargo fmt --check
cargo sort --check --check-format
