use array2d::Array2D;
use serde_wasm_bindgen;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::convert::{From, Into};
use std::marker::Copy;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: String);
}

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
 * returns coordinates in AR, in meters
 */
unsafe fn get_real_coordinate(location: &Node) -> Vec<f32> {
    let x: f32 = ((location.x as f32) - (MAX_ROW as f32 / 2f32)) * RACK_WIDTH;
    let y: f32 = ((location.y as f32) - MAX_COL as f32) * RACK_DEPTH;

    return vec![x, y];
}

unsafe fn get_grid_coordinate<T>(input_coordinates: Vec<T>) -> Node
where
    T: From<f32> + Into<f32> + Copy,
{
    let x = ((input_coordinates[0].into() / RACK_WIDTH) + (MAX_ROW as f32 / 2f32)).floor() as i32;
    let y = ((input_coordinates[1].into() / RACK_WIDTH) + MAX_COL as f32).floor() as i32;

    return Node{x, y};
}


#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Node {
    x: i32,
    y: i32,
}

impl Node {
    fn get_distance(&self, to_location: &Node) -> i32 {
        let x = (self.x - to_location.x).abs();
        let y = (self.y - to_location.y).abs();
    
        if x > y {
            return 14 * y + 10 * (x - y);
        }
        return 14 * x + 10 * (y - x);
    }


    unsafe fn get_neighbors(&self) -> Vec<Node> {
        let mut temp: Vec<Node> = vec![];

        for x in -1..=1 {
            for y in -1..=1 {
                if x == 0 && y == 0 {
                    continue;
                }

                let new_x = self.x + x;
                let new_y = self.y + y;

                if new_x < 0 || new_x >= MAX_ROW || new_y < 0 || new_y >= MAX_COL {
                    continue;
                }

                temp.push(Node{x: new_x, y: new_y});
            }
        }

        temp
    }

    fn get_coordinate(&self) -> (usize, usize) {
        return (self.x as usize, self.y as usize)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct State {
    node: Node,
    f_score: i32,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_score.cmp(&self.f_score)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/**
 * A* algorithm for calculating the pathings of two points
 */
#[wasm_bindgen]
pub unsafe fn calculate_path(start_lo: Vec<f32>, goal_lo: Vec<f32>) -> JsValue {
    let start: Node = get_grid_coordinate(start_lo);
    let goal: Node = get_grid_coordinate(goal_lo);

    // log(format!("start: x:{} y:{}, goal: x: {} y:{}", start.x, start.y, goal.x, goal.y));
    let mut open_set = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut g_score = HashMap::new();
    let mut f_score = HashMap::new();
    let grid = get_warehouse();

    g_score.insert(start, 0);
    f_score.insert(start, start.get_distance(&goal));

    open_set.push(State { node: start, f_score: start.get_distance(&goal) });

    while let Some(current_state) = open_set.pop() {
        let current = current_state.node;

        if current == goal {
            let mut path: Vec<Vec<f32>> = Vec::new();
            let mut current_node = current;

            while let Some(&node) = came_from.get(&current_node) {
                path.push(get_real_coordinate(&current_node));
                current_node = node;
            }

            path.push(get_real_coordinate(&start));
            // path.reverse();
            return serde_wasm_bindgen::to_value(&path).unwrap();
        }

        for neighbor in current.get_neighbors() {
            if grid[neighbor.get_coordinate()] {
                continue;
            }

            let tentative_g_score = g_score.get(&current).unwrap() + current.get_distance(&neighbor);

            if !g_score.contains_key(&neighbor) || tentative_g_score < *g_score.get(&neighbor).unwrap() {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                f_score.insert(neighbor, tentative_g_score + neighbor.get_distance(&goal));
                open_set.push(State { node: neighbor, f_score: tentative_g_score + neighbor.get_distance(&goal) });
            }
        }
    }

    return serde_wasm_bindgen::to_value("No Path Founds").unwrap();
}

/**
 * warehouse_width: width of the warehouse in meters
 * warehouse_depth: depth of the warehouse in meters
 * rack_width: width of the rack in meters
 * rack_depth: depth of the rack in meters
 * coords: an array of x and y values of the ard file \[(x1,y1), (x2, y2)]
*/
#[wasm_bindgen]
pub unsafe fn set_internal_coordinates(
    warehouse_width: f32,
    warehouse_depth: f32,
    rack_width: f32,
    rack_depth: f32,
    coords: JsValue,
) {
    WAREHOUSE_WIDTH = warehouse_width;
    WAREHOUSE_DEPTH = warehouse_depth;
    RACK_DEPTH = rack_depth;
    RACK_WIDTH = rack_width;
    MAX_ROW = (warehouse_width / rack_width).round() as i32;
    MAX_COL = (warehouse_depth / rack_depth).round() as i32;

    let mut coordinates: Vec<Vec<f32>> = serde_wasm_bindgen::from_value(coords).unwrap();

    // normalize all the values so that a single rack take up 1 cell in the internal representation of a 2d map
    let mut warehouse = Array2D::filled_with(false, MAX_ROW as usize, MAX_COL as usize);
    for coordinate in coordinates.iter_mut() {
        // shifts all the coordinate so that 0, 0 is middle bottom
        let coord: (usize, usize) = get_grid_coordinate(coordinate.to_owned()).get_coordinate();
        warehouse[coord] = true;
    }

    // log(format!("row: {MAX_ROW}, col:{MAX_COL}"));

    WAREHOUSE = Some(warehouse);
}

#[wasm_bindgen]
pub unsafe fn testing() -> JsValue {
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
