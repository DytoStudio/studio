# Dyto Studio

A modern OSM editor designed for `HDMaps`.

## Project Goals

Dyto Studio is a modern OSM editor designed for `HDMaps`. It is built using Rust
and TypeScript, and is designed to be fast and efficient.

### Roadmap and Features

- [ ] Image tile loading (Bing Aerial)
- [ ] Data loading and saving
    - [ ] OSM XML
    - [ ] OSM PBF
- [ ] OSM data loading and editing
    - [ ] Nodes
    - [ ] Ways
    - [ ] Areas
    - [ ] Side Panel with tags and metadata
- [ ] HDMap data loading and editing
    - [ ] Lane
    - [ ] LaneBoundary
    - [ ] TrafficSignalLine
    - [ ] StopLine
    - [ ] Crosswalk (Area)
    - [ ] Side Panel with tags and metadata
- [ ] Editor tools
    - [ ] Select
    - [ ] Move
    - [ ] Add Node/Way/Area
    - [ ] Delete
    - [ ] Undo / Redo
- [ ] 3D rendering of map data

### What is HDMap?

HDMap is a high-definition map format designed for autonomous vehicles.

Dyto Studio and it's extensions to OSM are designed to support the requirements
of AVs (Autonomous Vehicles), however the primary goal is purely to create
higher quality map data that can be used for a variety of purposes like
immersive navigation, gaming, and more.

### Why Rust?

Existing editors such as [iD](https://github.com/openstreetmap/iD) and
[Rapid](https://github.com/facebook/rapid) are built using JavaScript, which is
not suitable 3D and high detail editing. Rust is a systems programming language
that is designed to be fast and efficient.

## Development

Dyto Studio is built using Rust and TypeScript. The Rust code is used for the 3D
rendering and editing, while the TypeScript code is used for the UI and
application logic.

### Dependencies

- [Rust](https://www.rust-lang.org/)
  - `wasm32-unknown-unknown` target for web development
    (installed via `rustup target add wasm32-unknown-unknown`)
  - `wasm-pack` for JavaScript interop
    (installed via `cargo install wasm-pack`)
- [Node.js](https://nodejs.org/)
- [Pnpm](https://pnpm.io/)

### Building and Running

01. Clone the repository:
   ```bash
   git clone https://github.come/dytoStudio/studio.git
   cd studio
   ```
02. Build the Rust code for WebAssembly:
    ```bash
    cd packages/scene
    pnpm build
    ```
03. Start the development server:
    ```bash
    cd packages/app
    pnpm dev
    ```
04. Open your browser and navigate to `http://localhost:5173` to see the
    application running.

### Contributing

Contributions to Dyto Studio are welcome.
1. Make sure to run `pnpm format` to format TypeScript.
2. Make sure to run `cargo fmt` to format Rust code. 

## License

Dyto Studio currently is licensed under CC-BY-NC-ND. This will change in the
future.
