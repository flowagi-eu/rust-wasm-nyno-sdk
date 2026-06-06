use rmpv::Value;
use plugin_sdk::{NynoPlugin, export_plugin};
use scraper::{Html, Selector};
use std::panic::{catch_unwind, AssertUnwindSafe};
use html2md::parse_html;

#[derive(Default)]
pub struct NynoHtmlToMarkdown;

impl NynoPlugin for NynoHtmlToMarkdown {
    fn run(&self, args: Value, context: &mut Value) -> i32 {
        let result = catch_unwind(AssertUnwindSafe(|| {
            self.run_inner(args, context)
        }));

        if result.is_err() {
            set_error(context, "prev", "panic caught in WASM");
            return 1;
        }

        result.unwrap_or(1)
    }
}

impl NynoHtmlToMarkdown {
    fn run_inner(&self, args: Value, context: &mut Value) -> i32 {
        let set_name = get_set_name(context);

        // ----------------------------
        // Extract args
        // ----------------------------
        let args_vec = match args {
            Value::Array(v) => v,
            _ => vec![],
        };

        if args_vec.is_empty() {
            set_error(context, &set_name, "No HTML content provided in args[0]");
            return 1;
        }

        let input_data = &args_vec[0];

        // ----------------------------
        // Normalize input
        // ----------------------------
        let html_list: Vec<String> = match input_data {
            Value::Array(arr) => arr
                .iter()
                .map(|v| v.as_str().unwrap_or("").to_string())
                .collect(),
            Value::String(s) => vec![s.as_str().unwrap_or("").to_string()],
            _ => vec![],
        };

        // ----------------------------
        // Process function
        // ----------------------------
        fn process_html(html: &str) -> Value {
            let document = Html::parse_document(html);

            // ---- Extract front matter ----
            let mut front_matter: Vec<(Value, Value)> = Vec::new();

            // Title
            if let Ok(title_sel) = Selector::parse("title") {
                if let Some(el) = document.select(&title_sel).next() {
                    let title = el.text().collect::<Vec<_>>().join("").trim().to_string();
                    if !title.is_empty() {
                        front_matter.push((
                            Value::String("title".into()),
                            Value::String(title.into()),
                        ));
                    }
                }
            }

            // Meta tags
            if let Ok(meta_sel) = Selector::parse("meta") {
                for el in document.select(&meta_sel) {
                    let name = el.value().attr("name")
                        .or_else(|| el.value().attr("property"));
                    let content = el.value().attr("content");

                    if let (Some(n), Some(c)) = (name, content) {
                        front_matter.push((
                            Value::String(n.into()),
                            Value::String(c.into()),
                        ));
                    }
                }
            }

            // ---- HARD FILTERING ----
            let mut cleaned_html = html.to_string();

            for tag in ["script", "style", "noscript", "template", "footer"] {
                cleaned_html = remove_tag(&cleaned_html, tag);
            }

            let cleaned_doc = Html::parse_document(&cleaned_html);

            // ---- Pick real content ----
            let content_selectors = [
                "main",
                "article",
                r#"[role="main"]"#,
                "body",
            ];

            let mut html_content = cleaned_html.clone();

            for sel_str in content_selectors {
                if let Ok(sel) = Selector::parse(sel_str) {
                    if let Some(el) = cleaned_doc.select(&sel).next() {
                        html_content = el.html();
                        break;
                    }
                }
            }

            // ---- Convert to Markdown ----
            let raw_md = parse_html(&html_content);
            let markdown = normalize_headings(&raw_md);

            Value::Map(vec![
                (
                    Value::String("frontMatter".into()),
                    Value::Map(front_matter),
                ),
                (
                    Value::String("markdown".into()),
                    Value::String(markdown.into()),
                ),
            ])
        }

        // ----------------------------
        // Run processing
        // ----------------------------
        let results: Vec<Value> = html_list
            .iter()
            .map(|html| process_html(html))
            .collect();

        // ----------------------------
        // Store result
        // ----------------------------
        if let Value::Map(map) = context {
            if results.len() == 1 {
                if let Value::Map(obj) = &results[0] {
                    let mut front = None;
                    let mut md = None;

                    for (k, v) in obj {
                        if let Value::String(key) = k {
                            match key.as_str().unwrap_or("") {
                                "frontMatter" => front = Some(v.clone()),
                                "markdown" => md = Some(v.clone()),
                                _ => {}
                            }
                        }
                    }

                    if let Some(f) = front {
                        map.push((
                            Value::String(format!("{}_meta", set_name).into()),
                            Value::Map(vec![(
                                Value::String("frontMatter".into()),
                                f,
                            )]),
                        ));
                    }

                    if let Some(m) = md {
                        map.push((
                            Value::String(set_name.clone().into()),
                            m,
                        ));
                    }
                }
            } else {
                map.push((
                    Value::String(set_name.clone().into()),
                    Value::Array(results),
                ));
            }
        }

        0
    }
}

// ----------------------------
// Heading normalization
// ----------------------------
fn normalize_headings(md: &str) -> String {
    let mut lines = md.lines().peekable();
    let mut out = Vec::new();

    while let Some(line) = lines.next() {
        let next = lines.peek().copied();

        // Setext H1
        if let Some(next_line) = next {
            if next_line.trim().chars().all(|c| c == '=') {
                out.push(format!("# {}", line.trim()));
                lines.next();
                continue;
            }

            // Setext H2
            if next_line.trim().chars().all(|c| c == '-') {
                out.push(format!("## {}", line.trim()));
                lines.next();
                continue;
            }
        }

        // Clean ### title ###
        let cleaned = line
            .trim()
            .trim_end_matches('#')
            .trim_end()
            .to_string();

        out.push(cleaned);
    }

    out.join("\n")
}

// ----------------------------
// Helpers
// ----------------------------
fn remove_tag(html: &str, tag: &str) -> String {
    let mut result = html.to_string();

    loop {
        let open = format!("<{}", tag);
        let close = format!("</{}>", tag);

        if let Some(start) = result.find(&open) {
            if let Some(end) = result[start..].find(&close) {
                let end_idx = start + end + close.len();
                result.replace_range(start..end_idx, "");
            } else {
                break;
            }
        } else {
            break;
        }
    }

    result
}

fn get_set_name(context: &Value) -> String {
    if let Value::Map(map) = context {
        map.iter().find_map(|(k, v)| {
            if let (Value::String(key), Value::String(val)) = (k, v) {
                if key.as_str().unwrap_or("") == "set_context" {
                    return Some(val.as_str().unwrap_or("prev").to_string());
                }
            }
            None
        }).unwrap_or("prev".to_string())
    } else {
        "prev".to_string()
    }
}

fn set_error(context: &mut Value, set_name: &str, msg: &str) {
    if let Value::Map(map) = context {
        map.push((
            Value::String(format!("{}.error", set_name).into()),
            Value::String(msg.into()),
        ));
    }
}

// Export
export_plugin!(NynoHtmlToMarkdown);
