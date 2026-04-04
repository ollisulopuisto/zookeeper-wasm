//! Lumines WASM: A rhythm-puzzle game clone in Rust using Macroquad.

mod audio;

use macroquad::prelude::*;
use quad_rand as qrand;
use serde::{Deserialize, Serialize};
use audio::AudioManager;

const COLS: usize = 16;
const ROWS: usize = 10;
const VERSION: &str = "26.04.04.194";
const BPM: f32 = 130.0;
const BEATS_PER_SWEEP: f32 = 8.0;
const FREEZE_DURATION: f32 = 4.0;
const MAX_FREEZE_METER: f32 = 100.0;
const SCORE_PER_SQUARE: u32 = 50;  // points awarded per 2×2 square cleared
const COMBO_MIN_SQUARES: u32 = 4;  // min squares per sweep to maintain combo
const CHAIN_PROBABILITY: u32 = 12; // % chance a falling piece contains one chain cell
const CHAIN_SYMBOL_COLOR: Color = Color::new(0.0, 1.0, 0.0, 0.90);
const ENTRY_DELAY: f32 = 1.0;  // seconds block is held above playfield for repositioning
const HUD_CONTROL_PAD_RATIO: f32 = 0.01;
const HUD_CONTROL_PAD_MIN: f32 = 8.0;
const HUD_CONTROL_PAD_MAX: f32 = 14.0;
const NEXT_PREVIEW_CELL_HUD_RATIO: f32 = 0.35;
const NEXT_PREVIEW_MAX_SCREEN_WIDTH_RATIO: f32 = 0.35;
const NEXT_PREVIEW_MIN_HALF_WIDTH: f32 = 24.0;
const COMBO_VERTICAL_OFFSET_FACTOR: f32 = 0.95;
const COMBO_FONT_SCALE: f32 = 1.1;
const FREEZE_METER_VERTICAL_SPACING_FACTOR: f32 = 0.65;
const NEXT_PREVIEW_HORIZONTAL_SPACING_FACTOR: f32 = 1.25;
const NEXT_PREVIEW_VERTICAL_SPACING_FACTOR: f32 = 0.5;
const HUD_SECTION_GAP_FACTOR: f32 = 1.0;
// Block rendering constants
const BLOCK_BASE_DARKEN: f32 = 0.82;
const BLOCK_HIGHLIGHT_HEIGHT_RATIO: f32 = 0.36;
const BLOCK_HIGHLIGHT_ALPHA: f32 = 0.92;
const BLOCK_SHADOW_CUTOFF_RATIO: f32 = 0.38;
const BLOCK_SHADOW_ALPHA: f32 = 0.46;
const BLOCK_GLINT_SIZE_RATIO: f32 = 0.22;
const BLOCK_GLINT_MIN_PX: f32 = 2.0;
const BLOCK_GLINT_OFFSET_RATIO: f32 = 0.10;
const BLOCK_GLINT_ASPECT: f32 = 0.45;
const BLOCK_GLINT_ALPHA: f32 = 0.82;
const BLOCK_OUTLINE_ALPHA: f32 = 0.90;
const BLOCK_OUTLINE_INSET: f32 = 1.0;
const BLOCK_OUTLINE_MIN_WIDTH: f32 = 2.0;
// Completion animation constants
const CLEAR_FLASH_DURATION: f32 = 0.35;    // seconds for a column-clear flash to fade out
const CLEAR_FLASH_MAX_ALPHA: f32 = 0.55;   // max alpha for the column flash
const MATCH_FLASH_DECAY_RATE: f32 = 3.0;   // per-second decay of the match-detection glow (≈0.33 s)
const MATCH_GLOW_MAX_ALPHA: f32 = 0.5;     // max alpha for the board-edge match glow
const MATCH_GLOW_LINE_WIDTH: f32 = 6.0;    // thickness of the board-edge glow ring
const INTERNAL_COORDINATE_SCALE: f32 = 40.0; // logical pixels per cell used in particle physics
const PARTICLE_SPAWN_COUNT: usize = 12;    // number of particles per cleared cell
const PARTICLE_VX_RANGE: f32 = 150.0;      // max horizontal velocity (+/-)
const PARTICLE_VY_MIN: f32 = -200.0;       // initial vertical velocity min (upward)
const PARTICLE_VY_MAX: f32 = 50.0;         // initial vertical velocity max
const PARTICLE_LIFE_MIN: f32 = 0.5;        // min lifetime in seconds
const PARTICLE_LIFE_MAX: f32 = 1.0;        // max lifetime
const PARTICLE_GRAVITY: f32 = 200.0;       // downward acceleration
const PARTICLE_MIN_SIZE: f32 = 1.5;        // minimum particle radius in baseline (40 px/cell) space
const PARTICLE_MAX_SIZE: f32 = 5.0;        // maximum particle radius in baseline space
const PARTICLE_DECAY_RATE: f32 = 1.5;      // multiplier on dt for particle lifetime decay
const MARKED_PULSE_FREQ: f32 = 12.0 * std::f32::consts::TAU; // radians/sec for a 12 Hz marked-block brightness pulse
const MARKED_PULSE_AMPLITUDE: f32 = 0.35;  // 0..1 amplitude of the brightness pulse (35%)
const MARKED_GLOW_THRESHOLD: f32 = 0.88;   // pulse value above which the outer glow ring appears
// Falling animation constants
const FALL_GRAVITY: f32 = 28.0;   // grid-units per second² for falling blocks
const IMPACT_DURATION: f32 = 0.30; // seconds the squish effect lasts after landing
// Shared layout constants
const HUD_MARGIN_RATIO: f32 = 0.03;        // horizontal gutter as fraction of sw
const BTN_SIZE_RATIO: f32 = 0.06;          // pause/mute button size as fraction of sh
// Landscape layout constants
const LANDSCAPE_HUD_RATIO: f32 = 0.20;     // fraction of sh reserved for the top HUD bar
const LANDSCAPE_FONT_LG_RATIO: f32 = 0.25; // large font as fraction of top-bar height
const LANDSCAPE_FONT_SM_RATIO: f32 = 0.13; // small font as fraction of top-bar height
const LANDSCAPE_FREEZE_METER_W_RATIO: f32 = 0.30; // FREEZE meter width as fraction of sw
const LANDSCAPE_FREEZE_METER_H_RATIO: f32 = 0.11; // FREEZE meter height as fraction of hud_h
// Portrait (mobile) layout constants
const PORTRAIT_TOP_HUD_RATIO: f32 = 0.10;  // fraction of sh reserved for the top bar
const PORTRAIT_BOT_HUD_RATIO: f32 = 0.22;  // fraction of sh reserved for the bottom bar
const PORTRAIT_FONT_LG_RATIO: f32 = 0.45;  // large font as fraction of top-bar height
const PORTRAIT_FONT_SM_RATIO: f32 = 0.25;  // small font as fraction of top-bar height
const PORTRAIT_NEXT_CELL_HUD_RATIO: f32 = 0.32;  // NEXT cell size relative to bot_h
const PORTRAIT_NEXT_CELL_SCREEN_RATIO: f32 = 0.18; // NEXT cell size relative to sw
const PORTRAIT_NEXT_X_CENTER: f32 = 0.15;  // horizontal centre of NEXT preview (fraction of sw)
const PORTRAIT_METER_X_RATIO: f32 = 0.35;  // left edge of FREEZE meter (fraction of sw)
const PORTRAIT_METER_W_RATIO: f32 = 0.65;  // right portion of sw used by FREEZE meter
const PORTRAIT_METER_H_RATIO: f32 = 0.14;  // FREEZE bar height relative to bot_h

const MAX_HIGH_SCORES: usize = 10;
const MAX_NAME_LENGTH: usize = 10;
#[cfg(target_arch = "wasm32")]
const MOBILE_POPUP_MAX_WIDTH: f32 = 600.0;

