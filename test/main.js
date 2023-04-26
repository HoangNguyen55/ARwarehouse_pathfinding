import init, { set_internal_coordinates, testing, calculate_path } from './pkg/arwarehouse_pathfinding.js'

async function parseARD(path) {
    //convert feet to metre, value of ard are feet.
    let array = {
        rackWidth: null,
        rackDepth: null,
        warehouseWidth: null,
        warehouseDepth: null,
        locations: [],
        pickings: []
    }

    const text = await (await fetch(path, { headers: { "Content-Type": "text/xml" } })).text();
    const parser = new DOMParser();
    const xmlDoc = parser.parseFromString(text, "text/xml");

    const warehouse = xmlDoc.getElementsByTagName("warehouse")[0];
    array['rackWidth'] = warehouse.getAttribute("storagelocationwidth") / 3.281;
    array['rackDepth'] = warehouse.getAttribute("storagelocationdepth") / 3.281;

    array['warehouseWidth'] = warehouse.getAttribute("width") / 3.281;
    array['warehouseDepth'] = warehouse.getAttribute("depth") / 3.281;

    const regions = Array.from(xmlDoc.getElementsByTagName("region"));
    regions.forEach((region) => {
        const locations = Array.from(region.getElementsByTagName("storagelocation"));
        locations.forEach((location) => {
            const x = location.getAttribute("x") / 3.281;
            const z = location.getAttribute("y") / 3.281;
            array['locations'].push([x, z]);
        });
    });

    const picks = Array.from(xmlDoc.getElementsByTagName("picklocation"));
    picks.forEach((pick) => {
        let x = pick.getAttribute("x") / 3.281;
        let z = pick.getAttribute("y") / 3.281;
        array['pickings'].push([x,z]);
    });


    return array
}

function find_path(pick) {
    grid = testing();
    let path_found = calculate_path([0, 0], [pick[0], pick[1]]);
    path_found.forEach((i) => {
        let x = Math.floor((i[1] / arr['rackWidth']) + (max_row / 2));
        let y = Math.floor((i[0] / arr['rackDepth']) + max_col);
        grid[x][y] = 2;
    })

    for (let i = 0; i < grid.length; i++) {
        for (let j = 0; j < grid[i].length; j++) {
            if (grid[i][j] === 2) {
                ctx.fillStyle = "green";
                ctx.fillRect(j * 10, i * 10, 10, 10);
            } else if (grid[i][j] === 1) {
                ctx.fillStyle = "black";
                ctx.fillRect(j * 10, i * 10, 10, 10);
            }
        }
    }
}

await init(fetch('./pkg/arwarehouse_pathfinding_bg.wasm'))
const arr = await parseARD('./fishbone.ard');

set_internal_coordinates(arr['warehouseWidth'], arr['warehouseDepth'], arr['rackWidth'], arr['rackDepth'], arr['locations']);
let grid = testing();
const max_row = Math.round(arr['warehouseWidth'] / arr['rackWidth']);
const max_col = Math.round(arr['warehouseDepth'] / arr['rackDepth']);
const canvas = document.getElementById("canvas");
canvas.width = grid[0].length * 10;
canvas.height = grid.length * 10;
const ctx = canvas.getContext("2d");
let t = -3001;

arr['pickings'].forEach((pick) => {
    t += 3000;
    setTimeout(() => { 
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        find_path(pick)
    }, t);
})
