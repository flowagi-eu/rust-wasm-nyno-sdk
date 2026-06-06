#!/bin/bash
set -e

# Build plugin_sdk (rlib)
cd ../plugin_sdk
cargo build --release


# Build example-plugin (WASM)
cd ../example-plugin2-html-to-markdown
cargo build --release --target wasm32-unknown-unknown
