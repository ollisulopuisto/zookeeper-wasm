# Changelog

All notable changes to the games in this monorepo will be documented in this file. This project uses [CalVer](https://calver.org/) for versioning.

## [26.3.300169] - 2026-03-30
### Changed
- **CI/CD Optimization:** Integrated `wasm-opt` into the deployment pipeline to further reduce WASM binary sizes and improve load times.

## [26.3.300161] - 2026-03-30
### Changed
- **iOS Sound Initialization:** Added auto-start logic to skip the "Start Game" button on non-iOS platforms across all games (Bubbles, Zookeeper, Jetpac).
- **Level Transition Refinement (Bubbles):** Smoothed out the level cleared warp animation to be less "restless" by reducing rotation and scaling intensity.
- **Level Clear Animation (Zookeeper):** Added a refined, non-restless level clear text animation that settles before the level change.

## [26.3.27.150] - 2026-03-27
### Added
- **Expanded Bestiary (Zookeeper):** Added 8 new animal types: Rabbit, Cat, Dog, Mouse, Sheep, Chick, Fox, and Cow, bringing the total to 20.
- **Dynamic Difficulty Extension (Zookeeper):** Level progression now scales up to 20 animal types (at level 29+), starting with a more relaxed 6 animals at Level 1 for a smoother difficulty curve.
- **Asset Downloader (Zookeeper):** Updated `download_assets.py` to support the new animals.
### Fixed
- **Asset Tests (Zookeeper):** Updated `test_assets.py` to correctly assert the new emoji count (25) and allow shorter hex codes for UI icons.
- **Name Entry UX (Zookeeper):** Restricted the "TAP FOR POPUP" modal button on WASM to mobile-sized screens. Desktop users can now use direct keyboard input without being prompted for a modal, improving the experience on non-touch devices.

## [26.3.27.149] - 2026-03-27
### Added
- **Dynamic Difficulty Scaling (Zookeeper):** Animal variety now increases as you progress through levels.
  - **Level 1-2:** 7 animal types (standard).
  - **Progression:** One new animal type is added every 2 levels.
  - **Endgame:** Reaches 12 animal types at Level 11+, significantly increasing puzzle complexity.
- **Expanded Bestiary (Zookeeper):** Added 5 new animal types with custom emoji-based textures: Lion, Hippo, Zebra, Pig, and Koala.

## [26.3.26.145] - 2026-03-26
### Added
- **Gravity-based Acceleration (Zookeeper):** Falling tiles now speed up as they drop, creating a more dynamic and natural-looking "thud" as they land. This complements the existing squash and stretch animations for a more physical feel.

## [26.3.26.144] - 2026-03-26
### Changed
- **Animation Timing Refinement (Zookeeper):** Split clearing and falling speeds to improve game rhythm.
  - **Faster Disappearance:** Tile clearing (pop) duration reduced to 0.2s for a snappier feel.
  - **Weightier Falling:** Visual falling decay slowed down to give tiles more "gravitas" as they settle into place.