#[derive(Serialize, Deserialize, Clone)]
struct LeaderboardEntry {
    name: String,
    score: u32,
}

#[cfg(target_arch = "wasm32")]
extern "C" {
    fn js_load_leaderboard(ptr: *mut u8, max_len: u32) -> u32;
    fn js_save_leaderboard(ptr: *const u8, len: u32);
    fn js_ask_name(ptr: *mut u8, max_len: u32) -> u32;
}

fn load_high_scores() -> Vec<LeaderboardEntry> {
    #[cfg(target_arch = "wasm32")]
    {
        use shared::leaderboard::load_list;
        load_list::<LeaderboardEntry, _>(4096, |ptr, max_len| unsafe {
            js_load_leaderboard(ptr, max_len)
        })
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        Vec::new()
    }
}

fn save_high_scores(scores: &[LeaderboardEntry]) {
    #[cfg(target_arch = "wasm32")]
    {
        use shared::leaderboard::save_list;
        save_list(scores, |ptr, len| unsafe {
            js_save_leaderboard(ptr, len);
        });
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = scores;
}

/// Layout coordinates for the NEXT preview and FREEZE meter, computed per orientation.
struct HudLayout {
    next_cell:      f32,
    next_x:         f32,
    next_blocks_top: f32,
    next_label_y:   f32,
    meter_x:        f32,
    meter_w:        f32,
    meter_h:        f32,
    meter_y:        f32,
}

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
enum BlockColor {
    ColorA,
    ColorB,
}

impl BlockColor {
    fn random() -> Self {
        if qrand::gen_range(0, 2) == 0 { BlockColor::ColorA } else { BlockColor::ColorB }
    }

    fn random_2x2() -> [[BlockColor; 2]; 2] {
        [
            [Self::random(), Self::random()],
            [Self::random(), Self::random()],
        ]
    }
}

/// Generate a random 2×2 block together with chain flags.
/// With probability `CHAIN_PROBABILITY`% one random cell is a chain cell.
fn random_block_with_chain() -> ([[BlockColor; 2]; 2], [[bool; 2]; 2]) {
    let colors = BlockColor::random_2x2();
    let mut chains = [[false; 2]; 2];
    if qrand::gen_range(0u32, 100) < CHAIN_PROBABILITY {
        let row = qrand::gen_range(0, 2);
        let col = qrand::gen_range(0, 2);
        chains[row][col] = true;
    }
    (colors, chains)
}

struct ActiveBlock {
    x: i32,
    y: f32,
    colors: [[BlockColor; 2]; 2],
    is_chain: [[bool; 2]; 2],
}

impl ActiveBlock {
    fn new(colors: [[BlockColor; 2]; 2], is_chain: [[bool; 2]; 2]) -> Self {
        Self {
            x: COLS as i32 / 2 - 1,
            y: -2.0,
            colors,
            is_chain,
        }
    }

    fn rotate_cw(&mut self) {
        let tmp = self.colors[0][0];
        self.colors[0][0] = self.colors[1][0];
        self.colors[1][0] = self.colors[1][1];
        self.colors[1][1] = self.colors[0][1];
        self.colors[0][1] = tmp;

        let tmp = self.is_chain[0][0];
        self.is_chain[0][0] = self.is_chain[1][0];
        self.is_chain[1][0] = self.is_chain[1][1];
        self.is_chain[1][1] = self.is_chain[0][1];
        self.is_chain[0][1] = tmp;
    }

    fn rotate_ccw(&mut self) {
        let tmp = self.colors[0][0];
        self.colors[0][0] = self.colors[0][1];
        self.colors[0][1] = self.colors[1][1];
        self.colors[1][1] = self.colors[1][0];
        self.colors[1][0] = tmp;

        let tmp = self.is_chain[0][0];
        self.is_chain[0][0] = self.is_chain[0][1];
        self.is_chain[0][1] = self.is_chain[1][1];
        self.is_chain[1][1] = self.is_chain[1][0];
        self.is_chain[1][0] = tmp;
    }
}

struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    life: f32,
    color: Color,
    size: f32,
}

fn draw_stylized_block(x: f32, y: f32, size: f32, color: Color, border_width: f32, border_color: Color, scale_x: f32, scale_y: f32) {
    let w = size * scale_x;
    let h = size * scale_y;
    // Centre horizontally, anchor to bottom so squash looks grounded.
    let bx = x + (size - w) * 0.5;
    let by = y + (size - h);

    // Slightly-darkened base fill so the highlight band reads against it for ALL colors,
    // including pure white (light-gray base vs. white highlight = visible contrast).
    let base = Color::new(color.r * BLOCK_BASE_DARKEN, color.g * BLOCK_BASE_DARKEN, color.b * BLOCK_BASE_DARKEN, 1.0);
    draw_rectangle(bx, by, w, h, base);

    // Top highlight band at the original (brighter) color — always visibly lighter than base
    draw_rectangle(bx, by, w, h * BLOCK_HIGHLIGHT_HEIGHT_RATIO, Color::new(color.r, color.g, color.b, BLOCK_HIGHLIGHT_ALPHA));

    // Bottom-right shadow: dark triangle for a strong comic-book depth cue
    draw_triangle(
        vec2(bx + w * BLOCK_SHADOW_CUTOFF_RATIO, by + h),
        vec2(bx + w, by + h * BLOCK_SHADOW_CUTOFF_RATIO),
        vec2(bx + w, by + h),
        Color::new(0.0, 0.0, 0.0, BLOCK_SHADOW_ALPHA),
    );

    // Small specular glint in top-left corner — the comic-book "shine" pill
    let g = (size * BLOCK_GLINT_SIZE_RATIO).max(BLOCK_GLINT_MIN_PX);
    draw_rectangle(bx + w * BLOCK_GLINT_OFFSET_RATIO, by + h * BLOCK_GLINT_OFFSET_RATIO, g, g * BLOCK_GLINT_ASPECT, Color::new(1.0, 1.0, 1.0, BLOCK_GLINT_ALPHA));

    // Bold black outer outline + tinted inner border
    draw_rectangle_lines(bx, by, w, h, (border_width + BLOCK_OUTLINE_INSET).max(BLOCK_OUTLINE_MIN_WIDTH), Color::new(0.0, 0.0, 0.0, BLOCK_OUTLINE_ALPHA));
    draw_rectangle_lines(bx + BLOCK_OUTLINE_INSET, by + BLOCK_OUTLINE_INSET, (w - BLOCK_OUTLINE_INSET * 2.0).max(0.0), (h - BLOCK_OUTLINE_INSET * 2.0).max(0.0), border_width, border_color);
}

/// Draw a green "+" cross symbol centred on the cell to mark it as a chain cell.
fn draw_chain_symbol(x: f32, y: f32, size: f32) {
    let cx = x + size * 0.5;
    let cy = y + size * 0.5;
    let arm = size * 0.28;
    let thickness = (size * 0.12).max(2.0);
    // Horizontal bar
    draw_rectangle(cx - arm, cy - thickness * 0.5, arm * 2.0, thickness, CHAIN_SYMBOL_COLOR);
    // Vertical bar
    draw_rectangle(cx - thickness * 0.5, cy - arm, thickness, arm * 2.0, CHAIN_SYMBOL_COLOR);
}

