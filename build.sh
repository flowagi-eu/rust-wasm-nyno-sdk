#!/bin/bash
set -e

# Build plugin_sdk (rlib)
cd plugin_sdk
cargo build --release
cd ..

# Build example-plugin (WASM)
cd example-plugin-markdown
cargo build --release --target wasm32-unknown-unknown
mkdir -p ../build
echo "Build complete"
