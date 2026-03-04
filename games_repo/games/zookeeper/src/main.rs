//! Zookeeper WASM: A 60 FPS Match-3 Clone in Rust
//!
//! This module provides a fully self-contained match-3 game using the Macroquad engine.
//! It handles the 8x8 game board, animal matching logic, animations, high scores, persistent settings, and combo systems.

use macroquad::audio::{load_sound_from_bytes, play_sound, PlaySoundParams};
use macroquad::prelude::*;
use macroquad::prelude::collections::storage;
use quad_rand as qrand;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize, Clone)]
struct LeaderboardEntry {
    name: String,
    score: u32,
    combo: u32,
}

#[cfg(target_arch = "wasm32")]
extern "C" {
    fn js_load_leaderboard(ptr: *mut u8, max_len: u32) -> u32;
    fn js_save_leaderboard(ptr: *const u8, len: u32);
    fn js_ask_name(ptr: *mut u8, max_len: u32) -> u32;
}

#[cfg(not(target_arch = "wasm32"))]
unsafe fn js_load_leaderboard(_ptr: *mut u8, _max_len: u32) -> u32 { 0 }
#[cfg(not(target_arch = "wasm32"))]
unsafe fn js_save_leaderboard(_ptr: *const u8, _len: u32) { }
#[cfg(not(target_arch = "wasm32"))]
unsafe fn js_ask_name(_ptr: *mut u8, _max_len: u32) -> u32 { 0 }

/// Persistent user settings.
struct Settings {
    muted: bool,
    slow_mode: bool,
}

/// Represents the current state of the game loop and any active animations.
#[derive(Clone, PartialEq, Debug)]
enum GameState {
    /// The game is waiting for player input.
    Idle,
    /// Two tiles are in the process of being swapped.
    Swapping {
        x1: usize,
        y1: usize,
        x2: usize,
        y2: usize,
        timer: f32,
        revert: bool,
    },
    /// Tiles that formed a match are "popping" or fading before disappearing.
    Clearing {
        timer: f32,
        matches: [(usize, usize); COLS * ROWS], // Simplified match tracking for animation
        match_count: usize,
    },
    /// Empty spaces are being filled by tiles falling from above.
    Falling { timer: f32 },
    /// No legal moves are available on the board.
    NoMoreMoves { timer: f32 },
    /// New High Score name entry.
    EnteringName {
        score: u32,
        combo: u32,
        name: String,
    },
    /// The board is being refilled after a shuffle or level up.
    Reshuffling {
        target_grid: [[u8; COLS]; ROWS],
        next_row: usize,
        timer: f32,
    },
    /// Celebration for clearing a level.
    LevelUp { timer: f32 },
    /// The timer has reached zero.
    GameOver,
    /// The game is manually paused.
    Paused { previous_state: Box<GameState> },
}

/// Manages the 8x8 grid of animal tiles and the player's session state.
struct Board {
    grid: [[Option<u8>; COLS]; ROWS],
    state: GameState,
    score: u32,
    time_left: f32,
    selected: Option<(usize, usize)>,
    high_scores: Vec<LeaderboardEntry>,
    new_record: bool,
    combo_count: u32,
    max_combo: u32,
    level: u32,
    level_tiles_cleared: u32,
    level_goal: u32,
}

/// Rules for board generation to allow future tweaks.
struct GenerationRules {
    min_initial_moves: usize,
    max_attempts: usize,
}

impl Default for GenerationRules {
    fn default() -> Self {
        Self {
            min_initial_moves: 3,
            max_attempts: 100,
        }
    }
}

impl Board {
    fn new() -> Self {
        let mut board = Self {
            grid: [[None; COLS]; ROWS],
            state: GameState::Idle,
            score: 0,
            time_left: 60.0,
            selected: None,
            high_scores: Self::load_high_scores(),
            new_record: false,
            combo_count: 0,
            max_combo: 0,
            level: 1,
            level_tiles_cleared: 0,
            level_goal: 50,
        };
        board.fill_initial(GenerationRules::default());
        board
    }

