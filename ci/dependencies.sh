#!/usr/bin/env bash

set -e

cargo outdated --workspace --exit-code 1

cd fuzz

cargo outdated --exit-code 1
