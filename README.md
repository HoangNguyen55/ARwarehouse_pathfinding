# ARwarehouse_pathfinding

A* Pathfinding algorithm written in Rust compiles to Webassemply for the ARwarehouse project.

To run a standalone demo of the project:
```
wasm-pack build --target web --out-dir test/pkg/
cd test
python -m http.server 7666
```
Then access http://0.0.0.0:7666 in a Chromium-based browser.
