Check out my project at https://wasm.joshuasmith.net

Or on my GitHub (if you're not already there): https://github.com/ProbatusVir/mars-wasm

This is just a project that I'm making to clear my head from my college Capstone!

While not much yet, I intend to make a fully-fledged IDE for WebAssembly, very similar to MARS MIPS, but for Wasm instead!

## Dependencies
* wasm-pack to install, run:
```sh
cargo install wasm-pack
```

## How to build
* Release 
```sh
wasm-pack build --target web
```
* Debug
```sh
wasm-pack build --target web --dev
```

## How to view the page
You must first run a webserver. People usually do this with JavaScript or Python, here's the Python way:
```sh
python3 -m http.server [optional: port-number]
```
Then, you must connect to it in your browser: either `127.0.0.1:[PORT_NUMBER]` or `localhost:[PORT_NUMBER]`

