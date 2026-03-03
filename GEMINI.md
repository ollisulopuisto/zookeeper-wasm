# Monorepo Structure

This project is a monorepo for WASM-based games written in Rust.

## Workspaces

- **Root Workspace**:
  - `jetpac_wasm/`: A WASM implementation of Jetpac.
  - Managed by root `Cargo.toml`.

- **Games Repository (`games_repo/`)**:
  - A nested workspace (potentially a submodule or separate repo) for various games.
  - `games/zookeeper/`: A match-3 style game.
  - Managed by `games_repo/Cargo.toml`.

## Key Files
- `assets/`: Shared or project-specific assets (partially mirrored in sub-projects).
- `.github/workflows/deploy.yml`: Root deployment workflow.
- `scripts/`: Project-wide utility scripts (e.g., `bump_version.py`).

## Tech Stack
- **Language**: Rust
- **Target**: WebAssembly (WASM)
- **Deployment**: GitHub Pages (likely, based on `deploy.yml`)
