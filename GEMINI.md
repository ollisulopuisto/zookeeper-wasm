# Monorepo Structure

This project is a monorepo for WASM-based games written in Rust.

## Workspaces

Managed by root `Cargo.toml`:
- `games_repo/games/jetpac_wasm/`: A WASM implementation of Jetpac.
- `games_repo/games/zookeeper/`: A match-3 style game.

## Key Files
- `games_repo/index.html`: The main games portal.
- `.github/workflows/deploy.yml`: Root deployment workflow for all games and the portal.
- `scripts/`: Project-wide utility scripts.

## Tech Stack
- **Language**: Rust
- **Target**: WebAssembly (WASM)
- **Deployment**: GitHub Pages (automatically via GH Actions)
