use array2d::Array2D;
use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

type Location = (usize, usize);

static mut WAREHOUSE_WIDTH: f32 = 0f32;
static mut WAREHOUSE_DEPTH: f32 = 0f32;
static mut RACK_WIDTH: f32 = 0f32;
static mut RACK_DEPTH: f32 = 0f32;
static mut MAX_ROW: i32 = 0i32;
static mut MAX_COL: i32 = 0i32;
// a 2d map of the warehouse
static mut WAREHOUSE: Option<Array2D<bool>> = None;
fn get_warehouse() -> &'static Array2D<bool> {
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
    let mut warehouse =
        Array2D::filled_with(false, max_row.round() as usize, max_col.round() as usize);
    for coordinate in coordinates.iter_mut() {
        // shifts all the coordinate so that top left is 0, 0
        let x: usize = ((coordinate[0] / rack_width) + (max_row / 2f32)).floor() as usize;
        let y: usize = ((coordinate[1] / rack_depth) + max_col).floor() as usize;
        warehouse[(x, y)] = true;
    }

    unsafe {
        MAX_ROW = (warehouse_width / rack_width).round() as i32;
        MAX_COL = (warehouse_depth / rack_depth).round() as i32;
        WAREHOUSE = Some(warehouse);
    }
}

fn get_distance(current_location: &Location, end_location: &Location) -> usize {
    let diffx = (current_location.0 as f32 - end_location.0 as f32).powi(2);
    let diffy = (current_location.1 as f32 - end_location.1 as f32).powi(2);
    return  (diffx + diffy).sqrt().round() as usize;
}

fn get_walkable_surroudings(current: &Location) -> Vec<Location> {
    const EIGHT_DIRECTION: [(i32, i32); 8] = [
        (0, 1),
        (0, -1),
        (1, 0),
        (1, 1),
        (1, -1),
        (-1, 0),
        (-1, 1),
        (-1, -1),
    ];
    let warehouse = get_warehouse();

    let mut temp: Vec<Location> = vec![];
    
    for dir in &EIGHT_DIRECTION {
        let x = current.0 as i32 + dir.0;
        let y = current.1 as i32 + dir.1;
        unsafe {
            // check if the coordinate is in bounds
            if x > 0 && y > 0 && x < MAX_ROW && y < MAX_COL {
                let new_dir = (x as usize, y as usize);

                // check if the location is free to walk on 
                if !warehouse[new_dir] {
                    temp.push(new_dir);
                }
            } 
        }
    }

    return temp;
}

/**
 * A* algorithm for calculating the pathings of two points
 */
#[wasm_bindgen]
pub unsafe fn calculate_path(start_in: Vec<f32>, end_in: Vec<f32>) -> JsValue {
    // calculate the coordinate of the start and end location
    let start_location: Location = (((start_in[0] / RACK_WIDTH) + (MAX_ROW as f32 / 2f32)).floor() as usize,
                 ((start_in[1] / RACK_WIDTH) + MAX_COL as f32).floor() as usize);
    let end_location: Location = (((end_in[0] / RACK_WIDTH) + (MAX_ROW as f32 / 2f32)).floor() as usize,
                 ((end_in[1] / RACK_WIDTH) + MAX_COL as f32).floor() as usize);

    let mut queue: BinaryHeap<Reverse<Location>> = BinaryHeap::new();
    queue.push(Reverse(start_location));


    while !queue.is_empty() {
        let Reverse(current) = queue.pop().unwrap(); 
        if current == end_location {
            break
        }

        for next_coordinate in get_walkable_surroudings(&current) {

        }

    }

    //temp return so rust_analyzer dont error out
    return serde_wasm_bindgen::to_value(&queue).unwrap()
}

#[wasm_bindgen]
pub fn testing() -> JsValue {
    let warehouse: &Array2D<bool> = get_warehouse();
    let mut out: Vec<Vec<i32>> = vec![];
    for i in warehouse.columns_iter() {
        let mut temp: Vec<i32> = vec![];
        for ele in i {
            temp.push(if ele.to_owned() { 1 } else { 0 });
        }
        out.push(temp);
    }

    serde_wasm_bindgen::to_value(&out).unwrap()
}

