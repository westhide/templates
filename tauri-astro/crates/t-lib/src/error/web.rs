use wasm_bindgen::JsValue;

use crate::error::Error;

impl From<Error> for JsValue {
    fn from(error: Error) -> Self {
        let s = error.to_string();
        JsValue::from_str(&s)
    }
}
