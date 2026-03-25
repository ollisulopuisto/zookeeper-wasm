# Changelog

All notable changes to the games in this monorepo will be documented in this file. This project uses [CalVer](https://calver.org/) for versioning.

## [26.3.25.112] - 2026-03-25
### Fixed
- **Mobile Unpause (Zookeeper):** Moved UI buttons to be rendered last, ensuring they are always visible and tappable above overlays (fixing an issue where the pause overlay would block the resume button).

## [26.3.25.111] - 2026-03-25
### Added
- **Swipe-to-Swap (Zookeeper):** You can now swap tiles by clicking and dragging in any of the four cardinal directions.
- **UI Improvements (Zookeeper):** Added a helper text below the board to guide new players on control options.

## [26.3.25.110] - 2026-03-25
### Added
- **Keyboard Shortcuts (Zookeeper):** When a piece is selected, you can now use WASD or Arrow Keys to swap it with an adjacent piece.

## [26.3.4.1] - 2026-03-03
### Added
- **Bubbles Game:** A new 16-bit "Bubble Bobble" clone with 2-player support and Amiga/NEO GEO aesthetics.
- **Audiovisual Overhaul (Bubbles):**
  - High-fidelity NEO GEO-style expressive character animations.
  - Multi-channel procedural chiptune system featuring Rush-inspired melodic structures.
  - Smooth 60 FPS rendering with manual viewport scaling for maximum browser compatibility.
- **Enhanced Physics (Bubbles):**
  - "Hefty" movement with horizontal inertia, coyote time, and jump buffering.
  - Conditional vertical screen wrapping through level-specific gaps.
- **Power-up System (Bubbles):** Upgradeable bubble capacity, speed, range, and size.
- **Testing Infrastructure:** Mandated CI success through unit tests for physics, logic, and sprite data integrity.
### Changed
- **Unified High Score Entry:** Bubbles now uses the robust text-based name entry system from Zookeeper, optimized for both desktop and mobile.
### Fixed
- **HUD Readability (Zookeeper):** Readjusted score and time bar positions to prevent layout overlap on mobile.
- **WASM Stability:** Resolved `TypeError` in JS-WASM memory bridge for more reliable leaderboard persistence.

## [26.3.3.42] - 2026-03-03
### Added
- **Explicit Start Button:** Added a "START GAME" HTML button to index.html to ensure 100% reliable audio initialization on all browsers (especially iOS Safari).
- **Robust Audio Unlock:** WASM module now only loads and starts after a direct user gesture, fulfilling strict browser security policies for Web Audio.

## [26.3.3.41] - 2026-03-03

## [26.3.3.37] - 2026-03-03
### Added
- **Slow Mode:** New accessibility toggle (Snail icon) to slow down game speed by 3x for motor disability support.
### Fixed
- Resolved syntax error in Zookeeper pause logic.
- Cleaned up repository history by removing temporary build artifacts.

## [26.3.3.31] - [26.3.3.35] - 2026-03-03
### Added
- **Level Progression:** Each level now has a specific tile clearance goal.
- **Combo System:** Cumulative multipliers for cascades, accompanied by board shake and tile vibration.
- **Centralized Storage:** High scores and settings are now stored in a unified location.
### Changed
- Replaced external WAV assets with **Software Synthesized Audio** (Blips & Blops) for 100% reliability and dynamic pitch shifting.
- Converted 3-initial name entry to full text input.

## [26.3.3.27] - 2026-03-03
### Added
- **Game Portal:** Root landing page with visual "Game Cards" and icons.
- **Pause Button:** Functional pause/resume with Spacebar shortcut.
- **Repo Renaming:** Repository officially renamed to `games`.

## [26.3.3.14] - [26.3.3.23] - 2026-03-02
### Added
- **iOS Safari Support:** "Tap to Start" flow and JS-side audio context resume to fix silent audio on mobile.
- **Mute Toggle:** Persistent speaker icon to remember user audio preferences.
- **Animal Distinction:** Replaced confusingly similar Lion icon with a Penguin.

## [26.3.3.0] - [26.3.3.13] - 2026-03-02
### Added
- Initial Zookeeper Clone (Match-3 logic, 60 FPS WASM).
- Mobile-optimized portrait layout.
- Persistent local high scores.
- Automatic CI/CD pipeline via GitHub Actions.