    fn load_high_scores() -> Vec<LeaderboardEntry> {
        let mut buffer = [0u8; 4096];
        let len = unsafe { js_load_leaderboard(buffer.as_mut_ptr(), buffer.len() as u32) };
        if len == 0 {
            return vec![LeaderboardEntry { name: "---".to_string(), score: 0, combo: 0 }; MAX_HIGH_SCORES];
        }
        
        let json_str = String::from_utf8_lossy(&buffer[..len as usize]);
        serde_json::from_str(&json_str).unwrap_or_else(|_| {
            vec![LeaderboardEntry { name: "---".to_string(), score: 0, combo: 0 }; MAX_HIGH_SCORES]
        })
    }

    fn qualifies_for_leaderboard(&self) -> bool {
        self.high_scores.iter().any(|e| self.score > e.score) || self.high_scores.len() < MAX_HIGH_SCORES
    }

    fn add_to_leaderboard(&mut self, name: String, score: u32, combo: u32) {
        let name = if name.trim().is_empty() { "ANON".to_string() } else { name.trim().to_string() };
        self.new_record = self.high_scores.first().map_or(true, |best| score > best.score);
        
        // Add locally first
        self.high_scores.push(LeaderboardEntry { name, score, combo });
        self.high_scores.sort_by(|a, b| b.score.cmp(&a.score));
        self.high_scores.truncate(MAX_HIGH_SCORES);

        // Save to JS
        if let Ok(json_str) = serde_json::to_string(&self.high_scores) {
            unsafe { js_save_leaderboard(json_str.as_ptr(), json_str.len() as u32) };
        }
    }

