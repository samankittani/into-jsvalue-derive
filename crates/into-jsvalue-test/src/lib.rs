use serde::Serialize;
use wasm_bindgen::prelude::*;

use into_jsvalue_derive::IntoJsValue;

#[wasm_bindgen]
pub async fn async_test() -> Test {
    Test::One(String::new())
}

#[derive(Serialize, IntoJsValue)]
pub enum Test {
    One(String),
    Two(String, String),
}
