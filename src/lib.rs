use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: JsValue, b: u32) -> u32 {
    let c: u32 = serde_wasm_bindgen::from_value(a).unwrap();
    c + b
}

#[wasm_bindgen]
pub fn array_add(arr: JsValue) -> JsValue {
    let mut arr_in: Vec<Vec<f32>> = serde_wasm_bindgen::from_value(arr).unwrap();
    for row in arr_in.iter_mut() {
        for cell in row.iter_mut() {
            *cell += 52f32;
        }
    }

    serde_wasm_bindgen::to_value(&arr_in).unwrap()
}
// pub fn arrays(warehouse_width: f32, warehouse_depth: f32, rack_width: f32, rack_depth: f32, x_coordinates: Vec<f32>, y_coordinates: Vec<f32>) -> js_sys::Array {
//     let a = js_sys::Array::new();
//     a
// }