    fn fill_initial(&mut self, rules: GenerationRules) {
        let mut attempts = 0;
        loop {
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
            
            // Ensure solvability with a "nudge" towards a better starting field.
            let target_moves = if attempts < 10 { rules.min_initial_moves } else { 1 };
            if self.count_available_moves() >= target_moves {
                break;
            }
            attempts += 1;
            if attempts > rules.max_attempts {
                break;
            }
        }
    }

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
        v_count >= 3
    }

    fn find_matches(&self) -> Vec<(usize, usize)> {
        let mut matches = Vec::new();
        for y in 0..ROWS {
            for x in 0..COLS {
                if self.has_match_at(x, y) {
                    matches.push((x, y));
                }
            }
        }
        matches
    }

    fn count_available_moves(&mut self) -> usize {
        let mut moves = 0;
        for y in 0..ROWS {
            for x in 0..COLS {
                // Try swapping right
                if x < COLS - 1 {
                    let temp = self.grid[y][x];
                    self.grid[y][x] = self.grid[y][x+1];
                    self.grid[y][x+1] = temp;
                    if self.has_match_at(x, y) || self.has_match_at(x + 1, y) {
                        moves += 1;
                    }
                    let temp = self.grid[y][x];
                    self.grid[y][x] = self.grid[y][x+1];
                    self.grid[y][x+1] = temp;
                }
                // Try swapping down
                if y < ROWS - 1 {
                    let temp = self.grid[y][x];
                    self.grid[y][x] = self.grid[y+1][x];
                    self.grid[y+1][x] = temp;
                    if self.has_match_at(x, y) || self.has_match_at(x, y + 1) {
                        moves += 1;
                    }
                    let temp = self.grid[y][x];
                    self.grid[y][x] = self.grid[y+1][x];
                    self.grid[y+1][x] = temp;
                }
            }
        }
        moves
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Zookeeper WASM".to_owned(),
        window_width: 600,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

/// Centered text drawing helper
fn draw_text_centered(text: &str, y: f32, size: f32, color: Color) {
    let sw = screen_width();
    let dims = measure_text(text, None, size as u16, 1.0);
    draw_text(text, sw / 2.0 - dims.width / 2.0, y, size, color);
}

#[macroquad::main(window_conf)]
async fn main() {
    qrand::srand(macroquad::miniquad::date::now() as _);

    // Initialize settings storage
    storage::store(Settings { muted: false, slow_mode: false });

    let textures = [
        Texture2D::from_file_with_format(include_bytes!("../assets/1f435.png"), None),
        Texture2D::from_file_with_format(include_bytes!("../assets/1f427.png"), None),
        Texture2D::from_file_with_format(include_bytes!("../assets/1f42f.png"), None),
        Texture2D::from_file_with_format(include_bytes!("../assets/1f418.png"), None),
        Texture2D::from_file_with_format(include_bytes!("../assets/1f992.png"), None),
        Texture2D::from_file_with_format(include_bytes!("../assets/1f43c.png"), None),
        Texture2D::from_file_with_format(include_bytes!("../assets/1f438.png"), None),
    ];

    let tex_mute_on = Texture2D::from_file_with_format(include_bytes!("../assets/1f507.png"), None);
    let tex_mute_off = Texture2D::from_file_with_format(include_bytes!("../assets/1f50a.png"), None);
    let tex_snail = Texture2D::from_file_with_format(include_bytes!("../assets/1f40c.png"), None);
    let tex_pause = Texture2D::from_file_with_format(include_bytes!("../assets/23f8.png"), None);
    let tex_play = Texture2D::from_file_with_format(include_bytes!("../assets/25b6.png"), None);

    for t in &textures { t.set_filter(FilterMode::Linear); }
    tex_mute_on.set_filter(FilterMode::Linear);
    tex_mute_off.set_filter(FilterMode::Linear);
    tex_snail.set_filter(FilterMode::Linear);
    tex_pause.set_filter(FilterMode::Linear);
    tex_play.set_filter(FilterMode::Linear);

    // Generate software sounds
    let snd_swap = load_sound_from_bytes(&create_wav(440.0, 0.1, 0.5)).await.ok();
    let snd_fall = load_sound_from_bytes(&create_wav(220.0, 0.05, 0.3)).await.ok();
    let snd_game_over = load_sound_from_bytes(&create_wav(110.0, 0.5, 0.5)).await.ok();
    let snd_level_up = load_sound_from_bytes(&create_wav(880.0, 0.3, 0.5)).await.ok();
    let snd_reshuffle = load_sound_from_bytes(&create_wav(660.0, 0.2, 0.5)).await.ok();
    
    let mut snd_matches = Vec::new();
    for i in 0..10 {
        let freq = 550.0 + (i as f32 * 110.0);
        snd_matches.push(load_sound_from_bytes(&create_wav(freq, 0.15, 0.5)).await.ok());
    }

    let mut board = Board::new();

    loop {
        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

        let sw = screen_width();
        let sh = screen_height();
        let board_size = sw.min(sh * 0.8) * 0.95;
        let cell_size = board_size / COLS as f32;
        let offset_x = (sw - board_size) / 2.0;
        let offset_y = (sh - board_size) / 2.0 + (sh * 0.05);

        let mut settings = storage::get_mut::<Settings>();
        let dt = if settings.slow_mode { get_frame_time() * 0.3 } else { get_frame_time() };

        // Game Logic State Machine.
        let is_playing = !matches!(board.state, GameState::GameOver | GameState::Paused { .. } | GameState::EnteringName { .. } | GameState::LevelUp { .. } | GameState::NoMoreMoves { .. });
        
        if is_playing {
            board.time_left -= dt;
            if board.time_left <= 0.0 {
                if board.qualifies_for_leaderboard() {
                    board.state = GameState::EnteringName { score: board.score, combo: board.max_combo, name: "".to_string() };
                } else {
                    board.state = GameState::GameOver;
                }
                if !settings.muted {
                    if let Some(ref snd) = snd_game_over { play_sound(snd, PlaySoundParams::default()); }
                }
            }
        }

        // --- UI Buttons ---
        let pad = 10.0;
        let btn_size = sh * 0.06;
        let (mx, my) = mouse_position();
        let mute_x = sw - btn_size - pad;
        let mute_y = pad;
        let over_mute = mx >= mute_x - pad && mx <= sw && my >= 0.0 && my <= mute_y + btn_size + pad;
        // Pause button
        let pause_x = mute_x - btn_size - pad;
        let pause_y = pad;
        let over_pause = mx >= pause_x - pad && mx <= mute_x && my >= 0.0 && my <= pause_y + btn_size + pad;
        // Snail button
        let snail_x = pause_x - btn_size - pad;
        let snail_y = pad;
        let over_snail = mx >= snail_x - pad && mx <= pause_x && my >= 0.0 && my <= snail_y + btn_size + pad;

        match board.state {
            GameState::Paused { .. } => {
                if is_key_pressed(KeyCode::Space) || (is_mouse_button_pressed(MouseButton::Left) && over_pause) {
                    if let GameState::Paused { previous_state } = board.state {
                        board.state = *previous_state;
                    }
                }
            }
            _ => {
                if is_key_pressed(KeyCode::Space) || (is_mouse_button_pressed(MouseButton::Left) && over_pause) {
                    board.state = GameState::Paused { previous_state: Box::new(board.state.clone()) };
                }
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            if over_mute { settings.muted = !settings.muted; }
            if over_snail { settings.slow_mode = !settings.slow_mode; }
        }

        // Logic
        match board.state {
            GameState::Idle => {
                if is_mouse_button_pressed(MouseButton::Left) && !over_mute && !over_pause && !over_snail {
                    let gx = ((mx - offset_x) / cell_size) as i32;
                    let gy = ((my - offset_y) / cell_size) as i32;
                    if gx >= 0 && gx < COLS as i32 && gy >= 0 && gy < ROWS as i32 {
                        let gx = gx as usize;
                        let gy = gy as usize;
                        if let Some((sx, sy)) = board.selected {
                            if (gx == sx && (gy == sy + 1 || gy == sy.saturating_sub(1))) ||
                               (gy == sy && (gx == sx + 1 || gx == sx.saturating_sub(1))) {
                                board.state = GameState::Swapping { x1: sx, y1: sy, x2: gx, y2: gy, timer: 0.0, revert: false };
                                board.selected = None;
                                if !settings.muted {
                                    if let Some(ref snd) = snd_swap { play_sound(snd, PlaySoundParams::default()); }
                                }
                            } else {
                                board.selected = Some((gx, gy));
                            }
                        } else {
                            board.selected = Some((gx, gy));
                        }
                    } else {
                        board.selected = None;
                    }
                }
            }
            GameState::Swapping { x1, y1, x2, y2, mut timer, revert } => {
                timer += dt;
                if timer >= ANIM_DURATION {
                    let t1 = board.grid[y1][x1];
                    board.grid[y1][x1] = board.grid[y2][x2];
                    board.grid[y2][x2] = t1;

                    if revert {
                        board.state = GameState::Idle;
                    } else {
                        let matches = board.find_matches();
                        if matches.is_empty() {
                            board.state = GameState::Swapping { x1, y1, x2, y2, timer: 0.0, revert: true };
                        } else {
                            let mut match_arr = [(0, 0); COLS * ROWS];
                            for (i, m) in matches.iter().enumerate() { match_arr[i] = *m; }
                            board.state = GameState::Clearing { timer: 0.0, matches: match_arr, match_count: matches.len() };
                            board.combo_count = 1;
                        }
                    }
                } else {
                    board.state = GameState::Swapping { x1, y1, x2, y2, timer, revert };
                }
            }
            GameState::Clearing { mut timer, matches, match_count } => {
                timer += dt;
                if timer >= ANIM_DURATION {
                    for i in 0..match_count {
                        let (mx, my) = matches[i];
                        board.grid[my][mx] = None;
                    }
                    let points = (match_count as u32 * 10) * board.combo_count;
                    board.score += points;
                    board.level_tiles_cleared += match_count as u32;
                    board.time_left = (board.time_left + (match_count as f32 * 0.5)).min(60.0);
                    board.state = GameState::Falling { timer: 0.0 };
                    if !settings.muted {
                        let snd_idx = (board.combo_count as usize).min(snd_matches.len() - 1);
                        if let Some(ref snd) = snd_matches[snd_idx] { play_sound(snd, PlaySoundParams::default()); }
                    }
                } else {
                    board.state = GameState::Clearing { timer, matches, match_count };
                }
            }
            GameState::Falling { mut timer } => {
                timer += dt;
                if timer >= 0.1 {
                    let mut moved = false;
                    for x in 0..COLS {
                        for y in (1..ROWS).rev() {
                            if board.grid[y][x].is_none() && board.grid[y - 1][x].is_some() {
                                board.grid[y][x] = board.grid[y - 1][x];
                                board.grid[y - 1][x] = None;
                                moved = true;
                            }
                        }
                        if board.grid[0][x].is_none() {
                            board.grid[0][x] = Some((qrand::rand() % TILE_TYPES as u32) as u8);
                            moved = true;
                        }
                    }
                    if !moved {
                        let matches = board.find_matches();
                        if matches.is_empty() {
                            if board.level_tiles_cleared >= board.level_goal {
                                board.state = GameState::LevelUp { timer: 0.0 };
                                if !settings.muted {
                                    if let Some(ref snd) = snd_level_up { play_sound(snd, PlaySoundParams::default()); }
                                }
                            } else if board.count_available_moves() == 0 {
                                board.state = GameState::NoMoreMoves { timer: 0.0 };
                            } else {
                                board.state = GameState::Idle;
                                board.combo_count = 0;
                            }
                        } else {
                            let mut match_arr = [(0, 0); COLS * ROWS];
                            for (i, m) in matches.iter().enumerate() { match_arr[i] = *m; }
                            board.state = GameState::Clearing { timer: 0.0, matches: match_arr, match_count: matches.len() };
                            board.combo_count += 1;
                            board.max_combo = board.max_combo.max(board.combo_count);
                        }
                    } else {
                        board.state = GameState::Falling { timer: 0.0 };
                        if !settings.muted {
                            if let Some(ref snd) = snd_fall { play_sound(snd, PlaySoundParams::default()); }
                        }
                    }
                } else {
                    board.state = GameState::Falling { timer };
                }
            }
            GameState::NoMoreMoves { mut timer } => {
                timer += dt;
                if timer >= 1.5 {
                    let mut target = [[0u8; COLS]; ROWS];
                    let mut attempts = 0;
                    loop {
                        for y in 0..ROWS {
                            for x in 0..COLS {
                                loop {
                                    let tile = (qrand::rand() % TILE_TYPES as u32) as u8;
                                    target[y][x] = tile;
                                    let mut temp_board = [[None; COLS]; ROWS];
                                    for ty in 0..ROWS { for tx in 0..COLS { temp_board[ty][tx] = Some(target[ty][tx]); } }
                                    if !has_match_static(&temp_board, x, y) { break; }
                                }
                            }
                        }
                        if count_available_moves_static(&target) >= 3 { break; }
                        attempts += 1; if attempts > 100 { break; }
                    }
                    board.grid = [[None; COLS]; ROWS];
                    board.state = GameState::Reshuffling { target_grid: target, next_row: ROWS, timer: 0.0 };
                    if !settings.muted {
                        if let Some(ref snd) = snd_reshuffle { play_sound(snd, PlaySoundParams::default()); }
                    }
                } else { board.state = GameState::NoMoreMoves { timer }; }
            }
            GameState::Reshuffling { target_grid, next_row, mut timer } => {
                timer += dt;
                if timer >= 0.05 {
                    if next_row > 0 {
                        let r = next_row - 1;
                        for x in 0..COLS { board.grid[r][x] = Some(target_grid[r][x]); }
                        board.state = GameState::Reshuffling { target_grid, next_row: r, timer: 0.0 };
                    } else { board.state = GameState::Idle; }
                } else { board.state = GameState::Reshuffling { target_grid, next_row, timer }; }
            }
            GameState::LevelUp { mut timer } => {
                timer += dt;
                if timer >= 2.0 {
                    board.level += 1;
                    board.level_tiles_cleared = 0;
                    board.level_goal += 25;
                    board.time_left = 60.0;
                    let mut target = [[0u8; COLS]; ROWS];
                    loop {
                        for y in 0..ROWS { for x in 0..COLS { loop {
                            let tile = (qrand::rand() % TILE_TYPES as u32) as u8;
                            target[y][x] = tile;
                            let mut temp_board = [[None; COLS]; ROWS];
                            for ty in 0..ROWS { for tx in 0..COLS { temp_board[ty][tx] = Some(target[ty][tx]); } }
                            if !has_match_static(&temp_board, x, y) { break; }
                        } } }
                        if count_available_moves_static(&target) >= 3 { break; }
                    }
                    board.grid = [[None; COLS]; ROWS];
                    board.state = GameState::Reshuffling { target_grid: target, next_row: ROWS, timer: 0.0 };
                } else { board.state = GameState::LevelUp { timer }; }
            }
            GameState::EnteringName { score, combo, ref name } => {
                let mut submitted = false;
                let mut current_name = name.clone();
                while let Some(c) = get_char_pressed() {
                    if (c.is_alphanumeric() || c == ' ') && current_name.len() < 10 {
                        current_name.push(c);
                    }
                }
                if is_key_pressed(KeyCode::Backspace) { current_name.pop(); }

                let ok_w = sw * 0.3;
                let ok_x = sw / 2.0 - ok_w / 2.0;
                let ok_y = sh * 0.7;
                let ok_h = sh * 0.1;
                let _font_size = sh * 0.05;

                let prompt_w = sw * 0.4;
                let _prompt_x = sw / 2.0 - prompt_w / 2.0;
                let _prompt_y = sh * 0.6;
                let _prompt_h = sh * 0.06;

                #[cfg(target_arch = "wasm32")]
                {
                    draw_rectangle(_prompt_x, _prompt_y, prompt_w, _prompt_h, Color::new(0.2, 0.2, 0.2, 1.0));
                    draw_text_centered("TAP FOR POPUP", _prompt_y + _prompt_h * 0.7, _font_size * 0.4, WHITE);
                    if is_mouse_button_pressed(MouseButton::Left) && mx >= _prompt_x && mx <= _prompt_x + prompt_w && my >= _prompt_y && my <= _prompt_y + _prompt_h {
                        let mut buffer = [0u8; 16];
                        let len = unsafe { js_ask_name(buffer.as_mut_ptr(), buffer.len() as u32) };
                        if len > 0 {
                            if let Ok(js_name) = std::str::from_utf8(&buffer[..len as usize]) {
                                current_name = js_name.trim().to_string();
                            }
                        }
                    }
                }

                if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter) {
                    board.add_to_leaderboard(current_name.clone(), score, combo);
                    board.state = GameState::GameOver;
                    submitted = true;
                }
                if !submitted && is_mouse_button_pressed(MouseButton::Left) {
                    if mx >= ok_x && mx <= ok_x + ok_w && my >= ok_y && my <= ok_y + ok_h {
                        board.add_to_leaderboard(current_name.clone(), score, combo);
                        board.state = GameState::GameOver;
                        submitted = true;
                    }
                }
                if !submitted {
                    board.state = GameState::EnteringName { score, combo, name: current_name };
                }
            }
            GameState::GameOver => {
                if is_mouse_button_pressed(MouseButton::Left) && !over_mute && !over_pause && !over_snail {
                    board = Board::new();
                    board.state = GameState::Idle;
                }
            }
            GameState::Paused { .. } => {}
        }

        // Draw
        for y in 0..ROWS {
            for x in 0..COLS {
                let mut draw_x = offset_x + x as f32 * cell_size;
                let mut draw_y = offset_y + y as f32 * cell_size;
                let mut alpha = 1.0;
                let mut scale = 1.0;

                match board.state {
                    GameState::Swapping { x1, y1, x2, y2, timer, .. } => {
                        let t = timer / ANIM_DURATION;
                        if x == x1 && y == y1 {
                            draw_x += (x2 as f32 - x1 as f32) * cell_size * t;
                            draw_y += (y2 as f32 - y1 as f32) * cell_size * t;
                        } else if x == x2 && y == y2 {
                            draw_x += (x1 as f32 - x2 as f32) * cell_size * t;
                            draw_y += (y1 as f32 - y2 as f32) * cell_size * t;
                        }
                    }
                    GameState::Clearing { timer, ref matches, match_count } => {
                        for i in 0..match_count {
                            let (mx, my) = matches[i];
                            if x == mx && y == my {
                                let t = timer / ANIM_DURATION;
                                scale = 1.0 + (t * (0.5 + (board.combo_count as f32 * 0.1)));
                                alpha = 1.0 - t;
                                if board.combo_count > 1 {
                                    draw_x += qrand::gen_range(-2.0, 2.0) * board.combo_count as f32;
                                    draw_y += qrand::gen_range(-2.0, 2.0) * board.combo_count as f32;
                                }
                            }
                        }
                    }
                    _ => {}
                }
                if let Some(t_idx) = board.grid[y][x] {
                    let actual_cell = cell_size * scale;
                    let pad = cell_size * 0.1;
                    draw_texture_ex(
                        &textures[t_idx as usize],
                        draw_x + cell_size / 2.0 - actual_cell / 2.0 + pad,
                        draw_y + cell_size / 2.0 - actual_cell / 2.0 + pad,
                        Color::new(1.0, 1.0, 1.0, alpha),
                        DrawTextureParams { dest_size: Some(vec2(actual_cell - pad * 2.0, actual_cell - pad * 2.0)), ..Default::default() },
                    );
                }
            }
        }

        // --- HUD & Bars ---
        let font_size = sh * 0.05;
        let bar_w = board_size;
        let bar_h = 12.0;
        
        let line1_y = offset_y - 10.0;
        let line2_y = line1_y - font_size * 0.8;
        let time_bar_y = line2_y - font_size * 0.8 - 15.0;

        let time_progress = (board.time_left / 60.0).clamp(0.0, 1.0);
        let mut time_color = RED;
        if board.time_left < 10.0 {
            let blink_speed = if board.time_left < 5.0 { 15.0 } else { 8.0 };
            if (get_time() * blink_speed) as i32 % 2 == 0 { time_color = WHITE; }
        }
        draw_rectangle(offset_x, time_bar_y, bar_w, bar_h, Color::new(0.3, 0.1, 0.1, 1.0));
        draw_rectangle(offset_x, time_bar_y, bar_w * time_progress, bar_h, time_color);
        draw_text("TIME", offset_x, time_bar_y - 5.0, font_size * 0.4, time_color);

        draw_text(&format!("SCORE: {}", board.score), offset_x, line2_y, font_size * 0.8, WHITE);
        draw_text(&format!("MAX COMBO: X{}", board.max_combo), offset_x, line1_y, font_size * 0.6, YELLOW);

        if let Some((sx, sy)) = board.selected {
            draw_rectangle_lines(offset_x + sx as f32 * cell_size, offset_y + sy as f32 * cell_size, cell_size, cell_size, 4.0, YELLOW);
        }

        draw_texture_ex(if settings.muted { &tex_mute_on } else { &tex_mute_off }, mute_x, mute_y, WHITE, DrawTextureParams { dest_size: Some(vec2(btn_size, btn_size)), ..Default::default() });
        draw_texture_ex(if matches!(board.state, GameState::Paused { .. }) { &tex_play } else { &tex_pause }, pause_x, pause_y, WHITE, DrawTextureParams { dest_size: Some(vec2(btn_size, btn_size)), ..Default::default() });
        draw_texture_ex(&tex_snail, snail_x, snail_y, if settings.slow_mode { WHITE } else { Color::new(1.0, 1.0, 1.0, 0.3) }, DrawTextureParams { dest_size: Some(vec2(btn_size, btn_size)), ..Default::default() });

        if let GameState::Paused { .. } = board.state {
            draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.5));
            draw_text_centered("PAUSED", sh / 2.0, font_size * 2.0, WHITE);
        }

        if let GameState::GameOver = board.state {
            draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.8));
            draw_text_centered("GAME OVER", sh * 0.3, font_size * 2.0, RED);
            draw_text_centered(&format!("FINAL SCORE: {}", board.score), sh * 0.45, font_size, WHITE);
            draw_text_centered("LEADERBOARD", sh * 0.55, font_size * 0.8, YELLOW);
            for (i, entry) in board.high_scores.iter().enumerate() {
                let y = sh * 0.62 + (i as f32 * font_size * 0.8);
                draw_text_centered(&format!("{}. {} - {}", i+1, entry.name, entry.score), y, font_size * 0.6, WHITE);
            }
            if (get_time() * 2.0) as i32 % 2 == 0 {
                draw_text_centered("TAP TO RESTART", sh * 0.9, font_size * 0.7, YELLOW);
            }
        }

        if let GameState::NoMoreMoves { .. } = board.state {
            draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.5));
            draw_text_centered("NO MORE MOVES", sh / 2.0, font_size, YELLOW);
            draw_text_centered("RESHUFFLING...", sh / 2.0 + font_size, font_size * 0.6, WHITE);
        }

        if let GameState::LevelUp { .. } = board.state {
            draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.5));
            draw_text_centered(&format!("LEVEL {} CLEAR!", board.level), sh / 2.0, font_size * 1.5, GREEN);
            draw_text_centered("GET READY...", sh / 2.0 + font_size, font_size * 0.6, WHITE);
        }

        if let GameState::EnteringName { score, combo, name } = &board.state {
            draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.9));
            draw_text_centered("NEW HIGH SCORE!", sh * 0.15, font_size, YELLOW);
            let stats = format!("SCORE: {}  COMBO: X{}", score, combo);
            draw_text_centered(&stats, sh * 0.25, font_size * 0.6, WHITE);
            draw_text_centered("Type your name", sh * 0.35, font_size * 0.6, GRAY);
            let display_name = if name.is_empty() { "_".to_string() } else { format!("{}_", name) };
            draw_text_centered(&display_name, sh * 0.5, font_size, WHITE);

            #[cfg(target_arch = "wasm32")]
            {
                let prompt_w = sw * 0.4;
                let prompt_x = sw / 2.0 - prompt_w / 2.0;
                let prompt_y = sh * 0.6;
                let prompt_h = sh * 0.06;
                draw_rectangle(prompt_x, prompt_y, prompt_w, prompt_h, Color::new(0.2, 0.2, 0.2, 1.0));
                draw_text_centered("TAP FOR POPUP", prompt_y + prompt_h * 0.7, font_size * 0.4, WHITE);
            }

            let ok_text = "OK";
            let ok_w = sw * 0.3;
            let ok_x = sw / 2.0 - ok_w / 2.0;
            let ok_y = sh * 0.7;
            draw_rectangle(ok_x, ok_y, ok_w, sh * 0.1, Color::new(0.3, 0.8, 0.3, 1.0));
            draw_text_centered(ok_text, ok_y + sh * 0.07, font_size, WHITE);
        }

        next_frame().await
    }
}

