# Changelog

All notable changes to the games in this monorepo will be documented in this file. This project uses [CalVer](https://calver.org/) for versioning.

## [26.4.5.217] - 2026-04-05
### Added
- **Visual Progression & Aesthetic Styles (Lumines):**
  - Introduced five distinct visual themes: Classic, Neon, Retro, Crystal, and Inferno.
  - Added specialized rendering for each theme, including unique color palettes and block shapes.
  - Implemented a score-based progression system that unlocks new styles at 500, 2000, 5000, and 10,000 points.
  - Enhanced HUD with current Level and Style name indicators.
  - Added a "STYLE UNLOCKED!" notification that flashes when a new visual level is reached.
- **Expressive Completion Animations (Lumines):**
  - Added per-cell particle explosions when blocks are cleared by the sweep line.
  - Added column-clear flash and board-edge match glow effects.
### Fixed
- **Lumines WASM:** Fixed potential performance and correctness issues when the game hitches or is unfocused.
  - Clamped `dt` to 0.1s to prevent excessive processing during long frames.
  - Replaced single `if` with a `while` loop for the timeline wrap to ensure `timeline_x` always returns to the valid range.
### Refactored
- **Lumines WASM:** Extracted particle physics and completion animation magic numbers into named constants for improved maintainability.

## [26.4.3.168] - 2026-04-03
### Added
- **New Game: Lumines WASM:** A rhythm-puzzle game clone based on the Lumines mechanism.
  - Implemented 16x10 grid with 2x2 falling blocks.
  - Added rotation, movement, and fast drop mechanics.
  - Integrated synthesized audio system with music and sound effects.
  - Implemented Time Freeze (Shift key) mechanic when meter is full.
  - Added particle systems and board-flash animations for block clears.
  - Added high-score leaderboard tracking.
  - Optimized for both landscape (desktop) and portrait (mobile) layouts.
