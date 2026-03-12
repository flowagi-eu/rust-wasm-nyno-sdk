// cat example-plugin2/src/lib.rs
use serde_json::{Value, json};
use plugin_sdk::{Plugin, export_plugin};

#[derive(Default)]
pub struct SortKVPlugin;

impl Plugin for SortKVPlugin {
    fn run(&self, args: Vec<Value>, context: &mut Value) -> i32 {
        // Determine key to store result in context
        let set_name = context
            .get("set_context")
            .and_then(|v| v.as_str())
            .unwrap_or("prev")
            .to_string();

        // Compute sorted entries
        let result = (|| -> Result<Vec<Value>, &str> {
            // args[0] = object to sort
            let obj = args.get(0).ok_or("args[0] must be an object")?;
            let obj_map = obj.as_object().ok_or("args[0] must be an object")?;

            // args[1] = "asc" or "desc"
            let order = args
                .get(1)
                .and_then(|v| v.as_str())
                .unwrap_or("asc")
                .to_lowercase();

            // Convert object → Vec<(key, value)>
            let mut entries: Vec<(String, f64)> = obj_map
                .iter()
                .map(|(k, v)| (k.clone(), v.as_f64().unwrap_or(0.0)))
                .collect();

            // Sort
            if order == "desc" {
                entries.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
            } else {
                entries.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            }

            // Convert to Vec<Value> of [key, value]
            Ok(entries
                .into_iter()
                .map(|(k, v)| json!([k, v]))
                .collect())
        })();

        match result {
            Ok(sorted) => {
                context[&set_name] = json!(sorted);
                0 // success
            }
            Err(err) => {
                context[format!("{}.error", set_name)] = json!({"error": err});
                1 // error
            }
        }
    }
}

export_plugin!(SortKVPlugin);
