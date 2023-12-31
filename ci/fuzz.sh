#!/usr/bin/env sh

set -eux

cargo fuzz run main -- -max_total_time=1800
