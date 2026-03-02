//! Zookeeper WASM: A 60 FPS Match-3 Clone in Rust
//!
//! This module provides a fully self-contained match-3 game using the Macroquad engine.
//! It handles the 8x8 game board, animal matching logic, animations, and high scores.

use macroquad::prelude::*;
use quad_rand as qrand;

/// The standard grid width for the game board.
const COLS: usize = 8;
/// The standard grid height for the game board.
const ROWS: usize = 8;
/// The number of distinct animal types available in the game.
const TILE_TYPES: u8 = 7;
/// The duration (in seconds) of tile animations like swapping.
const ANIM_DURATION: f32 = 0.2;
/// Maximum number of high scores to keep in the local leaderboard.
const MAX_HIGH_SCORES: usize = 5;

/// Represents the current state of the game loop and any active animations.
#[derive(Clone, Copy, PartialEq, Debug)]
enum GameState {
    /// The game is waiting for player input.
    Idle,
    /// Two tiles are in the process of being swapped.
    /// - `x1, y1`: Coordinates of the first tile.
    /// - `x2, y2`: Coordinates of the second tile.
    /// - `timer`: Progress of the swap animation (0.0 to ANIM_DURATION).
    /// - `revert`: If true, the tiles will swap back because no match was made.
    Swapping {
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
        timer: f32,
        revert: bool,
    },
    /// Empty spaces are being filled by tiles falling from above.
    /// - `timer`: Progress of the gravity step animation.
    Falling { timer: f32 },
    /// The timer has reached zero.
    GameOver,
}

/// Manages the 8x8 grid of animal tiles and the player's session state.
struct Board {
    /// The 8x8 grid. `None` represents an empty cell. `Some(u8)` is an animal ID (0-6).
    grid: [[Option<u8>; COLS]; ROWS],
    /// Current state (e.g., Idle, Swapping, Falling).
    state: GameState,
    /// Player's total score for the current session.
    score: u32,
    /// Remaining time in seconds.
    time_left: f32,
    /// The coordinates of the currently selected tile, if any.
    selected: Option<(usize, usize)>,
    /// Top 5 high scores loaded from local storage.
    high_scores: Vec<u32>,
    /// Flag to indicate if the current score is a new record.
    new_record: bool,
}

impl Board {
    /// Creates a new game board, fills it without starting matches, and loads high scores.
    fn new() -> Self {
        let mut board = Self {
            grid: [[None; COLS]; ROWS],
            state: GameState::Idle,
            score: 0,
            time_left: 60.0,
            selected: None,
            high_scores: Self::load_high_scores(),
            new_record: false,
        };
        board.fill_initial();
        board
    }

    /// Loads high scores from Macroquad's internal storage (which maps to localStorage on WASM).
    fn load_high_scores() -> Vec<u32> {
        let mut scores = vec![];
        for i in 0..MAX_HIGH_SCORES {
            if let Some(score) = storage::load::<u32>(&format!("score_{}", i)) {
                scores.push(score);
            }
        }
        scores.sort_by(|a, b| b.cmp(a));
        scores
    }

    /// Saves the current list of high scores to Macroquad's internal storage.
    fn save_high_scores(&self) {
        for (i, score) in self.high_scores.iter().enumerate() {
            storage::store(&format!("score_{}", i), score);
        }
    }

    /// Updates the leaderboard if the current score qualifies.
    fn update_leaderboard(&mut self) {
        if self.high_scores.iter().any(|&s| self.score > s) || self.high_scores.len() < MAX_HIGH_SCORES {
            self.new_record = self.high_scores.first().map_or(true, |&best| self.score > best);
            self.high_scores.push(self.score);
            self.high_scores.sort_by(|a, b| b.cmp(a));
            self.high_scores.truncate(MAX_HIGH_SCORES);
            self.save_high_scores();
        }
    }

    /// Randomly fills the entire grid ensuring no matches of 3+ exist at initialization.
    fn fill_initial(&mut self) {
        for y in 0..ROWS {
            for x in 0..COLS {
                loop {
                    let tile = (qrand::rand() % TILE_TYPES as u32) as u8;
                    self.grid[y][x] = Some(tile);
                    if !self.has_match_at(x, y) {
                        break;
                    }
                }
            }
        }
    }

