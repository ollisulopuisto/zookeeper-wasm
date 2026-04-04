# Changelog

All notable changes to the games in this monorepo will be documented in this file. This project uses [CalVer](https://calver.org/) for versioning.

## [26.4.4.196] - 2026-04-04
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
