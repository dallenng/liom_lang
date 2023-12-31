#!/usr/bin/env sh

set -eux

cd crates/fuzz

cargo fuzz run main -- -max_total_time=1800
