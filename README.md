## Example Code (for nyno-if node):

```
use serde_json::{Value, json};
use plugin_sdk::{NynoPlugin, export_plugin};

#[derive(Default)]
pub struct NynoIf;

impl NynoPlugin for NynoIf {
    fn run(&self, args: Vec<Value>, context: &mut Value) -> i32 {
        let set_name = context.get("set_context").and_then(|v| v.as_str()).unwrap_or("prev").to_string();
        let result = if args.len() >= 3 {
            let left = args[0].as_f64().unwrap_or(0.0);
            let cond = args[1].as_str().unwrap_or("");
            let right = args[2].as_f64().unwrap_or(0.0);
            match cond {
                "lower than" => left < right,
                "higher than" => left > right,
                "equal to" => (left - right).abs() < 1e-9,
                _ => false,
            }
        } else {
            false
        };

        context[set_name] = json!(result);

        // Return 0 (left next node) or 1 (right next node)
        if result { 1 } else { 0 }
    }
}

export_plugin!(NynoIf);
```

# Rust to WASM for Nyno Workflows
Simple Rust to Nyno Plugin SDK (v3) for producing WASM that works well with NodeJS/Bun.

Goal: One simple safe fast interface for creating WASM (created by Rust) in NodeJS/Bun backends/engines. In our case for Nyno Workflows.

Status: Experimental (awaiting community feedback) 

## Usage:
```
bash build_and_run.sh 
```

Should output something like:
```
    Finished `release` profile [optimized] target(s) in 0.02s
    Finished `release` profile [optimized] target(s) in 0.02s
Build complete: build/rust_plugin.wasm
    Finished `release` profile [optimized] target(s) in 0.02s
Build complete: build/rust_plugin2.wasm
Debug: input JSON bytes length = 61 outPtr = 61
Debug: output offset = 77 length = 39
Debug: raw output string = [0,{"prev":false,"set_context":"prev"}]
NynoIf Plugin eval 1: [ 0, { prev: false, set_context: 'prev' } ]
Debug: input JSON bytes length = 61 outPtr = 61
Debug: output offset = 77 length = 38
Debug: raw output string = [1,{"prev":true,"set_context":"prev"}]
NynoIf Plugin eval 2: [ 1, { prev: true, set_context: 'prev' } ]
Debug: input JSON bytes length = 72 outPtr = 72
Debug: output offset = 88 length = 69
Debug: raw output string = [0,{"set_context":"sorted","sorted":[["c",9.0],["a",5.0],["b",2.0]]}]
NynoSortKv Plugin eval 3: [ 0, { set_context: 'sorted', sorted: [ [Array], [Array], [Array] ] } ]
```