fn has_match_static(grid: &[[Option<u8>; COLS]; ROWS], x: usize, y: usize) -> bool {
    let tile = grid[y][x]; if tile.is_none() { return false; }
    let mut h_count = 1; let mut cx = x as i32 - 1;
    while cx >= 0 && grid[y][cx as usize] == tile { h_count += 1; cx -= 1; }
    cx = x as i32 + 1; while cx < COLS as i32 && grid[y][cx as usize] == tile { h_count += 1; cx += 1; }
    if h_count >= 3 { return true; }
    let mut v_count = 1; let mut cy = y as i32 - 1;
    while cy >= 0 && grid[cy as usize][x] == tile { v_count += 1; cy -= 1; }
    cy = y as i32 + 1; while cy < ROWS as i32 && grid[cy as usize][x] == tile { v_count += 1; cy += 1; }
    v_count >= 3
}

fn count_available_moves_static(grid: &[[u8; COLS]; ROWS]) -> usize {
    let mut moves = 0; let mut temp = [[None; COLS]; ROWS];
    for y in 0..ROWS { for x in 0..COLS { temp[y][x] = Some(grid[y][x]); } }
    for y in 0..ROWS { for x in 0..COLS {
        if x < COLS - 1 {
            let t = temp[y][x]; temp[y][x] = temp[y][x+1]; temp[y][x+1] = t;
            if has_match_static(&temp, x, y) || has_match_static(&temp, x+1, y) { moves += 1; }
            let t = temp[y][x]; temp[y][x] = temp[y][x+1]; temp[y][x+1] = t;
        }
        if y < ROWS - 1 {
            let t = temp[y][x]; temp[y][x] = temp[y+1][x]; temp[y+1][x] = t;
            if has_match_static(&temp, x, y) || has_match_static(&temp, x, y+1) { moves += 1; }
            let t = temp[y][x]; temp[y][x] = temp[y+1][x]; temp[y+1][x] = t;
        }
    } }
    moves
}

