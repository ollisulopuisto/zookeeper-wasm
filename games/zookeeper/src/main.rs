//! Zookeeper WASM: A 60 FPS Match-3 Clone in Rust
//!
//! This module provides a fully self-contained match-3 game using the Macroquad engine.
//! It handles the 8x8 game board, animal matching logic, animations, high scores, persistent settings, and combo systems.

use macroquad::audio::{load_sound_from_bytes, play_sound, PlaySoundParams};
use macroquad::prelude::*;
use macroquad::prelude::collections::storage;
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

/// Leaderboard data stored in global storage.
struct Leaderboard {
    entries: Vec<(String, u32)>,
}

/// Persistent user settings.
struct Settings {
    muted: bool,
}

/// Represents the current state of the game loop and any active animations.
#[derive(Clone, PartialEq, Debug)]
enum GameState {
    /// Waiting for initial tap to start (essential for iOS audio context).
    WaitingToStart,
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
    /// New High Score name entry.
    EnteringName {
        score: u32,
        initials: [char; 3],
        active_index: usize,
    },
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
    high_scores: Vec<(String, u32)>,
    new_record: bool,
    combo_count: u32,
}

impl Board {
    fn new() -> Self {
        let mut board = Self {
            grid: [[None; COLS]; ROWS],
            state: GameState::WaitingToStart,
            score: 0,
            time_left: 60.0,
            selected: None,
            high_scores: Self::load_high_scores(),
            new_record: false,
            combo_count: 0,
        };
        board.fill_initial();
        board
    }

    fn load_high_scores() -> Vec<(String, u32)> {
        if let Some(lb) = storage::get_mut::<Leaderboard>() {
            lb.entries.clone()
        } else {
            let initial = vec![("---".to_string(), 0); MAX_HIGH_SCORES];
            storage::store(Leaderboard { entries: initial.clone() });
            initial
        }
    }

    fn save_high_scores(&self) {
        if let Some(lb) = storage::get_mut::<Leaderboard>() {
            lb.entries = self.high_scores.clone();
        }
    }

    fn qualifies_for_leaderboard(&self) -> bool {
        self.high_scores.iter().any(|(_, s)| self.score > *s) || self.high_scores.len() < MAX_HIGH_SCORES
    }

    fn add_to_leaderboard(&mut self, name: String, score: u32) {
        self.new_record = self.high_scores.first().map_or(true, |(_, best)| score > *best);
        self.high_scores.push((name, score));
        self.high_scores.sort_by(|a, b| b.1.cmp(&a.1));
        self.high_scores.truncate(MAX_HIGH_SCORES);
        self.save_high_scores();
    }

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

    fn has_match_at(&self, x: usize, y: usize) -> bool {
        let tile = self.grid[y][x];
        if tile.is_none() { return false; }

        let mut h_count = 1;
        let mut cx = x as i32 - 1;
        while cx >= 0 && self.grid[y][cx as usize] == tile { h_count += 1; cx -= 1; }
        cx = x as i32 + 1;
        while cx < COLS as i32 && self.grid[y][cx as usize] == tile { h_count += 1; cx += 1; }
        if h_count >= 3 { return true; }

        let mut v_count = 1;
        let mut cy = y as i32 - 1;
        while cy >= 0 && self.grid[cy as usize][x] == tile { v_count += 1; cy -= 1; }
        cy = y as i32 + 1;
        while cy < ROWS as i32 && self.grid[cy as usize][x] == tile { v_count += 1; cy += 1; }
        if v_count >= 3 { return true; }

        false
    }

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

    fn clear_matches(&mut self) {
        let matches = self.find_matches();
        // Award points with combo multiplier
        self.score += matches.len() as u32 * 10 * self.combo_count;
        self.time_left = (self.time_left + matches.len() as f32 * 0.5).min(60.0);
        for &(x, y) in &matches {
            self.grid[y][x] = None;
        }
    }

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

    fn swap(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let tmp = self.grid[y1][x1];
        self.grid[y1][x1] = self.grid[y2][x2];
        self.grid[y2][x2] = tmp;
    }
}

/// A simple easing function for smoother movement.
fn cubic_out(t: f32) -> f32 {
    let f = t - 1.0;
    f * f * f + 1.0
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

const ENTRY_CHARS: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', ' ', '.', '!', '?'
];

