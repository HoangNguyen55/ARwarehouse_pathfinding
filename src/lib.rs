use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: u32, b: u32) -> u32 {
    a + b
}

// can't 
#[wasm_bindgen]
pub fn arrays(arr: js_sys::Array) -> js_sys::Array {
    let mut out = js_sys::Array::new();
    out.push(&JsValue::from(32));
    out.extend(arr.iter().collect());
    out
}
// pub fn arrays(warehouse_width: f32, warehouse_depth: f32, rack_width: f32, rack_depth: f32, x_coordinates: Vec<f32>, y_coordinates: Vec<f32>) -> js_sys::Array {
//     let a = js_sys::Array::new();
//     a
// }