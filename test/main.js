import init, {set_internal_coordinates, testing} from './pkg/arwarehouse_pathfinding.js'

async function run() {
    await init(fetch('./pkg/arwarehouse_pathfinding_bg.wasm'))
    const result = add(1, 2);
    console.log(result)
    let arrs = [[1.0, 2.5, 3.6, 1.2], [9.2, 1.6, 3.2, 6.1]]
    arrs = array_add(arrs)
    console.log(arrs)
}

run()