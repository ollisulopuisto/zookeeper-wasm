# WASM Games Monorepo

A collection of self-contained, 60 FPS WASM games built with Rust and Macroquad.

## 🎮 Play Games
**[Main Portal on GitHub Pages](https://ollisulopuisto.github.io/games/)**

Available Games:
- **[Zookeeper Clone](https://ollisulopuisto.github.io/games/zookeeper/)**

## 🚀 Features
- **Monorepo Architecture:** Powered by Cargo Workspaces.
- **High Performance:** 60 FPS WebGL rendering via Rust and Macroquad.
- **Mobile Optimized:** Responsive layouts with touch controls.
- **Unified CI/CD:** Automated builds and deployments for all games.

## 🛠 Tech Stack
- **Workspace:** [Cargo Workspaces](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html)
- **Game Engine:** [Macroquad](https://macroquad.rs/) (Rust)
- **Asset Management:** Python with [uv](https://github.com/astral-sh/uv)
- **Versioning:** [CalVer](https://calver.org/)

## 📦 Building Locally

### Prerequisites
- [Rust](https://rustup.rs/) with `wasm32-unknown-unknown` target
- [uv](https://github.com/astral-sh/uv) for Python scripts

### Commands
```bash
# Setup all games
make setup

# Build all games into the docs/ folder
make build

# Run tests for the entire workspace
make test
```

## ➕ Adding a New Game
1. Create a new directory in `games/`.
2. Initialize a new Macroquad project or copy an existing one.
3. Ensure it has a `Makefile` with `setup`, `assets`, and `build` targets.
4. Add a link to the new game in the root `index.html`.

## 📄 License
MIT / Public Domain
