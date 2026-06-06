# For WasmHub: Open-Source .wasm Projects. Run with JS. Contribute with Rust.


```rust
use plugin_sdk::{NynoPlugin, export_plugin};
use rmpv::Value;

#[derive(Default)]
pub struct HelloPlugin;

impl NynoPlugin for HelloPlugin {
    fn run(&self, _args: Value, context: &mut Value) -> i32 {
        if let Value::Map(map) = context {
            map.push((
                Value::String("prev".into()),
                Value::String("Hello from Rust!".into()),
            ));
        }

        0
    }
}

export_plugin!(HelloPlugin);
```

# Rust to WASM for Nyno Workflows
WasmHub/Nyno Plugin SDK (v1) for producing WASM that works well with NodeJS/Bun.

Goal: One simple safe fast interface for creating WASM (created by Rust) in NodeJS/Bun backends/engines. In our case for [Nyno Workflows](https://nyno.dev).

## Usage:
```
bash build.sh

## Run:
- see runWasm.js for JS glue.
