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

