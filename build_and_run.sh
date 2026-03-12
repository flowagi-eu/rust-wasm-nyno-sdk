#!/bin/bash
set -e

# Build plugin_sdk (rlib)
cd plugin_sdk
cargo build --release
cd ..

# Build example-plugin (WASM)
cd example-plugin
cargo build --release --target wasm32-unknown-unknown
mkdir -p ../build
cp target/wasm32-unknown-unknown/release/plugin.wasm ../build/rust_plugin.wasm

echo "Build complete: build/rust_plugin.wasm"

cd ..

node run.js
