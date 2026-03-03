# Changelog

All notable changes to the games in this monorepo will be documented in this file. This project uses [CalVer](https://calver.org/) for versioning.

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
