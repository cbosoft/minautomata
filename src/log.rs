use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern {
    /// Bring the external function "console.log" into rust.
    #[wasm_bindgen(js_namespace = console)]
    pub fn log(s: String);
}
