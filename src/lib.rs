use array2d::Array2D;
use serde_wasm_bindgen;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::convert::{From, Into};
use std::marker::Copy;
use wasm_bindgen::prelude::*;

// global variable to cache needed values for later.
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
            return std::mem::transmute(WAREHOUSE.as_mut().unwrap());
        }
    }
}

/**
 Returns coordinates in AR, in meters
 */
unsafe fn get_real_coordinate(location: &Node) -> Vec<f32> {
    let x: f32 = ((location.x as f32) - (MAX_ROW as f32 / 2f32)) * RACK_WIDTH;
    let y: f32 = ((location.y as f32) - MAX_COL as f32) * RACK_DEPTH;

    return vec![x, y];
}

/**
  shifts all the coordinate so that 0, 0 is middle bottom
  Returns coordinates in the grid
 */
unsafe fn get_grid_coordinate<T>(input_coordinates: Vec<T>) -> Node
where
    T: From<f32> + Into<f32> + Copy,
{
    let mut x = ((input_coordinates[0].into() / RACK_WIDTH) + (MAX_ROW as f32 / 2f32)).floor() as i32;
    let mut y = ((input_coordinates[1].into() / RACK_WIDTH) + MAX_COL as f32).floor() as i32;
    if x >= MAX_ROW {
        x = MAX_ROW - 1
    } else if x <= 0 {
        x = 0;
    }
    if y >= MAX_COL {
        y = MAX_COL - 1
    } else if y <= 0 {
        y = 0;
    }

    return Node { x, y };
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Node {
    x: i32,
    y: i32,
}

impl Node {
    // heuristic function
    /**
     Returns distance from current node, 10 for updownleftright and 14 for diagonal
     */
    fn get_distance(&self, to_location: &Node) -> i32 {
        let x = (self.x - to_location.x).abs();
        let y = (self.y - to_location.y).abs();

        if x > y {
            return 14 * y + 10 * (x - y);
        }
        return 14 * x + 10 * (y - x);
    }

    /**
     Returns adjacent node that are in bound
     */
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

                temp.push(Node { x: new_x, y: new_y });
            }
        }

        return temp;
    }

    /**
     Returns x, y coordinate as a tuple, for use with array2d
     */
    fn get_coordinate(&self) -> (usize, usize) {
        return (self.x as usize, self.y as usize);
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
// a wrapper around node so when using with a binaryheap it is sorted by f_score
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
 A* algorithm for calculating the pathings of two points
 */
#[wasm_bindgen]
pub unsafe fn calculate_path(start_lo: Vec<f32>, goal_lo: Vec<f32>) -> JsValue {
    let start: Node = get_grid_coordinate(start_lo);
    let goal: Node = get_grid_coordinate(goal_lo);
    let grid: &Array2D<bool> = get_warehouse();

    // priority queue, O(logn) finding the smallest f_cost
    let mut open_set: BinaryHeap<State> = BinaryHeap::new();

    // keep track of traversed path
    let mut came_from: HashMap<Node, Node> = HashMap::new();

    // keep track of g_score, which we can also 
    // use to see which path have been visited
    let mut g_score: HashMap<Node, i32> = HashMap::new();

    g_score.insert(start, 0);

    open_set.push(State {
        node: start,
        f_score: start.get_distance(&goal),
    });

    // loop until open_set is empty
    while let Some(current_state) = open_set.pop() {
        let current = current_state.node;

        // if we are at destination, retrace our steps in the came_from map
        if current == goal {
            let mut path: Vec<Vec<f32>> = Vec::new();
            let mut current_node = current;

            while let Some(&node) = came_from.get(&current_node) {
                path.push(get_real_coordinate(&current_node));
                current_node = node;
            }

            path.push(get_real_coordinate(&start));
            return serde_wasm_bindgen::to_value(&path).unwrap();
        }

        for neighbor in current.get_neighbors() {
            // if neighbor is a wall skip current iteration
            if grid[neighbor.get_coordinate()] {
                continue;
            }

            let tentative_g_score =
                g_score.get(&current).unwrap() + current.get_distance(&neighbor);

            if !g_score.contains_key(&neighbor)
                || tentative_g_score < *g_score.get(&neighbor).unwrap()
            {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g_score);
                open_set.push(State {
                    node: neighbor,
                    f_score: tentative_g_score + neighbor.get_distance(&goal),
                });
            }
        }
    }

    return serde_wasm_bindgen::to_value("No Path Founds").unwrap();
}

/**
 Calculate all the needed value and cache them
 Params: warehouse_width: width of the warehouse in meters
 Params: warehouse_depth: depth of the warehouse in meters
 Params: rack_width: width of the rack in meters
 Params: rack_depth: depth of the rack in meters
 Params: coords: an array of x and y values of the ard file \[(x1,y1), (x2, y2)]
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

    let mut warehouse = Array2D::filled_with(false, MAX_ROW as usize, MAX_COL as usize);
    for coordinate in coordinates.iter_mut() {
        let coord: (usize, usize) = get_grid_coordinate(coordinate.to_owned()).get_coordinate();
        warehouse[coord] = true;
    }

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