fn create_wav(freq: f32, duration: f32, volume: f32) -> Vec<u8> {
    let sample_rate = 44100u32; let num_samples = (duration * sample_rate as f32) as u32;
    let data_size = num_samples * 2; let mut wav = Vec::with_capacity(44 + data_size as usize);
    wav.extend_from_slice(b"RIFF"); wav.extend_from_slice(&(36 + data_size).to_le_bytes());
    wav.extend_from_slice(b"WAVEfmt "); wav.extend_from_slice(&16u32.to_le_bytes());
    wav.extend_from_slice(&1u16.to_le_bytes()); wav.extend_from_slice(&1u16.to_le_bytes());
    wav.extend_from_slice(&sample_rate.to_le_bytes()); wav.extend_from_slice(&(sample_rate * 2).to_le_bytes());
    wav.extend_from_slice(&2u16.to_le_bytes()); wav.extend_from_slice(&16u16.to_le_bytes());
    wav.extend_from_slice(b"data"); wav.extend_from_slice(&data_size.to_le_bytes());
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let sample = (t * freq * 2.0 * std::f32::consts::PI).sin();
        let amplitude = volume * (1.0 - (i as f32 / num_samples as f32));
        wav.extend_from_slice(&((sample * amplitude * 32767.0) as i16).to_le_bytes());
    }
    wav
}
