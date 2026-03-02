# Zookeeper WASM

A self-contained, 60 FPS Match-3 clone built with Rust and Macroquad, optimized for mobile browsers.

## 🎮 Play Now
**[Live Demo on GitHub Pages](https://ollisulopuisto.github.io/zookeeper-wasm/)**

## 🚀 Features
- **High Performance:** 60 FPS WebGL rendering via Rust and Macroquad.
- **Mobile Optimized:** Responsive portrait layout with touch controls.
- **Persistent High Scores:** Leaderboard saved to browser `localStorage`.
- **Juicy Animations:** Smooth tile swaps, gravity effects, and clear animations.
- **Embedded Assets:** All sounds and graphics are embedded in the WASM binary for zero-latency loading.

## 🛠 Tech Stack
- **Game Engine:** [Macroquad](https://macroquad.rs/) (Rust)
- **Asset Management:** Python with [uv](https://github.com/astral-sh/uv)
- **Versioning:** [CalVer](https://calver.org/)
- **CI/CD:** GitHub Actions with atomic deployments

## 📦 Building Locally

### Prerequisites
- [Rust](https://rustup.rs/) with `wasm32-unknown-unknown` target
- [uv](https://github.com/astral-sh/uv) for Python scripts

### Commands
```bash
# Setup environments
make setup

# Run Rust tests
make test

# Build and serve locally
make serve
```

## 📄 License
MIT / Public Domain (Assets from Twemoji and Wikimedia Commons)
