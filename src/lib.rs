extern crate zxcvbn;

use wasm_bindgen::prelude::*;
use zxcvbn::zxcvbn as zxcvbn_fn;
use serde_wasm_bindgen::to_value;

#[wasm_bindgen(js_name = zxcvbn)]
pub fn zxcvbn_js(password: &str) -> JsValue {
    let result = zxcvbn_fn(password, &[]);
    // Directly convert the Rust struct to JsValue, which will be a JavaScript object
    to_value(&result).unwrap()
}
