# ARwarehouse_pathfinding

external dependencies: `wasm-pack`

A* Pathfinding algorithm written in Rust compiles to Webassemply for the ARwarehouse project.

To run a standalone demo of the project:
```
wasm-pack build --target web --out-dir test/pkg/
cd test
python -m http.server 5173
```
Then access http://0.0.0.0:5173 in a Chromium-based browser.