struct Game {
    grid: [[Option<BlockColor>; COLS]; ROWS],
    marked: [[bool; COLS]; ROWS],
    // Per-cell falling animation state (grid-unit offsets, velocities, impact timers)
    v_offsets: [[f32; COLS]; ROWS],
    v_velocities: [[f32; COLS]; ROWS],
    impact_timers: [[f32; COLS]; ROWS],
    active: ActiveBlock,
    next_block: [[BlockColor; 2]; 2],
    next_chain: [[bool; 2]; 2],
    timeline_x: f32,
    timeline_speed: f32,
    score: u32,
    combo: u32,
    squares_cleared_this_sweep: u32,
    game_over: bool,
    waiting_to_start: bool,
    drop_timer: f32,
    drop_interval: f32,
    audio: AudioManager,
    particles: Vec<Particle>,
    clear_flashes: Vec<(usize, f32)>,  // (column, life 0..1) – brief flash when timeline clears a column
    match_flash: f32,                   // brief board-edge glow when a new 2×2 match is detected
    is_paused: bool,
    
    // Time Freeze
    freeze_meter: f32,
    is_frozen: bool,
    freeze_timer: f32,

    // Entry phase: block is shown above playfield before it starts dropping
    entry_timer: f32,

    // Touch/Mouse state
    swipe_start: Option<Vec2>,
    tap_timer: f32,
    swipe_occurred: bool,
    tex_mute_on: Texture2D,
    tex_mute_off: Texture2D,
    tex_pause: Texture2D,
    tex_play: Texture2D,

    // Hiscores
    high_scores: Vec<LeaderboardEntry>,
    leaderboard_saved: bool,
    new_score_rank: Option<usize>,
    entering_name: bool,
    current_name: String,
    just_finished_name_entry: bool,
}

impl Game {
    async fn new() -> Self {
        let audio = AudioManager::new().await;
        let tex_mute_on = Texture2D::from_file_with_format(include_bytes!("../../zookeeper/assets/1f507.png"), None);
        let tex_mute_off = Texture2D::from_file_with_format(include_bytes!("../../zookeeper/assets/1f50a.png"), None);
        let tex_pause = Texture2D::from_file_with_format(include_bytes!("../../zookeeper/assets/23f8.png"), None);
        let tex_play = Texture2D::from_file_with_format(include_bytes!("../../zookeeper/assets/25b6.png"), None);
        tex_mute_on.set_filter(FilterMode::Linear);
        tex_mute_off.set_filter(FilterMode::Linear);
        tex_pause.set_filter(FilterMode::Linear);
        tex_play.set_filter(FilterMode::Linear);
        
        let seconds_per_sweep = (60.0 / BPM) * BEATS_PER_SWEEP;
        let timeline_speed = COLS as f32 / seconds_per_sweep;

        let (init_colors, init_chain) = random_block_with_chain();
        let (next_colors, next_chain) = random_block_with_chain();

        Self {
            grid: [[None; COLS]; ROWS],
            marked: [[false; COLS]; ROWS],
            v_offsets: [[0.0; COLS]; ROWS],
            v_velocities: [[0.0; COLS]; ROWS],
            impact_timers: [[0.0; COLS]; ROWS],
            active: ActiveBlock::new(init_colors, init_chain),
            next_block: next_colors,
            next_chain,
            timeline_x: 0.0,
            timeline_speed,
            score: 0,
            combo: 0,
            squares_cleared_this_sweep: 0,
            game_over: false,
            waiting_to_start: true,
            drop_timer: 0.0,
            drop_interval: 1.0,
            audio,
            particles: Vec::new(),
            clear_flashes: Vec::new(),
            match_flash: 0.0,
            is_paused: false,
            freeze_meter: 0.0,
            is_frozen: false,
            freeze_timer: 0.0,
            entry_timer: 0.0,  // set to ENTRY_DELAY when game starts
            swipe_start: None,
            tap_timer: 0.0,
            swipe_occurred: false,
            tex_mute_on,
            tex_mute_off,
            tex_pause,
            tex_play,
            high_scores: Vec::new(),
            leaderboard_saved: false,
            new_score_rank: None,
            entering_name: false,
            current_name: String::new(),
            just_finished_name_entry: false,
        }
    }

    fn qualifies_for_leaderboard(&self) -> bool {
        self.high_scores.iter().any(|e| self.score > e.score) || self.high_scores.len() < MAX_HIGH_SCORES
    }

    fn add_to_leaderboard(&mut self, name: &str) {
        self.high_scores.push(LeaderboardEntry {
            name: name.to_string(),
            score: self.score,
        });
        self.high_scores.sort_by(|a, b| b.score.cmp(&a.score));
        self.high_scores.truncate(MAX_HIGH_SCORES);
        self.new_score_rank = self
            .high_scores
            .iter()
            .rposition(|e| e.name == name && e.score == self.score);
        save_high_scores(&self.high_scores);
    }

    fn spawn_particles(&mut self, x: f32, y: f32, color: Color) {
        for _ in 0..PARTICLE_SPAWN_COUNT {
            self.particles.push(Particle {
                x,
                y,
                vx: qrand::gen_range(-PARTICLE_VX_RANGE, PARTICLE_VX_RANGE),
                vy: qrand::gen_range(PARTICLE_VY_MIN, PARTICLE_VY_MAX),
                life: qrand::gen_range(PARTICLE_LIFE_MIN, PARTICLE_LIFE_MAX),
                color,
                size: qrand::gen_range(PARTICLE_MIN_SIZE, PARTICLE_MAX_SIZE),
            });
        }
    }

