#!/usr/bin/env bash

set -e

RUSTFLAGS="-D warnings" cargo build -q
cargo test -q
cargo clippy -q --all -- -D warnings
cargo fmt -q  -- --check