    /// Checks if a tile at (x, y) is part of a 3-in-a-row match horizontally or vertically.
    fn has_match_at(&self, x: usize, y: usize) -> bool {
        let tile = self.grid[y][x];
        if tile.is_none() {
            return false;
        }

        // Horizontal
        let mut h_count = 1;
        let mut cx = x as i32 - 1;
        while cx >= 0 && self.grid[y][cx as usize] == tile {
            h_count += 1;
            cx -= 1;
        }
        cx = x as i32 + 1;
        while cx < COLS as i32 && self.grid[y][cx as usize] == tile {
            h_count += 1;
            cx += 1;
        }
        if h_count >= 3 {
            return true;
        }

        // Vertical
        let mut v_count = 1;
        let mut cy = y as i32 - 1;
        while cy >= 0 && self.grid[cy as usize][x] == tile {
            v_count += 1;
            cy -= 1;
        }
        cy = y as i32 + 1;
        while cy < ROWS as i32 && self.grid[cy as usize][x] == tile {
            v_count += 1;
            cy += 1;
        }
        if v_count >= 3 {
            return true;
        }

        false
    }

    /// Scans the entire board for all matching tiles and returns their coordinates.
    fn find_matches(&self) -> Vec<(usize, usize)> {
        let mut matches = vec![];
        for y in 0..ROWS {
            for x in 0..COLS {
                if self.has_match_at(x, y) {
                    matches.push((x, y));
                }
            }
        }
        matches
    }

    /// Removes all currently matching tiles, updates the score, and adds time to the clock.
    /// Returns true if any matches were cleared.
    fn clear_matches(&mut self) -> bool {
        let matches = self.find_matches();
        if matches.is_empty() {
            return false;
        }
        self.score += matches.len() as u32 * 10;
        self.time_left = (self.time_left + matches.len() as f32 * 0.5).min(60.0);
        for &(x, y) in &matches {
            self.grid[y][x] = None;
        }
        true
    }

    /// Performs a single gravity step: moves existing tiles down and spawns new tiles at the top.
    /// Returns true if any movement or spawning occurred.
    fn apply_gravity(&mut self) -> bool {
        let mut moved = false;
        for x in 0..COLS {
            for y in (1..ROWS).rev() {
                if self.grid[y][x].is_none() && self.grid[y - 1][x].is_some() {
                    self.grid[y][x] = self.grid[y - 1][x].take();
                    moved = true;
                }
            }
            if self.grid[0][x].is_none() {
                self.grid[0][x] = Some((qrand::rand() % TILE_TYPES as u32) as u8);
                moved = true;
            }
        }
        moved
    }

    /// Swaps the logical positions of two tiles in the grid.
    fn swap(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let tmp = self.grid[y1][x1];
        self.grid[y1][x1] = self.grid[y2][x2];
        self.grid[y2][x2] = tmp;
    }
}