    fn update(&mut self, dt: f32) {
        self.just_finished_name_entry = false;
        let dt = dt.min(0.1);
        let sw = screen_width();
        let sh = screen_height();
        let pad = (sw * HUD_CONTROL_PAD_RATIO).clamp(HUD_CONTROL_PAD_MIN, HUD_CONTROL_PAD_MAX);
        let btn_size = sh * BTN_SIZE_RATIO;
        let (mx, my) = mouse_position();
        let mute_x = sw - btn_size - pad;
        let mute_y = pad;
        let over_mute = mx >= mute_x - pad && mx <= sw && my >= 0.0 && my <= mute_y + btn_size + pad;
        let pause_x = mute_x - btn_size - pad;
        let pause_y = pad;
        let over_pause = mx >= pause_x - pad && mx <= mute_x && my >= 0.0 && my <= pause_y + btn_size + pad;

        if is_mouse_button_pressed(MouseButton::Left) {
            if over_mute {
                let new_muted = !self.audio.is_muted();
                self.audio.set_muted(new_muted);
                if !new_muted && !self.is_paused && !self.game_over && !self.waiting_to_start && !self.is_frozen {
                    self.audio.play_music();
                }
            }
            if over_pause {
                self.is_paused = !self.is_paused;
                if self.is_paused {
                    self.audio.stop_music();
                } else if !self.game_over && !self.waiting_to_start && !self.is_frozen {
                    self.audio.play_music();
                }
            }
        }

        if is_key_pressed(KeyCode::P) {
            self.is_paused = !self.is_paused;
            if self.is_paused {
                self.audio.stop_music();
            } else if !self.game_over && !self.waiting_to_start && !self.is_frozen {
                self.audio.play_music();
            }
        }

        if self.entering_name {
            while let Some(c) = get_char_pressed() {
                if (c.is_alphanumeric() || c == ' ') && self.current_name.len() < MAX_NAME_LENGTH {
                    self.current_name.push(c);
                }
            }
            if is_key_pressed(KeyCode::Backspace) {
                self.current_name.pop();
            }

            #[cfg(target_arch = "wasm32")]
            {
                let is_mobile = sw < MOBILE_POPUP_MAX_WIDTH && sw < sh;
                if is_mobile {
                    let prompt_w = sw * 0.4;
                    let prompt_x = sw / 2.0 - prompt_w / 2.0;
                    let prompt_y = sh * 0.62;
                    let prompt_h = sh * 0.06;
                    if is_mouse_button_pressed(MouseButton::Left)
                        && mx >= prompt_x
                        && mx <= prompt_x + prompt_w
                        && my >= prompt_y
                        && my <= prompt_y + prompt_h
                    {
                        let mut buf = [0u8; 16];
                        let len = unsafe { js_ask_name(buf.as_mut_ptr(), buf.len() as u32) } as usize;
                        if let Ok(js_name) = std::str::from_utf8(&buf[..len.min(buf.len())]) {
                            self.current_name = js_name.trim().to_string();
                        }
                    }
                }
            }

            let ok_w = sw * 0.3;
            let ok_x = sw / 2.0 - ok_w / 2.0;
            let ok_y = sh * 0.74;
            let ok_h = sh * 0.1;
            let submit = is_key_pressed(KeyCode::Enter)
                || is_key_pressed(KeyCode::KpEnter)
                || (is_mouse_button_pressed(MouseButton::Left)
                    && mx >= ok_x
                    && mx <= ok_x + ok_w
                    && my >= ok_y
                    && my <= ok_y + ok_h);
            if submit {
                let name = self.current_name.clone();
                self.add_to_leaderboard(&name);
                self.entering_name = false;
                self.just_finished_name_entry = true;
            }
        }

        if self.is_paused || self.game_over || self.waiting_to_start {
            return;
        }

        // --- Time Freeze Logic ---
        if self.is_frozen {
            self.freeze_timer -= dt;
            if self.freeze_timer <= 0.0 {
                self.is_frozen = false;
                self.audio.play_music();
            }
        }

        if !self.is_frozen && (is_key_pressed(KeyCode::LeftShift) || is_key_pressed(KeyCode::R)) && self.freeze_meter >= MAX_FREEZE_METER {
            self.is_frozen = true;
            self.freeze_timer = FREEZE_DURATION;
            self.freeze_meter = 0.0;
            self.audio.stop_music(); 
        }

        // Update Timeline
        if !self.is_frozen {
            let old_x = self.timeline_x;
            self.timeline_x += self.timeline_speed * dt;
            
            let mut cleared_this_step = 0;
            let mut squares_this_step = 0u32;
            let start_col = old_x.floor() as usize;
            let end_col = self.timeline_x.floor() as usize;
            let mut cleared_per_col = [false; COLS];

            for col in start_col..=end_col {
                let actual_col = col % COLS;

                // Count 2×2 squares whose top-left corner is at actual_col (before clearing).
                // The right column (actual_col + 1) is still intact at this point, allowing
                // detection of all squares that begin here.
                if actual_col + 1 < COLS {
                    for row in 0..ROWS - 1 {
                        if self.marked[row][actual_col]
                            && self.marked[row][actual_col + 1]
                            && self.marked[row + 1][actual_col]
                            && self.marked[row + 1][actual_col + 1]
                        {
                            squares_this_step += 1;
                        }
                    }
                }

                for row in 0..ROWS {
                    if self.marked[row][actual_col] {
                        if let Some(color) = self.grid[row][actual_col] {
                            let p_color = match color {
                                BlockColor::ColorA => WHITE,
                                BlockColor::ColorB => ORANGE,
                            };
                            self.spawn_particles(actual_col as f32 * INTERNAL_COORDINATE_SCALE, row as f32 * INTERNAL_COORDINATE_SCALE, p_color);
                        }
                        self.grid[row][actual_col] = None;
                        self.marked[row][actual_col] = false;
                        cleared_this_step += 1;
                        cleared_per_col[actual_col] = true;
                    }
                }
            }

            if squares_this_step > 0 {
                self.score += squares_this_step * SCORE_PER_SQUARE * (1 + self.combo);
                self.squares_cleared_this_sweep += squares_this_step;
                self.audio.play_clear(1.0 + (self.combo as f32 * 0.1).min(1.0));
            }
            // Freeze meter rewards every block cleared (including the right halves of squares
            // counted in a prior column step).
            if cleared_this_step > 0 {
                self.freeze_meter = (self.freeze_meter + cleared_this_step as f32 * 0.5).min(MAX_FREEZE_METER);
                // Push a flash for every column that had blocks cleared
                for (col, &had_clear) in cleared_per_col.iter().enumerate() {
                    if had_clear {
                        self.clear_flashes.push((col, 1.0));
                    }
                }
            }

            while self.timeline_x >= COLS as f32 {
                self.timeline_x -= COLS as f32;
                if self.squares_cleared_this_sweep >= COMBO_MIN_SQUARES {
                    self.combo += 1;
                } else {
                    self.combo = 0;
                }
                self.squares_cleared_this_sweep = 0;
                self.apply_gravity();
            }
        }

        // Mark matches
        self.update_matches();

        // Handle Active Block
        // During the entry phase the block sits above the playfield; count down
        // the timer and skip automatic dropping until the phase expires.
        if self.entry_timer > 0.0 {
            self.entry_timer = (self.entry_timer - dt).max(0.0);
            if self.entry_timer == 0.0 {
                // Entry phase just expired – trigger an immediate drop so the
                // block doesn't linger invisibly above the playfield.
                self.drop_timer = self.drop_interval;
            } else {
                self.drop_timer = 0.0;
            }
        } else {
            self.drop_timer += dt;
            if self.drop_timer >= self.drop_interval {
                self.drop_timer = 0.0;
                if !self.move_active(0, 1) {
                    self.lock_active();
                }
            }
        }

        // --- Input Handling ---
        
        // Keyboard
        if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
            self.move_active(-1, 0);
        }
        if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
            self.move_active(1, 0);
        }
        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
            self.entry_timer = 0.0;  // cancel entry phase immediately
            if !self.move_active(0, 1) {
                self.lock_active();
            }
            self.drop_timer = 0.0;
        }
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::K) {
            self.active.rotate_cw();
            self.audio.play_rotate();
            if self.collides(self.active.x, self.active.y.floor() as i32) {
                self.active.rotate_ccw();
            }
        }
        if is_key_pressed(KeyCode::Space) {
            self.entry_timer = 0.0;  // cancel entry phase immediately
            while self.move_active(0, 1) {}
            self.lock_active();
        }

        // Touch & Mouse
        let mouse_pos = mouse_position().into();
        if is_mouse_button_pressed(MouseButton::Left) && !over_mute && !over_pause {
            self.swipe_start = Some(mouse_pos);
            self.tap_timer = 0.0;
            self.swipe_occurred = false;
        }
        
        if let Some(start) = self.swipe_start {
            self.tap_timer += dt;
            let diff = mouse_pos - start;
            if is_mouse_button_released(MouseButton::Left) {
                if diff.length() < 10.0 && self.tap_timer < 0.2 && !self.swipe_occurred {
                    // Tap to rotate
                    self.active.rotate_cw();
                    self.audio.play_rotate();
                    if self.collides(self.active.x, self.active.y.floor() as i32) {
                        self.active.rotate_ccw();
                    }
                } else if diff.y > 50.0 {
                    // Swipe down to drop – also cancels entry phase
                    self.entry_timer = 0.0;
                    while self.move_active(0, 1) {}
                    self.lock_active();
                }
                self.swipe_start = None;
            } else if diff.length() > 30.0 {
                // Dragging to move
                let cell_w = screen_width() / COLS as f32;
                let dx = (diff.x / cell_w) as i32;
                if dx != 0 {
                    if self.move_active(dx, 0) {
                        self.swipe_occurred = true;
                        self.swipe_start = Some(mouse_pos);
                    }
                }
            }
        }

        // Update Particles
        for p in self.particles.iter_mut() {
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            p.vy += PARTICLE_GRAVITY * dt;  // downward gravity
            p.life -= dt * PARTICLE_DECAY_RATE;
        }
        self.particles.retain(|p| p.life > 0.0);

        // Decay column clear flashes and match flash
        for flash in self.clear_flashes.iter_mut() {
            flash.1 -= dt / CLEAR_FLASH_DURATION;
        }
        self.clear_flashes.retain(|f| f.1 > 0.0);
        self.match_flash = (self.match_flash - dt * MATCH_FLASH_DECAY_RATE).max(0.0);

        // Update falling-block animation physics (gravity + landing squish timers).
        for y in 0..ROWS {
            for x in 0..COLS {
                if self.grid[y][x].is_none() {
                    self.v_offsets[y][x] = 0.0;
                    self.v_velocities[y][x] = 0.0;
                    self.impact_timers[y][x] = 0.0;
                    continue;
                }

                if self.v_offsets[y][x] < 0.0 {
                    self.v_velocities[y][x] += dt * FALL_GRAVITY;
                    self.v_offsets[y][x] += dt * self.v_velocities[y][x];
                    if self.v_offsets[y][x] >= 0.0 {
                        self.v_offsets[y][x] = 0.0;
                        self.v_velocities[y][x] = 0.0;
                        self.impact_timers[y][x] = IMPACT_DURATION;
                    }
                }
                if self.impact_timers[y][x] > 0.0 {
                    self.impact_timers[y][x] -= dt;
                }
            }
        }

        // Decay column clear flashes and match flash
        for flash in self.clear_flashes.iter_mut() {
            flash.1 -= dt / CLEAR_FLASH_DURATION;
        }
        self.clear_flashes.retain(|f| f.1 > 0.0);
        self.match_flash = (self.match_flash - dt * MATCH_FLASH_DECAY_RATE).max(0.0);
    }

    fn collides(&self, x: i32, y: i32) -> bool {
        for r in 0..2 {
            for c in 0..2 {
                let gx = x + c as i32;
                let gy = y + r as i32;
                if gx < 0 || gx >= COLS as i32 || gy >= ROWS as i32 {
                    return true;
                }
                if gy >= 0 && self.grid[gy as usize][gx as usize].is_some() {
                    return true;
                }
            }
        }
        false
    }

    fn move_active(&mut self, dx: i32, dy: i32) -> bool {
        let nx = self.active.x + dx;
        let ny = self.active.y.floor() as i32 + dy;
        if !self.collides(nx, ny) {
            self.active.x = nx;
            self.active.y = ny as f32;
            return true;
        }
        false
    }

    fn lock_active(&mut self) {
        self.audio.play_land();
        let mut chain_cells: Vec<(usize, usize)> = Vec::new();
        for r in 0..2 {
            for c in 0..2 {
                let gx = self.active.x + c as i32;
                let gy = self.active.y.floor() as i32 + r as i32;
                if gy >= 0 && gy < ROWS as i32 && gx >= 0 && gx < COLS as i32 {
                    self.grid[gy as usize][gx as usize] = Some(self.active.colors[r][c]);
                    if self.active.is_chain[r][c] {
                        chain_cells.push((gx as usize, gy as usize));
                    }
                } else if gy < 0 {
                    self.game_over = true;
                }
            }
        }
        
        if !self.game_over {
            // Flood-fill chain cells before gravity so positions are still accurate.
            let mut any_chained = false;
            for (cx, cy) in chain_cells {
                if self.flood_mark_chain(cx, cy) > 0 {
                    any_chained = true;
                }
            }
            if any_chained {
                self.audio.play_match();
            }

            self.apply_gravity();
            self.update_matches();
            
            let colors = self.next_block;
            let is_chain = self.next_chain;
            let (new_next_colors, new_next_chain) = random_block_with_chain();
            self.next_block = new_next_colors;
            self.next_chain = new_next_chain;
            
            self.active = ActiveBlock::new(colors, is_chain);
            if self.collides(self.active.x, self.active.y.floor() as i32) {
                self.game_over = true;
            } else {
                // Start the entry phase so the player can position the block
                self.entry_timer = ENTRY_DELAY;
                self.drop_timer = 0.0;
            }
        }
        if self.game_over {
            self.audio.stop_music();
        }
    }

    /// BFS flood-fill: marks all grid cells connected to (x, y) that share the same color.
    /// Returns the number of newly marked cells.
    fn flood_mark_chain(&mut self, x: usize, y: usize) -> usize {
        let target_color = match self.grid[y][x] {
            Some(c) => c,
            None => return 0,
        };
        let mut count = 0usize;
        let mut stack: Vec<(usize, usize)> = vec![(x, y)];
        let mut visited = [[false; COLS]; ROWS];
        while let Some((cx, cy)) = stack.pop() {
            if visited[cy][cx] { continue; }
            visited[cy][cx] = true;
            if self.grid[cy][cx] == Some(target_color) {
                if !self.marked[cy][cx] {
                    self.marked[cy][cx] = true;
                    count += 1;
                }
                if cx > 0 { stack.push((cx - 1, cy)); }
                if cx + 1 < COLS { stack.push((cx + 1, cy)); }
                if cy > 0 { stack.push((cx, cy - 1)); }
                if cy + 1 < ROWS { stack.push((cx, cy + 1)); }
            }
        }
        count
    }

    fn apply_gravity(&mut self) {
        for x in 0..COLS {
            let mut write_y = ROWS - 1;
            let mut num_blocks = 0usize;
            for y in (0..ROWS).rev() {
                if let Some(color) = self.grid[y][x] {
                    let drop = write_y as i32 - y as i32;
                    self.grid[y][x] = None;
                    self.grid[write_y][x] = Some(color);
                    if drop > 0 {
                        // Block falls `drop` rows: carry existing offset plus additional drop.
                        self.v_offsets[write_y][x] = self.v_offsets[y][x] - drop as f32;
                        self.v_velocities[write_y][x] = self.v_velocities[y][x];
                        self.impact_timers[write_y][x] = self.impact_timers[y][x];
                        
                        self.v_offsets[y][x] = 0.0;
                        self.v_velocities[y][x] = 0.0;
                        self.impact_timers[y][x] = 0.0;
                    }
                    num_blocks += 1;
                    write_y = write_y.saturating_sub(1);
                }
            }
            // Clear offsets for rows that are empty after compaction (rows above the topmost block).
            let filled_from = ROWS - num_blocks;
            for y in 0..filled_from {
                self.v_offsets[y][x] = 0.0;
                self.v_velocities[y][x] = 0.0;
                self.impact_timers[y][x] = 0.0;
            }
        }
    }

    fn update_matches(&mut self) {
        let mut new_match = false;
        let mut new_marked = [[false; COLS]; ROWS];
        for y in 0..ROWS - 1 {
            for x in 0..COLS - 1 {
                if let Some(c1) = self.grid[y][x] {
                    if self.grid[y][x+1] == Some(c1) &&
                       self.grid[y+1][x] == Some(c1) &&
                       self.grid[y+1][x+1] == Some(c1) {
                        new_marked[y][x] = true;
                        new_marked[y][x+1] = true;
                        new_marked[y+1][x] = true;
                        new_marked[y+1][x+1] = true;
                        if !self.marked[y][x] { new_match = true; }
                    }
                }
            }
        }
        
        if new_match {
            self.audio.play_match();
            self.match_flash = 1.0;
        }

        for y in 0..ROWS {
            for x in 0..COLS {
                if new_marked[y][x] {
                    self.marked[y][x] = true;
                }
            }
        }
    }

    fn draw(&self) {
        let sw = screen_width();
        let sh = screen_height();

        // On portrait screens (mobile) use a thin top bar and a dedicated bottom
        // info bar so that NEXT preview and FREEZE meter get plenty of space.
        // On landscape screens keep the original single top-HUD layout.
        let is_portrait = sw < sh;
        let hud_h = if is_portrait { sh * PORTRAIT_TOP_HUD_RATIO } else { sh * LANDSCAPE_HUD_RATIO };
        let bot_h = if is_portrait { sh * PORTRAIT_BOT_HUD_RATIO } else { 0.0 };
        // Font sizes scale with the top-bar height. Use different ratios so text
        // fits the thinner portrait bar while preserving landscape proportions.
        let font_lg = if is_portrait { hud_h * PORTRAIT_FONT_LG_RATIO } else { hud_h * LANDSCAPE_FONT_LG_RATIO };
        let font_sm = if is_portrait { hud_h * PORTRAIT_FONT_SM_RATIO } else { hud_h * LANDSCAPE_FONT_SM_RATIO };
        let margin  = sw * HUD_MARGIN_RATIO;    // horizontal gutter

        // Board fills the space between the two bars, centred horizontally.
        let cell_size = (sw / COLS as f32).min((sh - hud_h - bot_h) / ROWS as f32);
        let board_w = cell_size * COLS as f32;
        let board_h = cell_size * ROWS as f32;
        let offset_x = (sw - board_w) / 2.0;
        let offset_y = hud_h;

        // Draw Background
        let bg_color = if self.is_frozen { Color::new(0.05, 0.05, 0.05, 1.0) } else { Color::new(0.05, 0.05, 0.1, 1.0) };
        draw_rectangle(offset_x, offset_y, board_w, board_h, bg_color);

        // Draw Grid Blocks
        for y in 0..ROWS {
            for x in 0..COLS {
                if let Some(color) = self.grid[y][x] {
                    let mut c = match color {
                        BlockColor::ColorA => WHITE,
                        BlockColor::ColorB => ORANGE,
                    };
                    if self.is_frozen {
                        let avg = (c.r + c.g + c.b) / 3.0;
                        c = Color::new(avg * 0.8, avg * 0.8, avg * 0.8, 1.0);
                    }

                    // Apply falling offset so blocks animate smoothly to their target row.
                    let bx = offset_x + x as f32 * cell_size;
                    let by = offset_y + (y as f32 + self.v_offsets[y][x]) * cell_size;

                    // Landing squish: briefly squash vertically and stretch horizontally.
                    let mut scale_x = 1.0f32;
                    let mut scale_y = 1.0f32;
                    if self.impact_timers[y][x] > 0.0 {
                        let t = (self.impact_timers[y][x] / IMPACT_DURATION).clamp(0.0, 1.0);
                        let s = (t * std::f32::consts::PI).sin();
                        scale_y -= s * 0.15;
                        scale_x += s * 0.10;
                    }

                    if self.marked[y][x] {
                        let t = get_time() as f32;
                        // Faster, more dramatic pulse (frequency and amplitude raised vs original)
                        let pulse = (t * MARKED_PULSE_FREQ).sin() * MARKED_PULSE_AMPLITUDE + (1.0 - MARKED_PULSE_AMPLITUDE);
                        let h_color = if self.is_frozen {
                            Color::new(0.5, 0.5, 1.0, 1.0)
                        } else {
                            // Border oscillates between yellow and a pale warm yellow tint
                            let shift = (t * 5.0).sin() * 0.5 + 0.5;
                            Color::new(1.0, 0.8 + shift * 0.2, shift * 0.3, 1.0)
                        };
                        let highlight = Color::new(c.r * pulse, c.g * pulse, c.b * pulse, 1.0);
                        draw_stylized_block(bx, by, cell_size, highlight, 3.0, h_color, scale_x, scale_y);
                        // Extra glow outline that flares at pulse peaks
                        if !self.is_frozen && pulse > MARKED_GLOW_THRESHOLD {
                            let glow_a = ((pulse - MARKED_GLOW_THRESHOLD) / (1.0 - MARKED_GLOW_THRESHOLD)) * 0.55;
                            draw_rectangle_lines(bx - 2.0, by - 2.0, cell_size + 4.0, cell_size + 4.0, 2.0,
                                Color::new(1.0, 1.0, 0.8, glow_a));
                        }
                    } else {
                        draw_stylized_block(bx, by, cell_size, c, 1.0, Color::new(0.0, 0.0, 0.0, 0.5), scale_x, scale_y);
                    }
                }
            }
        }

        // Draw column clear flash effects (yellow-white burst when timeline clears a column)
        if !self.is_frozen {
            for &(col, life) in &self.clear_flashes {
                let fx = offset_x + col as f32 * cell_size;
                let alpha = life * life * CLEAR_FLASH_MAX_ALPHA;  // quadratic falloff for a sharp flash
                draw_rectangle(fx, offset_y, cell_size, board_h, Color::new(1.0, 0.95, 0.5, alpha));
            }
        }

        // Draw match flash – a brief board-edge glow when a new 2×2 match is found
        if self.match_flash > 0.0 {
            let alpha = self.match_flash * self.match_flash * MATCH_GLOW_MAX_ALPHA;
            let lw = MATCH_GLOW_LINE_WIDTH;
            draw_rectangle_lines(offset_x - lw * 0.5, offset_y - lw * 0.5,
                board_w + lw, board_h + lw, lw, Color::new(1.0, 1.0, 0.3, alpha));
        }

        // Draw Active Block
        if !self.game_over && !self.waiting_to_start {
            let in_entry = self.entry_timer > 0.0;

            // During the entry phase draw a staging area above the playfield so the
            // player can clearly see and position the incoming block.
            if in_entry {
                let bx = offset_x + self.active.x as f32 * cell_size;
                let staging_top = offset_y - cell_size * 2.0;
                let staging_w   = cell_size * 2.0;
                let staging_h   = cell_size * 2.0;
                // Dark backdrop
                draw_rectangle(bx - 2.0, staging_top - 2.0, staging_w + 4.0, staging_h + 4.0,
                    Color::new(0.05, 0.05, 0.15, 0.88));
                // Thin border
                draw_rectangle_lines(bx - 2.0, staging_top - 2.0, staging_w + 4.0, staging_h + 4.0,
                    1.5, Color::new(0.6, 0.85, 1.0, 0.5));
                // Countdown bar: full = entry is fresh, empty = about to drop
                let bar_frac = self.entry_timer / ENTRY_DELAY;
                let bar_h    = (cell_size * 0.12).max(3.0);
                let bar_y    = staging_top - bar_h - 3.0;
                draw_rectangle(bx, bar_y, staging_w, bar_h, Color::new(0.2, 0.2, 0.3, 0.8));
                draw_rectangle(bx, bar_y, staging_w * bar_frac, bar_h, SKYBLUE);
            }

            for r in 0..2 {
                for c in 0..2 {
                    let gx = self.active.x + c as i32;
                    let gy = self.active.y + r as f32;
                    // Render cells inside the playfield always; render cells above the
                    // playfield whenever they are above y=0 (e.g. during entry or just
                    // after entry expires but before the first drop moves the block down).
                    if gy >= 0.0 || self.active.y < 0.0 {
                        let mut color = match self.active.colors[r][c] {
                            BlockColor::ColorA => WHITE,
                            BlockColor::ColorB => ORANGE,
                        };
                        if self.is_frozen {
                            let avg = (color.r + color.g + color.b) / 3.0;
                            color = Color::new(avg * 0.8, avg * 0.8, avg * 0.8, 1.0);
                        }
                        let bx = offset_x + gx as f32 * cell_size;
                        let by = offset_y + gy * cell_size;
                        let glow_alpha = if in_entry {
                            (get_time() as f32 * 12.0).sin() * 0.15 + 0.35
                        } else {
                            (get_time() as f32 * 8.0).sin() * 0.08 + 0.22
                        };
                        let border_color = if self.active.is_chain[r][c] { 
                            LIME 
                        } else if in_entry { 
                            YELLOW 
                        } else { 
                            SKYBLUE 
                        };
                        draw_stylized_block(bx, by, cell_size, color, 2.0, border_color, 1.0, 1.0);
                        draw_rectangle_lines(bx - 1.0, by - 1.0, cell_size + 2.0, cell_size + 2.0, 1.0, Color::new(0.6, 0.85, 1.0, glow_alpha));
                        if self.active.is_chain[r][c] {
                            draw_chain_symbol(bx, by, cell_size);
                        }
                    }
                }
            }
        }

        // Draw Particles
        let p_scale = (cell_size / INTERNAL_COORDINATE_SCALE).max(0.5);
        for p in &self.particles {
            let mut c = p.color;
            c.a = p.life * p.life;  // quadratic fade – crisper disappearance
            draw_circle(
                offset_x + p.x * p_scale,
                offset_y + p.y * p_scale,
                (p.size * p_scale).max(1.5),
                c,
            );
        }

        // Draw Timeline
        if !self.is_frozen {
            let tx = offset_x + self.timeline_x * cell_size;
            draw_line(tx, offset_y, tx, offset_y + board_h, 4.0, WHITE);
            let gradient_w = 20.0;
            draw_rectangle(tx - gradient_w, offset_y, gradient_w, board_h, Color::new(1.0, 1.0, 1.0, 0.15));
        } else {
            let tx = offset_x + self.timeline_x * cell_size;
            draw_line(tx, offset_y, tx, offset_y + board_h, 4.0, SKYBLUE);
            let tf_dims = measure_text("TIME FROZEN", None, font_lg as u16, 1.0);
            draw_text("TIME FROZEN", sw / 2.0 - tf_dims.width / 2.0, offset_y - font_lg * 0.7, font_lg, SKYBLUE);
        }

        // Draw HUD (AFTER blocks so it overlays any that bleed above offset_y during grace period)
        let pad = (sw * HUD_CONTROL_PAD_RATIO).clamp(HUD_CONTROL_PAD_MIN, HUD_CONTROL_PAD_MAX);
        let btn_size = sh * BTN_SIZE_RATIO;
        let mute_x = sw - btn_size - pad;
        let mute_y = pad;
        let pause_x = mute_x - btn_size - pad;
        let pause_y = pad;

        let hud_top_text_y = pad + font_lg;
        draw_text(&format!("SCORE: {}", self.score), margin, hud_top_text_y, font_lg, WHITE);
        if self.combo > 0 {
            draw_text(
                &format!("COMBO x{}", self.combo),
                margin,
                hud_top_text_y + font_lg * COMBO_VERTICAL_OFFSET_FACTOR,
                font_lg * COMBO_FONT_SCALE,
                YELLOW,
            );
        }

        // Version label – skip in portrait mode where the top bar is narrow.
        if !is_portrait {
            let version_label = format!("v{}", VERSION);
            let version_dims = measure_text(&version_label, None, font_sm as u16, 1.0);
            let version_x = (pause_x - pad - version_dims.width).max(margin);
            draw_text(&version_label, version_x, mute_y + font_sm, font_sm, GRAY);
        }

        // Pause & Mute buttons
        draw_texture_ex(
            if self.audio.is_muted() { &self.tex_mute_on } else { &self.tex_mute_off },
            mute_x,
            mute_y,
            WHITE,
            DrawTextureParams { dest_size: Some(vec2(btn_size, btn_size)), ..Default::default() },
        );
        draw_texture_ex(
            if self.is_paused { &self.tex_play } else { &self.tex_pause },
            pause_x,
            pause_y,
            WHITE,
            DrawTextureParams { dest_size: Some(vec2(btn_size, btn_size)), ..Default::default() },
        );

        // Compute layout-specific coordinates for NEXT preview and FREEZE meter,
        // then draw both using a single shared block below.
        let layout = if is_portrait {
            // --- Portrait (mobile) bottom info bar ---
            // Anchor the bar to the reserved bottom area so it stays fixed at the screen bottom.
            let bar_top  = sh - bot_h;
            let bar_mid_y = bar_top + bot_h * 0.5;

            // NEXT preview – left portion of the bar, vertically centred.
            let next_cell = (bot_h * PORTRAIT_NEXT_CELL_HUD_RATIO).min(sw * PORTRAIT_NEXT_CELL_SCREEN_RATIO);
            let next_w    = next_cell * 2.0;
            let next_min_x = margin;
            let next_max_x = (sw - margin - next_w).max(next_min_x);
            let next_x    = (sw * PORTRAIT_NEXT_X_CENTER - next_w * 0.5).clamp(next_min_x, next_max_x);
            let next_blocks_top = bar_mid_y - next_cell; // centre 2 rows vertically
            let next_label_y = next_blocks_top - font_sm * 0.5;

            // FREEZE meter – right side of the bar.
            let meter_x = sw * PORTRAIT_METER_X_RATIO + margin;
            let meter_w = sw * PORTRAIT_METER_W_RATIO - margin * 2.0;
            let meter_h = bot_h * PORTRAIT_METER_H_RATIO;
            let meter_y = bar_mid_y - meter_h * 0.5;

            HudLayout { next_cell, next_x, next_blocks_top, next_label_y, meter_x, meter_w, meter_h, meter_y }
        } else {
            // --- Landscape: original single-HUD layout ---
            // NEXT preview – place to the left of controls to avoid overlap.
            let preview_cell_from_hud = hud_h * NEXT_PREVIEW_CELL_HUD_RATIO;
            let max_preview_width_from_screen = sw * NEXT_PREVIEW_MAX_SCREEN_WIDTH_RATIO - 2.0 * margin;
            let clamped_preview_width = max_preview_width_from_screen.max(NEXT_PREVIEW_MIN_HALF_WIDTH);
            let preview_cell_from_screen = clamped_preview_width / 2.0;
            let next_cell = preview_cell_from_hud.min(preview_cell_from_screen);
            let next_w = next_cell * 2.0;
            let next_x = (pause_x - pad * NEXT_PREVIEW_HORIZONTAL_SPACING_FACTOR - next_w).max(margin);
            let next_blocks_top = mute_y + btn_size + pad * NEXT_PREVIEW_VERTICAL_SPACING_FACTOR;
            let next_label_y = next_blocks_top - font_sm * 0.5;

            // FREEZE meter: cap width so it never overlaps the NEXT preview on narrow screens.
            let meter_x = margin;
            let meter_desired_w = sw * LANDSCAPE_FREEZE_METER_W_RATIO;
            let meter_h = hud_h * LANDSCAPE_FREEZE_METER_H_RATIO;
            let meter_y = mute_y + btn_size + pad * FREEZE_METER_VERTICAL_SPACING_FACTOR;
            let meter_gap = pad * HUD_SECTION_GAP_FACTOR;
            let meter_max_w_before_next = (next_x - margin - meter_gap).max(0.0);
            let meter_w = meter_desired_w.min(meter_max_w_before_next);

            HudLayout { next_cell, next_x, next_blocks_top, next_label_y, meter_x, meter_w, meter_h, meter_y }
        };

        // Draw NEXT preview (shared for both orientations).
        draw_text("NEXT", layout.next_x, layout.next_label_y, font_sm * 1.2, WHITE);
        for r in 0..2 {
            for c in 0..2 {
                let color = match self.next_block[r][c] {
                    BlockColor::ColorA => WHITE,
                    BlockColor::ColorB => ORANGE,
                };
                let bx = layout.next_x + c as f32 * layout.next_cell;
                let by = layout.next_blocks_top + r as f32 * layout.next_cell;
                let border = if self.next_chain[r][c] { LIME } else { BLACK };
                draw_stylized_block(bx, by, layout.next_cell, color, 1.0, border, 1.0, 1.0);
                if self.next_chain[r][c] {
                    draw_chain_symbol(bx, by, layout.next_cell);
                }
            }
        }

        // Draw FREEZE meter (shared for both orientations).
        draw_rectangle(layout.meter_x, layout.meter_y, layout.meter_w, layout.meter_h, DARKGRAY);
        draw_rectangle(layout.meter_x, layout.meter_y, layout.meter_w * (self.freeze_meter / MAX_FREEZE_METER), layout.meter_h,
            if self.freeze_meter >= MAX_FREEZE_METER { SKYBLUE } else { BLUE });
        draw_text("FREEZE", layout.meter_x, layout.meter_y + layout.meter_h + font_sm, font_sm, GRAY);

        if self.waiting_to_start {
            draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.85));
            draw_text("LUMINES WASM", sw / 2.0 - 140.0, sh / 2.0 - 60.0, 50.0, ORANGE);
            draw_text("TAP or SPACE to Start", sw / 2.0 - 130.0, sh / 2.0, 30.0, WHITE);
            draw_text("SHIFT: Time Freeze (when full)", sw / 2.0 - 120.0, sh / 2.0 + 40.0, 20.0, SKYBLUE);
            draw_text("Swipe/Arrows: Move", sw / 2.0 - 100.0, sh / 2.0 + 70.0, 20.0, GRAY);
        }

        if self.game_over {
            draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.88));

            let title_sz = (sh * 0.07).clamp(30.0, 60.0);
            let title = "GAME OVER";
            let tm = measure_text(title, None, title_sz as u16, 1.0);
            draw_text(title, sw / 2.0 - tm.width / 2.0, sh * 0.10, title_sz, RED);

            let score_str = format!("SCORE: {}", self.score);
            let score_sz = (sh * 0.045).clamp(18.0, 36.0);
            let sm = measure_text(&score_str, None, score_sz as u16, 1.0);
            draw_text(&score_str, sw / 2.0 - sm.width / 2.0, sh * 0.18, score_sz, WHITE);

            if !self.high_scores.is_empty() {
                let lbl = "HIGH SCORES";
                let lbl_sz = (sh * 0.038).clamp(14.0, 28.0);
                let lm = measure_text(lbl, None, lbl_sz as u16, 1.0);
                draw_text(lbl, sw / 2.0 - lm.width / 2.0, sh * 0.27, lbl_sz, YELLOW);

                let entry_sz = (sh * 0.038).clamp(13.0, 22.0);
                let row_h = (sh * 0.062).clamp(16.0, 30.0);

                let center_x = sw / 2.0;
                let rank_x = center_x - (sh * 0.22).clamp(80.0, 160.0);
                let name_x = center_x - (sh * 0.15).clamp(50.0, 110.0);
                let score_x = center_x + (sh * 0.22).clamp(80.0, 160.0);

                for (i, entry) in self.high_scores.iter().take(MAX_HIGH_SCORES).enumerate() {
                    let y = sh * 0.34 + i as f32 * row_h;
                    let is_new = self.new_score_rank == Some(i);
                    let color = if is_new { ORANGE } else { Color::new(0.85, 0.85, 0.85, 1.0) };

                    // Rank column (left aligned)
                    draw_text(&format!("{}.", i + 1), rank_x, y, entry_sz, color);

                    // Name column (left aligned)
                    draw_text(&entry.name, name_x, y, entry_sz, color);

                    // Score column (right aligned)
                    let score_str = format!("{}", entry.score);
                    let sem = measure_text(&score_str, None, entry_sz as u16, 1.0);
                    draw_text(&score_str, score_x - sem.width, y, entry_sz, color);
                }
            }

            let restart_str = "TAP or SPACE to Restart";
            let rst_sz = (sh * 0.035).clamp(14.0, 24.0);
            let rm = measure_text(restart_str, None, rst_sz as u16, 1.0);
            draw_text(restart_str, sw / 2.0 - rm.width / 2.0, sh * 0.96, rst_sz, YELLOW);

            if self.entering_name {
                draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.9));

                let title = "NEW HIGH SCORE!";
                let title_sz = (sh * 0.06).clamp(24.0, 44.0);
                let tm = measure_text(title, None, title_sz as u16, 1.0);
                draw_text(title, sw / 2.0 - tm.width / 2.0, sh * 0.18, title_sz, YELLOW);

                let prompt = "Type your name";
                let prompt_sz = (sh * 0.04).clamp(16.0, 30.0);
                let pm = measure_text(prompt, None, prompt_sz as u16, 1.0);
                draw_text(prompt, sw / 2.0 - pm.width / 2.0, sh * 0.36, prompt_sz, GRAY);

                let display_name = if self.current_name.is_empty() {
                    "_".to_string()
                } else {
                    format!("{}_", self.current_name)
                };
                let name_sz = (sh * 0.06).clamp(22.0, 42.0);
                let nm = measure_text(&display_name, None, name_sz as u16, 1.0);
                draw_text(&display_name, sw / 2.0 - nm.width / 2.0, sh * 0.5, name_sz, WHITE);

                #[cfg(target_arch = "wasm32")]
                {
                    let is_mobile = sw < MOBILE_POPUP_MAX_WIDTH && sw < sh;
                    if is_mobile {
                        let prompt_w = sw * 0.4;
                        let prompt_x = sw / 2.0 - prompt_w / 2.0;
                        let prompt_y = sh * 0.62;
                        let prompt_h = sh * 0.06;
                        draw_rectangle(prompt_x, prompt_y, prompt_w, prompt_h, Color::new(0.2, 0.2, 0.2, 1.0));
                        let popup = "TAP FOR POPUP";
                        let pop_sz = (sh * 0.024).clamp(12.0, 18.0);
                        let popm = measure_text(popup, None, pop_sz as u16, 1.0);
                        draw_text(popup, sw / 2.0 - popm.width / 2.0, prompt_y + prompt_h * 0.72, pop_sz, WHITE);
                    }
                }

                let ok_w = sw * 0.3;
                let ok_x = sw / 2.0 - ok_w / 2.0;
                let ok_y = sh * 0.74;
                let ok_h = sh * 0.1;
                draw_rectangle(ok_x, ok_y, ok_w, ok_h, Color::new(0.3, 0.8, 0.3, 1.0));
                let ok = "OK";
                let ok_sz = (sh * 0.05).clamp(18.0, 32.0);
                let okm = measure_text(ok, None, ok_sz as u16, 1.0);
                draw_text(ok, sw / 2.0 - okm.width / 2.0, ok_y + ok_h * 0.68, ok_sz, WHITE);
            }
        }

        if self.is_paused {
            draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.92));
            draw_text("PAUSED", sw / 2.0 - 80.0, sh / 2.0, 50.0, WHITE);
            draw_text("Press P or Tap to Resume", sw / 2.0 - 110.0, sh / 2.0 + 40.0, 20.0, GRAY);
        }
    }
}