#[macroquad::main(window_conf)]
async fn main() {
    qrand::srand(macroquad::miniquad::date::now() as _);

    // Initialize high score and settings storage
    storage::store(Leaderboard { entries: vec![("---".to_string(), 0); MAX_HIGH_SCORES] });
    storage::store(Settings { muted: false });

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
    let tex_pause = Texture2D::from_file_with_format(include_bytes!("../assets/23f8.png"), None);
    let tex_play = Texture2D::from_file_with_format(include_bytes!("../assets/25b6.png"), None);

    for t in &textures {
        t.set_filter(FilterMode::Linear);
    }
    tex_mute_on.set_filter(FilterMode::Linear);
    tex_mute_off.set_filter(FilterMode::Linear);
    tex_pause.set_filter(FilterMode::Linear);
    tex_play.set_filter(FilterMode::Linear);

    // Load sounds from embedded bytes
    let snd_swap = load_sound_from_bytes(include_bytes!("../assets/swap.wav")).await.unwrap();
    let snd_match = load_sound_from_bytes(include_bytes!("../assets/match.wav")).await.unwrap();
    let snd_fall = load_sound_from_bytes(include_bytes!("../assets/fall.wav")).await.unwrap();
    let snd_game_over = load_sound_from_bytes(include_bytes!("../assets/game_over.wav")).await.unwrap();

    let mut board = Board::new();

    loop {
        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

        let sw = screen_width();
        let sh = screen_height();
        let board_size = sw.min(sh * 0.8) * 0.95;
        let cell_size = board_size / COLS as f32;
        let offset_x = (sw - board_size) / 2.0;
        let offset_y = (sh - board_size) / 2.0 + (sh * 0.1);

        let mut settings = storage::get_mut::<Settings>();

        // Game Logic State Machine.
        let is_playing = !matches!(board.state, GameState::GameOver | GameState::WaitingToStart | GameState::Paused { .. } | GameState::EnteringName { .. });
        
        if is_playing {
            board.time_left -= get_frame_time();
            if board.time_left <= 0.0 {
                if board.qualifies_for_leaderboard() {
                    board.state = GameState::EnteringName {
                        score: board.score,
                        initials: ['A', 'A', 'A'],
                        active_index: 0,
                    };
                } else {
                    board.state = GameState::GameOver;
                }
                if !settings.muted {
                    info!("SND: Game Over");
                    play_sound(&snd_game_over, PlaySoundParams { looped: false, volume: 1.0 });
                }
            }
        }

        // --- UI Buttons ---
        let pad = 10.0;
        let btn_size = sh * 0.06;
        let (mx, my) = mouse_position();

        // Mute button
        let mute_x = sw - btn_size - pad;
        let mute_y = pad;
        let over_mute = mx >= mute_x - pad && mx <= sw && my >= 0.0 && my <= mute_y + btn_size + pad;

        // Pause button
        let pause_x = mute_x - btn_size - pad;
        let pause_y = pad;
        let over_pause = mx >= pause_x - pad && mx <= mute_x && my >= 0.0 && my <= pause_y + btn_size + pad;

        if is_mouse_button_pressed(MouseButton::Left) {
            if over_mute {
                settings.muted = !settings.muted;
            } else if over_pause {
                match board.state.clone() {
                    GameState::Paused { previous_state } => {
                        board.state = *previous_state;
                    }
                    GameState::GameOver | GameState::WaitingToStart | GameState::EnteringName { .. } => {}
                    other => {
                        board.state = GameState::Paused { previous_state: Box::new(other) };
                    }
                }
            }
        }

        // --- Logic Updates ---
        match board.state.clone() {
            GameState::WaitingToStart => {
                if is_mouse_button_pressed(MouseButton::Left) && !over_mute && !over_pause {
                    if !settings.muted {
                        info!("SND: iOS Audio Context Unlock");
                        play_sound(&snd_swap, PlaySoundParams { looped: false, volume: 0.01 });
                    }
                    board.state = GameState::Idle;
                }
            }
            GameState::Idle => {
                board.combo_count = 0;
                if is_mouse_button_pressed(MouseButton::Left) && !over_mute && !over_pause {
                    if mx >= offset_x && mx < offset_x + board_size && my >= offset_y && my < offset_y + board_size {
                        let cx = ((mx - offset_x) / cell_size) as usize;
                        let cy = ((my - offset_y) / cell_size) as usize;
                        if let Some((sx, sy)) = board.selected {
                            let dx = (cx as i32 - sx as i32).abs();
                            let dy = (cy as i32 - sy as i32).abs();
                            if dx + dy == 1 {
                                board.swap(sx, sy, cx, cy);
                                let matches = board.find_matches();
                                let revert = matches.is_empty();
                                board.swap(cx, cy, sx, sy);
                                board.state = GameState::Swapping { x1: sx, y1: sy, x2: cx, y2: cy, timer: 0.0, revert };
                                if !revert {
                                    board.combo_count = 1;
                                }
                                if !settings.muted {
                                    info!("SND: Swap");
                                    play_sound(&snd_swap, PlaySoundParams { looped: false, volume: 1.0 });
                                }
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
            GameState::Swapping { x1, y1, x2, y2, mut timer, revert } => {
                timer += get_frame_time();
                if timer >= ANIM_DURATION {
                    board.swap(x1, y1, x2, y2);
                    if revert {
                        board.swap(x1, y1, x2, y2);
                        board.state = GameState::Idle;
                    } else {
                        let matches = board.find_matches();
                        let mut match_arr = [(0, 0); COLS * ROWS];
                        for (i, m) in matches.iter().enumerate() { match_arr[i] = *m; }
                        board.state = GameState::Clearing { timer: 0.0, matches: match_arr, match_count: matches.len() };
                        if !settings.muted {
                            info!("SND: Match (Combo {})", board.combo_count);
                            play_sound(&snd_match, PlaySoundParams { looped: false, volume: 1.0 });
                        }
                    }
                } else {
                    board.state = GameState::Swapping { x1, y1, x2, y2, timer, revert };
                }
            }
            GameState::Clearing { mut timer, matches, match_count } => {
                timer += get_frame_time();
                if timer >= ANIM_DURATION {
                    board.clear_matches();
                    board.state = GameState::Falling { timer: 0.0 };
                } else {
                    board.state = GameState::Clearing { timer, matches, match_count };
                }
            }
            GameState::Falling { mut timer } => {
                timer += get_frame_time();
                if timer >= ANIM_DURATION / 2.0 {
                    if board.apply_gravity() {
                        board.state = GameState::Falling { timer: 0.0 };
                        if !settings.muted {
                            info!("SND: Fall");
                            play_sound(&snd_fall, PlaySoundParams { looped: false, volume: 0.3 });
                        }
                    } else {
                        let matches = board.find_matches();
                        if !matches.is_empty() {
                            board.combo_count += 1;
                            let mut match_arr = [(0, 0); COLS * ROWS];
                            for (i, m) in matches.iter().enumerate() { match_arr[i] = *m; }
                            board.state = GameState::Clearing { timer: 0.0, matches: match_arr, match_count: matches.len() };
                            if !settings.muted {
                                info!("SND: Match Cascade (Combo {})", board.combo_count);
                                play_sound(&snd_match, PlaySoundParams { looped: false, volume: 1.0 });
                            }
                        } else {
                            board.state = GameState::Idle;
                        }
                    }
                } else {
                    board.state = GameState::Falling { timer };
                }
            }
            GameState::EnteringName { score, mut initials, mut active_index } => {
                if is_mouse_button_pressed(MouseButton::Left) {
                    let char_w = sw * 0.15;
                    let start_x = sw / 2.0 - char_w * 1.5;
                    let entry_y = sh * 0.5;

                    for i in 0..3 {
                        let rect_x = start_x + i as f32 * char_w;
                        if mx >= rect_x && mx <= rect_x + char_w && my >= entry_y - char_w && my <= entry_y + char_w {
                            if i == active_index {
                                // Cycle character
                                let current_char = initials[i];
                                let pos = ENTRY_CHARS.iter().position(|&c| c == current_char).unwrap_or(0);
                                if my < entry_y {
                                    initials[i] = ENTRY_CHARS[(pos + 1) % ENTRY_CHARS.len()];
                                } else {
                                    initials[i] = ENTRY_CHARS[(pos + ENTRY_CHARS.len() - 1) % ENTRY_CHARS.len()];
                                }
                            } else {
                                active_index = i;
                            }
                        }
                    }

                    // OK Button
                    let ok_w = sw * 0.3;
                    let ok_x = sw / 2.0 - ok_w / 2.0;
                    let ok_y = sh * 0.7;
                    if mx >= ok_x && mx <= ok_x + ok_w && my >= ok_y && my <= ok_y + sh * 0.1 {
                        let name: String = initials.iter().collect();
                        board.add_to_leaderboard(name, score);
                        board.state = GameState::GameOver;
                    }
                }
                board.state = GameState::EnteringName { score, initials, active_index };
            }
            GameState::GameOver => {
                if is_mouse_button_pressed(MouseButton::Left) && !over_mute && !over_pause {
                    board = Board::new();
                    board.state = GameState::Idle;
                }
            }
            GameState::Paused { .. } => {
                if is_mouse_button_pressed(MouseButton::Left) && !over_mute && !over_pause {
                    if let GameState::Paused { previous_state } = board.state {
                        board.state = *previous_state;
                    }
                }
            }
        }

        // --- Rendering ---
        let mut shake_x = 0.0;
        let mut shake_y = 0.0;
        if board.combo_count > 1 && matches!(board.state, GameState::Clearing { .. }) {
            let intensity = (board.combo_count as f32 - 1.0) * 2.0;
            shake_x = qrand::gen_range(-intensity, intensity);
            shake_y = qrand::gen_range(-intensity, intensity);
        }

        draw_rectangle(offset_x + shake_x, offset_y + shake_y, board_size, board_size, Color::new(0.2, 0.2, 0.2, 1.0));

        if let Some((sx, sy)) = board.selected {
            draw_rectangle(offset_x + sx as f32 * cell_size + shake_x, offset_y + sy as f32 * cell_size + shake_y, cell_size, cell_size, Color::new(1.0, 1.0, 1.0, 0.3));
        }

        for y in 0..ROWS {
            for x in 0..COLS {
                let mut draw_x = offset_x + x as f32 * cell_size + shake_x;
                let mut draw_y = offset_y + y as f32 * cell_size + shake_y;
                let mut scale = 1.0;
                let mut alpha = 1.0;

                match board.state {
                    GameState::Swapping { x1, y1, x2, y2, timer, revert } => {
                        let progress = timer / ANIM_DURATION;
                        let t = if revert { (progress * std::f32::consts::PI).sin() } else { cubic_out(progress) };
                        if x == x1 && y == y1 {
                            draw_x += (x2 as f32 - x1 as f32) * cell_size * t;
                            draw_y += (y2 as f32 - y1 as f32) * cell_size * t;
                        } else if x == x2 && y == y2 {
                            draw_x += (x1 as f32 - x2 as f32) * cell_size * t;
                            draw_y += (y1 as f32 - y2 as f32) * cell_size * t;
                        }
                    }
                    GameState::Clearing { timer, matches, match_count } => {
                        for i in 0..match_count {
                            if matches[i].0 == x && matches[i].1 == y {
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
                        DrawTextureParams {
                            dest_size: Some(vec2(actual_cell - pad * 2.0, actual_cell - pad * 2.0)),
                            ..Default::default()
                        },
                    );
                }
            }
        }

        // --- HUD & Buttons ---
        let font_size = sh * 0.05;
        draw_text(&format!("SCORE: {}", board.score), offset_x, offset_y - font_size, font_size, WHITE);
        draw_text(&format!("TIME: {:.0}", board.time_left.max(0.0)), offset_x + board_size - font_size * 5.0, offset_y - font_size, font_size, WHITE);

        if board.combo_count > 1 {
            let combo_text = format!("COMBO X{}", board.combo_count);
            let tw = measure_text(&combo_text, None, (font_size * 1.2) as _, 1.0).width;
            draw_text(&combo_text, sw / 2.0 - tw / 2.0, offset_y + board_size / 2.0, font_size * 1.2, YELLOW);
        }

        draw_texture_ex(
            if settings.muted { &tex_mute_on } else { &tex_mute_off },
            mute_x, mute_y, WHITE,
            DrawTextureParams { dest_size: Some(vec2(btn_size, btn_size)), ..Default::default() },
        );

        if !matches!(board.state, GameState::WaitingToStart | GameState::GameOver | GameState::EnteringName { .. }) {
            draw_texture_ex(
                if matches!(board.state, GameState::Paused { .. }) { &tex_play } else { &tex_pause },
                pause_x, pause_y, WHITE,
                DrawTextureParams { dest_size: Some(vec2(btn_size, btn_size)), ..Default::default() },
            );
        }

        // --- Overlays ---
        if board.state == GameState::WaitingToStart {
            draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.8));
            let start_text = "TAP TO START";
            let tw = measure_text(start_text, None, font_size as _, 1.0).width;
            draw_text(start_text, sw / 2.0 - tw / 2.0, sh / 2.0, font_size, WHITE);
        }

        if let GameState::Paused { .. } = board.state {
            draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.6));
            let p_text = "PAUSED";
            let tw = measure_text(p_text, None, font_size as _, 1.0).width;
            draw_text(p_text, sw / 2.0 - tw / 2.0, sh * 0.4, font_size, WHITE);
            let r_text = "Tap to resume";
            let rtw = measure_text(r_text, None, (font_size*0.6) as _, 1.0).width;
            draw_text(r_text, sw / 2.0 - rtw / 2.0, sh * 0.5, font_size * 0.6, GRAY);
        }

        if let GameState::EnteringName { initials, active_index, .. } = board.state {
            draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.9));
            let title = "NEW HIGH SCORE!";
            let tw = measure_text(title, None, font_size as _, 1.0).width;
            draw_text(title, sw / 2.0 - tw / 2.0, sh * 0.2, font_size, YELLOW);

            let sub = "Enter Initials";
            let stw = measure_text(sub, None, (font_size * 0.6) as _, 1.0).width;
            draw_text(sub, sw / 2.0 - stw / 2.0, sh * 0.3, font_size * 0.6, WHITE);

            let char_w = sw * 0.15;
            let start_x = sw / 2.0 - char_w * 1.5;
            let entry_y = sh * 0.5;

            for i in 0..3 {
                let rect_x = start_x + i as f32 * char_w;
                if i == active_index {
                    draw_rectangle(rect_x + 5.0, entry_y - char_w * 0.8, char_w - 10.0, char_w * 1.6, Color::new(1.0, 1.0, 1.0, 0.2));
                }
                let c_text = initials[i].to_string();
                let cw = measure_text(&c_text, None, char_w as _, 1.0).width;
                draw_text(&c_text, rect_x + char_w / 2.0 - cw / 2.0, entry_y + char_w * 0.3, char_w, if i == active_index { YELLOW } else { WHITE });
                
                // Draw indicators
                if i == active_index {
                    draw_text("▲", rect_x + char_w / 2.0 - 10.0, entry_y - char_w * 0.9, char_w * 0.4, WHITE);
                    draw_text("▼", rect_x + char_w / 2.0 - 10.0, entry_y + char_w * 1.1, char_w * 0.4, WHITE);
                }
            }

            let ok_text = "OK";
            let ok_w = sw * 0.3;
            let ok_x = sw / 2.0 - ok_w / 2.0;
            let ok_y = sh * 0.7;
            draw_rectangle(ok_x, ok_y, ok_w, sh * 0.1, Color::new(0.3, 0.8, 0.3, 1.0));
            let otw = measure_text(ok_text, None, font_size as _, 1.0).width;
            draw_text(ok_text, sw / 2.0 - otw / 2.0, ok_y + sh * 0.07, font_size, WHITE);
        }

        if board.state == GameState::GameOver {
            draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.8));
            draw_text("GAME OVER", sw / 2.0 - measure_text("GAME OVER", None, font_size as _, 1.0).width / 2.0, sh * 0.15, font_size, RED);
            
            draw_text("TOP SCORES:", sw * 0.2, sh * 0.3, font_size * 0.7, WHITE);
            for (i, (name, score)) in board.high_scores.iter().enumerate() {
                let color = if *score == board.score && board.score > 0 { YELLOW } else { GRAY };
                draw_text(&format!("{}. {}  {}", i + 1, name, score), sw * 0.25, sh * 0.38 + (i as f32 * font_size * 0.8), font_size * 0.6, color);
            }
            draw_text("Tap to restart", sw / 2.0 - measure_text("Tap to restart", None, (font_size * 0.6) as _, 1.0).width / 2.0, sh * 0.85, font_size * 0.6, WHITE);
        }

        next_frame().await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_board_initialization() { let board = Board::new(); for y in 0..ROWS { for x in 0..COLS { assert!(board.grid[y][x].is_some()); } } }
    #[test] fn test_initial_board_has_no_matches() { let board = Board::new(); assert!(board.find_matches().is_empty()); }
}