/// Macroquad window configuration for high DPI and portrait-like aspect ratio.
fn window_conf() -> Conf {
    Conf {
        window_title: "Zookeeper WASM".to_owned(),
        window_width: 600,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

/// Entry point: Initializes assets, seeds RNG, and runs the main game loop.
#[macroquad::main(window_conf)]
async fn main() {
    qrand::srand(macroquad::miniquad::date::now() as _);

    // Animal textures (Twemoji 72x72) embedded into the binary.
    let textures = [
        Texture2D::from_file_with_format(include_bytes!("../assets/1f435.png"), None),
        Texture2D::from_file_with_format(include_bytes!("../assets/1f981.png"), None),
        Texture2D::from_file_with_format(include_bytes!("../assets/1f42f.png"), None),
        Texture2D::from_file_with_format(include_bytes!("../assets/1f418.png"), None),
        Texture2D::from_file_with_format(include_bytes!("../assets/1f992.png"), None),
        Texture2D::from_file_with_format(include_bytes!("../assets/1f43c.png"), None),
        Texture2D::from_file_with_format(include_bytes!("../assets/1f438.png"), None),
    ];

    for t in &textures {
        t.set_filter(FilterMode::Linear);
    }

    let mut board = Board::new();

    loop {
        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

        let sw = screen_width();
        let sh = screen_height();

        // Responsive layout calculations.
        let board_size = sw.min(sh * 0.8) * 0.95;
        let cell_size = board_size / COLS as f32;
        let offset_x = (sw - board_size) / 2.0;
        let offset_y = (sh - board_size) / 2.0 + (sh * 0.1);

        // Game Logic State Machine.
        if board.state != GameState::GameOver {
            board.time_left -= get_frame_time();
            if board.time_left <= 0.0 {
                board.state = GameState::GameOver;
                board.update_leaderboard();
            }
        }

        match board.state {
            GameState::Idle => {
                if is_mouse_button_pressed(MouseButton::Left) {
                    let (mx, my) = mouse_position();
                    if mx >= offset_x
                        && mx < offset_x + board_size
                        && my >= offset_y
                        && my < offset_y + board_size
                    {
                        let cx = ((mx - offset_x) / cell_size) as usize;
                        let cy = ((my - offset_y) / cell_size) as usize;

                        if let Some((sx, sy)) = board.selected {
                            let dx = (cx as i32 - sx as i32).abs();
                            let dy = (cy as i32 - sy as i32).abs();
                            // Only allow adjacent swaps.
                            if dx + dy == 1 {
                                board.swap(sx, sy, cx, cy);
                                let matches = board.find_matches();
                                let revert = matches.is_empty();
                                board.swap(cx, cy, sx, sy); // Swap back to animate properly.
                                board.state = GameState::Swapping {
                                    x1: sx,
                                    y1: sy,
                                    x2: cx,
                                    y2: cy,
                                    timer: 0.0,
                                    revert,
                                };
                            }
                            board.selected = None;
                        } else {
                            board.selected = Some((cx, cy));
                        }
                    } else {
                        board.selected = None;
                    }
                }
            }
            GameState::Swapping {
                x1,
                y1,
                x2,
                y2,
                mut timer,
                revert,
            } => {
                timer += get_frame_time();
                if timer >= ANIM_DURATION {
                    board.swap(x1, y1, x2, y2);
                    if revert {
                        board.swap(x1, y1, x2, y2); // No match found, swap back.
                        board.state = GameState::Idle;
                    } else {
                        board.clear_matches();
                        board.state = GameState::Falling { timer: 0.0 };
                    }
                } else {
                    board.state = GameState::Swapping {
                        x1,
                        y1,
                        x2,
                        y2,
                        timer,
                        revert,
                    };
                }
            }
            GameState::Falling { mut timer } => {
                timer += get_frame_time();
                if timer >= ANIM_DURATION / 2.0 {
                    if board.apply_gravity() {
                        board.state = GameState::Falling { timer: 0.0 };
                    } else if board.clear_matches() {
                        board.state = GameState::Falling { timer: 0.0 };
                    } else {
                        board.state = GameState::Idle;
                    }
                } else {
                    board.state = GameState::Falling { timer };
                }
            }
            GameState::GameOver => {
                if is_mouse_button_pressed(MouseButton::Left) {
                    board = Board::new();
                }
            }
        }

        // --- Rendering ---

        // Board Background
        draw_rectangle(offset_x, offset_y, board_size, board_size, Color::new(0.2, 0.2, 0.2, 1.0));

        // Highlight selected tile
        if let Some((sx, sy)) = board.selected {
            draw_rectangle(
                offset_x + sx as f32 * cell_size,
                offset_y + sy as f32 * cell_size,
                cell_size,
                cell_size,
                Color::new(1.0, 1.0, 1.0, 0.3),
            );
        }

        // Draw the tiles with animation offsets.
        for y in 0..ROWS {
            for x in 0..COLS {
                let mut draw_x = offset_x + x as f32 * cell_size;
                let mut draw_y = offset_y + y as f32 * cell_size;

                let tile = board.grid[y][x];

                if let GameState::Swapping {
                    x1,
                    y1,
                    x2,
                    y2,
                    timer,
                    revert,
                } = board.state
                {
                    let progress = timer / ANIM_DURATION;
                    let t = if revert {
                        (progress * std::f32::consts::PI).sin()
                    } else {
                        progress
                    };

                    if x == x1 && y == y1 {
                        draw_x += (x2 as f32 - x1 as f32) * cell_size * t;
                        draw_y += (y2 as f32 - y1 as f32) * cell_size * t;
                    } else if x == x2 && y == y2 {
                        draw_x += (x1 as f32 - x2 as f32) * cell_size * t;
                        draw_y += (y1 as f32 - y2 as f32) * cell_size * t;
                    }
                }

                if let Some(t_idx) = tile {
                    draw_texture_ex(
                        &textures[t_idx as usize],
                        draw_x + cell_size * 0.1,
                        draw_y + cell_size * 0.1,
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(cell_size * 0.8, cell_size * 0.8)),
                            ..Default::default()
                        },
                    );
                }
            }
        }

        // HUD: Score and Time.
        let font_size = sh * 0.05;
        draw_text(
            &format!("SCORE: {}", board.score),
            offset_x,
            offset_y - font_size,
            font_size,
            WHITE,
        );
        draw_text(
            &format!("TIME: {:.0}", board.time_left.max(0.0)),
            offset_x + board_size - font_size * 5.0,
            offset_y - font_size,
            font_size,
            WHITE,
        );

        // Game Over Overlay and Leaderboard.
        if board.state == GameState::GameOver {
            draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.8));
            
            let go_text = "GAME OVER";
            let tw = measure_text(go_text, None, font_size as _, 1.0).width;
            draw_text(go_text, sw / 2.0 - tw / 2.0, sh * 0.2, font_size, RED);

            if board.new_record {
                let rec_text = "NEW HIGH SCORE!";
                let rtw = measure_text(rec_text, None, (font_size * 0.8) as _, 1.0).width;
                draw_text(rec_text, sw / 2.0 - rtw / 2.0, sh * 0.28, font_size * 0.8, YELLOW);
            }

            draw_text("TOP SCORES:", sw * 0.3, sh * 0.4, font_size * 0.7, WHITE);
            for (i, score) in board.high_scores.iter().enumerate() {
                draw_text(
                    &format!("{}. {}", i + 1, score),
                    sw * 0.35,
                    sh * 0.45 + (i as f32 * font_size * 0.8),
                    font_size * 0.6,
                    if *score == board.score && board.score > 0 { YELLOW } else { GRAY },
                );
            }

            let tap_text = "Tap to restart";
            let ttw = measure_text(tap_text, None, (font_size * 0.6) as _, 1.0).width;
            draw_text(
                tap_text,
                sw / 2.0 - ttw / 2.0,
                sh * 0.85,
                font_size * 0.6,
                WHITE,
            );
        }

        next_frame().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that every cell in the grid is non-empty after initialization.
    #[test]
    fn test_board_initialization() {
        let board = Board::new();
        for y in 0..ROWS {
            for x in 0..COLS {
                assert!(board.grid[y][x].is_some(), "Every cell should be filled");
            }
        }
    }

    /// Verifies that the initial board generation algorithm correctly avoids matches.
    #[test]
    fn test_initial_board_has_no_matches() {
        let board = Board::new();
        assert!(board.find_matches().is_empty(), "Initial board should not have matches");
    }

    /// Tests horizontal match detection with a forced 3-in-a-row scenario.
    #[test]
    fn test_match_detection_horizontal() {
        let mut board = Board {
            grid: [[Some(10); COLS]; ROWS],
            state: GameState::Idle,
            score: 0,
            time_left: 60.0,
            selected: None,
            high_scores: vec![],
            new_record: false,
        };
        // Set unique types to prevent accidental matches.
        for y in 0..ROWS {
            for x in 0..COLS {
                board.grid[y][x] = Some(((y * COLS + x) % 10 + 10) as u8);
            }
        }
        
        // Create horizontal match.
        board.grid[0][0] = Some(1);
        board.grid[0][1] = Some(1);
        board.grid[0][2] = Some(1);
        
        let matches = board.find_matches();
        assert!(!matches.is_empty(), "Matches should be found");
        assert!(matches.contains(&(0, 0)));
        assert!(matches.contains(&(1, 0)));
        assert!(matches.contains(&(2, 0)));
    }
}