#[macroquad::main("Lumines WASM")]
async fn main() {
    qrand::srand(macroquad::miniquad::date::now() as _);
    let mut game = Game::new().await;

    loop {
        clear_background(BLACK);

        let dt = get_frame_time();
        game.update(dt);

        // When game just ended, load scores, optionally prompt for name, and save.
        // Must run before draw() so the leaderboard table is visible on the first game-over frame.
        if game.game_over && !game.leaderboard_saved {
            game.leaderboard_saved = true;
            game.high_scores = load_high_scores();
            // Sort after loading since load_list does not guarantee order.
            game.high_scores.sort_by(|a, b| b.score.cmp(&a.score));
            if game.score > 0 && game.qualifies_for_leaderboard() {
                game.entering_name = true;
            }
        }

        game.draw();

        if (game.game_over || game.waiting_to_start)
            && !game.entering_name
            && !game.just_finished_name_entry
            && (is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left))
        {
            #[cfg(target_arch = "wasm32")]
            {
                // This is a hacky way to focus the canvas in Macroquad WASM if needed,
                // but usually clicking it is enough if it has a tabindex.
            }
            
            let audio = game.audio;
            let muted = audio.is_muted();
            game = Game::new().await;
            game.audio = audio;
            game.audio.set_muted(muted);
            game.waiting_to_start = false;
            game.entry_timer = ENTRY_DELAY;  // give player time to position the first block
            game.audio.play_music();
        }

        next_frame().await
    }
}
