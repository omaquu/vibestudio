# Vibe Studio 🎬

A video editor-style timeline application built in **Rust** with [Dioxus](https://dioxuslabs.com/), compiled to WebAssembly.

## Features

- **Recursive layer tree** — Compositions can contain any layer, with unlimited nesting depth
- **Drag-and-drop reparenting** — Drag layers between compositions, unbind to root, or nest into children
- **Master timeline** — All compositions appear as boxes in a single horizontal track
- **Unbound layers** — Root-level layers span the entire timeline independently
- **Property inspector** — Edit transform, timing, opacity, and scale per layer
- **Dark theme** — Premium dark UI matching professional video editors

## Quick Start

```bash
# Install Rust (if not already)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Dioxus CLI
cargo install dioxus-cli

# Run dev server
dx serve
```

## Deploy to GitHub Pages

Push to `main` — the included GitHub Actions workflow builds and deploys automatically.

## Project Structure

```
src/
├── main.rs        # App layout, entry point, add-item modal
├── model.rs       # Layer, LayerType, AppState (all data + mutations)
├── sidebar.rs     # Left panel: recursive tree with drag-and-drop
├── timeline.rs    # Bottom panel: master comp track + unbound layers
└── inspector.rs   # Right panel: property editor
assets/
└── style.css      # Complete dark theme
```

## Roadmap

- [ ] Audio engine (Web Audio API via `web-sys`)
- [ ] 3D visualizer (WebGL bindings)
- [ ] Timeline clip dragging (trim, move)
- [ ] Desktop target (Dioxus Desktop)
