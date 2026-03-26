# Changelog

All notable changes to the games in this monorepo will be documented in this file. This project uses [CalVer](https://calver.org/) for versioning.

## [26.3.26.144] - 2026-03-26
### Changed
- **Animation Timing Refinement (Zookeeper):** Split clearing and falling speeds to improve game rhythm.
  - **Faster Disappearance:** Tile clearing (pop) duration reduced to 0.2s for a snappier feel.
  - **Weightier Falling:** Visual falling decay slowed down to give tiles more "gravitas" as they settle into place.

## [26.3.26.143] - 2026-03-26
### Added
- **Disney-style "Squash and Stretch" (Zookeeper):** Major animation overhaul focusing on weight and fluidity.
  - **Swapping:** Tiles now elongate in the direction of movement and squash upon arrival.
  - **Landing "Thud":** Falling tiles now squash vertically when hitting the bottom or other tiles, giving them a sense of weight.
  - **Enhanced Pop:** Added "anticipation" squash before tiles clear, making the pop feel more reactive.
### Changed
- **Animation Pacing (Zookeeper):** Slowed down core animations (increased duration to 0.35s) to allow the new squash/stretch effects to be visually registered by the player.

## [26.3.26.142] - 2026-03-26
### Added
- **Floating Scores (Zookeeper):** Points now float up from cleared matches, providing immediate and satisfying visual feedback for score gains.

## [26.3.26.141] - 2026-03-26
### Added
- **"Juicy" Animations (Zookeeper):** Significant visual overhaul for all core game actions to make them feel more responsive and satisfying.
  - **Swapping:** Now uses "Back Out" easing for an expressive overshoot effect.
  - **Clearing:** Tiles "pop" more vividly with updated scaling curves and quadratic fades.
  - **Falling:** Discrete 1-cell jumps replaced with smooth interpolation and visual offsets for a natural "cascade" feel.
  - **Selection:** Selected tiles now pulse gently, providing clearer visual feedback.
### Changed
- **Animation Timing (Zookeeper):** Increased internal falling logic frequency to 20Hz for smoother visual stepping.

## [26.3.26.140] - 2026-03-26
### Added
- **Highscore Highlighting (Bubbles & Zookeeper):** Highscores now highlight the entry from the current game session (using yellow) if it made it onto the list.
- **Improved Leaderboard UX:** Clearer visual feedback when you break a record and make it to the top scores.

## [26.3.25.114] - 2026-03-25
### Fixed
- **Input Precision (Zookeeper):** Fixed a bug where tiles on the topmost row and leftmost column were difficult or impossible to click/tap.
### Added
- **Developer Tools (Zookeeper):** The game now logs its version (CalVer) to the browser console on startup for easier troubleshooting.

## [26.3.25.113] - 2026-03-25
### Added
- **Start Screen (Zookeeper):** Added an explicit "Waiting to Start" overlay with game instructions (Controls and Tips).
- **Improved Controls (Zookeeper):** Helper text now mentions "Click" in addition to Swipe and WASD.
- **Enhanced Pause (Zookeeper):** You can now unpause by tapping anywhere on the screen, not just the small Play icon.
### Fixed
- **Mobile Interaction (Zookeeper):** Moved the game board slightly down to avoid interference with browser/OS-level gestures when interacting with the top row.

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
