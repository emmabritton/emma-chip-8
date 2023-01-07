#!/usr/bin/env bash

set -e

cargo build -q --release --bin ec8-logging --features="ec8-core/logging"
cargo build -q --release --bin ec8