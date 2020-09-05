use pulldown_cmark::{html, Options, Parser};
use serde_json::value::to_value;
use std::collections::HashMap;
use tera::{Result, Value};

pub fn mark_down(input: &str) -> String {
    let parser = Parser::new_ext(input, Options::empty());

    // Write to String buffer.
    let mut html_output: String = String::with_capacity(input.len() * 3 / 2);
    html::push_html(&mut html_output, parser);
    html_output
}

pub fn markdown_filter(value: &Value, _: &HashMap<String, Value>) -> Result<Value> {
    let s = try_get_value!("markdown_filter", "value", String, value);
    Ok(to_value(mark_down(&s)).unwrap())
}
