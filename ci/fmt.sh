#!/usr/bin/env sh

set -eux

cargo fmt --all --check
cargo sort --workspace --check --check-format
