use wasm_bindgen::{JsValue, prelude::wasm_bindgen};
use wasm_bindgen_futures::future_to_promise;
use web_sys::js_sys::Promise;

use crate::rpc;

#[wasm_bindgen]
pub fn get_username(username: String) -> Promise {
    future_to_promise(async {
        let username = rpc::get_username(username).await?;
        Ok(JsValue::from_str(&username))
    })
}
