#![cfg(target_arch = "wasm32")]

use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;
use product::*;

#[wasm_bindgen(module = "demo.js")]
extern "C" {
    #[wasm_bindgen(js_name = "getProductSKU")]
    fn get_product_sku(index: usize) -> String;
}

#[wasm_bindgen_test]
fn should_work() {
    assert_eq!(get_product_sku(0), "PROD-001");
    assert_eq!(get_product_sku(1), "PROD-002");
}
