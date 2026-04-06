# Changelog

All notable changes to the games in this monorepo will be documented in this file. This project uses [CalVer](https://calver.org/) for versioning.

## [26.4.6.225] - 2026-04-06
### Changed
- **Refactoring & Optimization:** Improved codebase maintainability and encapsulation across shared library and games.
  - Encapsulated shape and color selection into `shared::theme::Theme` helpers.
  - Refactored `draw_stylized_block` in Lumines to use a new `draw_shape_fill` helper, reducing code duplication.
  - Streamlined procedural music generation with more concise tone pattern definitions.
  - Decoupled high-score input logic from platform detection for better modularity.
  - Moved `BlockColor` to the shared library to centralize game state definitions.

## [26.04.06.218] - 2026-04-06

### Added
- **Lumines WASM Difficulty Progression:** Implemented a new leveling system inspired by *Lumines Remastered* and *Lumines Arise*.
  - Added level progression (Level up every 10 squares deleted).
  - Implemented dynamic drop-speed scaling using a smooth curve ($1.0 \times 0.98^{(level-1)}$).
  - Synchronized skin/theme transitions to trigger every 10 levels (100 squares).
  - Updated HUD to show "NEXT IN: X" to display progress toward the next level/skin.
  - Linked theme BPM to the timeline speed for increased challenge as themes advance.

## [26.4.5.217] - 2026-04-05
### Fixed
- **Music Editor WASM:** Fixed CI/CD build failure by adding missing `index.html` and adding the project to the root Cargo workspace.

## [26.4.5.216] - 2026-04-05
### Fixed
- **Lumines WASM Sync:** Decoupled visual style transitions from the score-based theme suggestion. Visuals, timeline speed, and "STYLE UNLOCKED" notifications now trigger in perfect sync with the music loop point.
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
