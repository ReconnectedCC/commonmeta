use crate::{parse_content, parse_to_json as internal_parse_to_json};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse_to_json(input: &str) -> String {
    internal_parse_to_json(input)
}

#[wasm_bindgen]
pub fn parse_to_object(input: &str) -> JsValue {
    let result = parse_content(input);
    serde_wasm_bindgen::to_value(&result).unwrap_or(JsValue::NULL)
}

#[wasm_bindgen]
pub fn parse_pairs(input: &str) -> JsValue {
    let result = parse_content(input);
    if result.success {
        serde_wasm_bindgen::to_value(&result.pairs).unwrap_or(JsValue::NULL)
    } else {
        JsValue::NULL
    }
}

#[wasm_bindgen]
pub fn validate(input: &str) -> bool {
    parse_content(input).success
}

#[wasm_bindgen]
pub fn get_error(input: &str) -> Option<String> {
    let result = parse_content(input);
    if !result.success { result.error } else { None }
}
