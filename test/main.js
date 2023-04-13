import init, { set_internal_coordinates, testing } from './pkg/arwarehouse_pathfinding.js'

async function parseARD(path) {
    //convert feet to metre, value of ard are feet.
    let array = {
        rackWidth: null,
        rackDepth: null,
        warehouseWidth: null,
        warehouseDepth: null,
        locations: []
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

    return array
}

async function run() {
    await init(fetch('./pkg/arwarehouse_pathfinding_bg.wasm'))
    const arr = await parseARD('./fishbone.ard');

    set_internal_coordinates(arr['warehouseWidth'], arr['warehouseDepth'], arr['rackWidth'], arr['rackDepth'], arr['locations']);
    let a = testing();
    console.log(a);
}

run()