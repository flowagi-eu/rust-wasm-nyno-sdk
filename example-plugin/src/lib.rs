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
