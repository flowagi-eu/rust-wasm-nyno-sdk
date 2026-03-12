#[allow(unused_imports)]
use serde::Serialize;
#[allow(unused_imports)]
use serde_json::{Value, json};

// Trait that all plugins must implement. 
pub trait Plugin {
    fn run(&self, args: Vec<Value>, context: Value) -> Value;
}

// Macro to export a plugin as a WASM-compatible function.
#[macro_export]
macro_rules! export_plugin {
    ($plugin:ty) => {
        #[unsafe(no_mangle)]
        pub extern "C" fn run(ptr: u32, len: u32, out_ptr: u32) {
            // Convert the incoming pointer/length from JS into a Rust byte slice
            let input_bytes = unsafe { std::slice::from_raw_parts(ptr as *const u8, len as usize) };

            // Parse the input as JSON
            let input: Value = match serde_json::from_slice(input_bytes) {
                Ok(v) => v,
                Err(_) => return $crate::write_output(out_ptr as usize, json!({"error":"invalid JSON"})),
            };

            // Extract args/context
            let args = input.get("args").and_then(|v| v.as_array().cloned()).unwrap_or_default();
            let context = input.get("context").cloned().unwrap_or(json!({}));

            // Instantiate the plugin
            let plugin = <$plugin>::default();

            // Run plugin
            let output = plugin.run(args, context);

            // Write output back (safe bounds checking optional in JS)
            $crate::write_output(out_ptr as usize, output);
        }
    };
}

// Safe write_output without js_sys
pub fn write_output(out_ptr: usize, value: impl Serialize) {
    let bytes = match serde_json::to_vec(&value) {
        Ok(b) => b,
        Err(_) => return, // skip if serialization fails
    };
    let offset = out_ptr + 16;
    let len = bytes.len();

    unsafe {
        // Copy bytes
        let mem = std::slice::from_raw_parts_mut(offset as *mut u8, len);
        mem.copy_from_slice(&bytes);

        // Write metadata
        let out_mem = std::slice::from_raw_parts_mut(out_ptr as *mut u8, 8);
        out_mem[0..4].copy_from_slice(&(offset as u32).to_le_bytes());
        out_mem[4..8].copy_from_slice(&(len as u32).to_le_bytes());
    }
}
