## Example Code (Nyno Plugin):

```
use serde_json::{Value, json};
use plugin_sdk::{Plugin, export_plugin};

#[derive(Default)]
pub struct SimpleRustPlugin;

impl Plugin for SimpleRustPlugin {
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

export_plugin!(SimpleRustPlugin);
```

# Rust to WASM
Simple Rust to WASM SDK (v0.4) that works well with NodeJS/Bun.

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
