use thiserror::Error;
use wasm_bindgen::JsValue;

#[derive(Debug, Error)]
#[error("JsValue: {value}")]
pub struct JsValueError {
    // JsValue is not Send + Sync, but openscq30_lib::Error requires inner errors to be Send + Sync,
    // so we must convert it to a string now.
    value: String,
}

impl From<JsValue> for JsValueError {
    fn from(value: JsValue) -> Self {
        Self {
            value: format!("{value:?}"),
        }
    }
}
