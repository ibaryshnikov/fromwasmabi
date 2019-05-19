use wasm_bindgen::prelude::*;

#[allow(non_snake_case)]
pub mod Namespace {
    use super::*;

    #[wasm_bindgen]
    extern "C" {
        // after adding `,catch` after `Namespace` error goes away
        #[wasm_bindgen(js_namespace = Namespace)]
        pub fn example() -> Result<JsValue, JsValue>;
    }
}
