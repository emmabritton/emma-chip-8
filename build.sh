#!/usr/bin/env bash

set -e

mkdir -p build/bin

cd ec8-client
./build.sh
cd ..
cargo build --release -q --manifest-path ec8-assembler/Cargo.toml

mv ec8-client/target/release/ec8 build/bin
mv ec8-client/target/release/ec8-logging build/bin
mv ec8-assembler/target/release/ec8-assembler build/bin