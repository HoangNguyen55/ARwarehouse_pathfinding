use array2d::Array2D;
use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;

static mut WAREHOUSE_WIDTH: f32 = 0f32;
static mut WAREHOUSE_DEPTH: f32 = 0f32;
static mut RACK_WIDTH: f32 = 0f32;
static mut RACK_DEPTH: f32 = 0f32;

// a 2d map of the warehouse
static mut WAREHOUSE: Option<Array2D<bool>> = None;
fn get_warehouse() -> &'static mut Array2D<bool> {
    unsafe {
        if WAREHOUSE.is_none() {
            panic!("Run set_internal_coordinates() first");
        } else {
            std::mem::transmute(WAREHOUSE.as_mut().unwrap())
        }
    }
}

/**
 * warehouse_width: width of the warehouse in meters
 * warehouse_depth: depth of the warehouse in meters
 * rack_width: width of the rack in meters
 * rack_depth: depth of the rack in meters
 * coords: an array of x and y values of the ard file \[(x1,y1), (x2, y2)]
*/
#[wasm_bindgen]
pub fn set_internal_coordinates(
    warehouse_width: f32,
    warehouse_depth: f32,
    rack_width: f32,
    rack_depth: f32,
    coords: JsValue,
) {
    unsafe {
        WAREHOUSE_WIDTH = warehouse_width;
        WAREHOUSE_DEPTH = warehouse_depth;
        RACK_DEPTH = rack_depth;
        RACK_WIDTH = rack_width;
    }

    let mut coordinates: Vec<Vec<f32>> = serde_wasm_bindgen::from_value(coords).unwrap();

    // normalize all the values so that a single rack take up 1 cell in the internal representation of a 2d map
    let max_row = warehouse_width / rack_width;
    let max_col = warehouse_depth / rack_depth;
    let mut warehouse = Array2D::filled_with(false, max_row.round() as usize, max_col.round() as usize);
    for coordinate in coordinates.iter_mut() {
        // shifts all the coordinate so that top left is 0, 0
        let x: usize = ((coordinate[0] / rack_width) + (max_row / 2f32)).floor() as usize;
        let y: usize = ((coordinate[1] / rack_depth) + max_col).floor() as usize;
        warehouse[(x, y)] = true;
    }

    unsafe {
        WAREHOUSE = Some(warehouse);
    }
}

#[wasm_bindgen]
pub fn testing() -> JsValue {
    let warehouse: &Array2D<bool> = get_warehouse();
    let mut out: Vec<Vec<i32>> = vec![];
    for i in warehouse.columns_iter() {
        let mut temp: Vec<i32> = vec![];
        for ele in i {
            temp.push(if ele.to_owned() {1} else {0});
        }
        out.push(temp);
    }

    serde_wasm_bindgen::to_value(&out).unwrap()
}

#[wasm_bindgen]
pub fn calculate_path(from: Vec<f32>, to: Vec<f32>) -> JsValue {
    let mut paths_found: Vec<f32> = Vec::new();

    serde_wasm_bindgen::to_value(&paths_found).unwrap()
}
