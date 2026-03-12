## Example Code:

```
// cat example-plugin/src/lib.rs
use serde_json::{Value, json};
use plugin_sdk::{Plugin, export_plugin};

#[derive(Default)]
pub struct SimpleRustPlugin;

impl Plugin for SimpleRustPlugin {
    fn run(&self, args: Vec<Value>, context: Value) -> Value {
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
        } else { false };

        json!({"args": args, "context": context, "result": result})
    }
}

export_plugin!(SimpleRustPlugin);

```

# Rust to WASM
Simple Rust to WASM SDK (v0.3) that works well with NodeJS/Bun.

Goal: One simple safe fast interface for creating WASM (created by Rust) in NodeJS/Bun backends/engines. In our case for Nyno Workflows.

Status: Experimental (awaiting community feedback) 

## Usage:
```
bash build_and_run.sh 
```

Should output something like:
```
    Finished `release` profile [optimized] target(s) in 0.02s
[DEBUG] Input JSON: {"args":[10,"lower than",5],"context":{"set_context":"prev"}}
[DEBUG] Output offset: 93 length: 76
[DEBUG] Output JSON string: {"args":[10,"lower than",5],"context":{"set_context":"prev"},"result":false}
Rust plugin eval 1: {
  args: [ 10, 'lower than', 5 ],
  context: { set_context: 'prev' },
  result: false
}
[DEBUG] Input JSON: {"args":[7,"higher than",3],"context":{"set_context":"prev"}}
[DEBUG] Output offset: 93 length: 75
[DEBUG] Output JSON string: {"args":[7,"higher than",3],"context":{"set_context":"prev"},"result":true}
Rust plugin eval 2: {
  args: [ 7, 'higher than', 3 ],
  context: { set_context: 'prev' },
  result: true
}
```
