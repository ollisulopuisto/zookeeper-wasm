use macroquad::prelude::*;
use quad_rand as qrand;

const COLS: usize = 8;
const ROWS: usize = 8;
const TILE_TYPES: u8 = 7;
const ANIM_DURATION: f32 = 0.2;

#[derive(Clone, Copy, PartialEq)]
enum GameState {
    Idle,
    Swapping { x1: usize, y1: usize, x2: usize, y2: usize, timer: f32, revert: bool },
    Falling { timer: f32 },
    GameOver,
}

struct Board {
    grid: [[Option<u8>; COLS]; ROWS],
    state: GameState,
    score: u32,
    time_left: f32,
    selected: Option<(usize, usize)>,
}

impl Board {
    fn new() -> Self {
        let mut board = Self {
            grid: [[None; COLS]; ROWS],
            state: GameState::Idle,
            score: 0,
            time_left: 60.0,
            selected: None,
        };
        board.fill_initial();
        board
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

        // Horizontal
        let mut h_count = 1;
        let mut cx = x;
        while cx > 0 && self.grid[y][cx - 1] == tile { h_count += 1; cx -= 1; }
        cx = x;
        while cx < COLS - 1 && self.grid[y][cx + 1] == tile { h_count += 1; cx += 1; }
        if h_count >= 3 { return true; }

        // Vertical
        let mut v_count = 1;
        let mut cy = y;
        while cy > 0 && self.grid[cy - 1][x] == tile { v_count += 1; cy -= 1; }
        cy = y;
        while cy < ROWS - 1 && self.grid[cy + 1][x] == tile { v_count += 1; cy += 1; }
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

    fn clear_matches(&mut self) -> bool {
        let matches = self.find_matches();
        if matches.is_empty() { return false; }
        self.score += matches.len() as u32 * 10;
        self.time_left = (self.time_left + matches.len() as f32 * 0.5).min(60.0);
        for &(x, y) in &matches {
            self.grid[y][x] = None;
        }
        true
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

fn window_conf() -> Conf {
    Conf {
        window_title: "Zookeeper WASM".to_owned(),
        window_width: 600,
        window_height: 800,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    qrand::srand(macroquad::miniquad::date::now() as _);
    
    let textures = [
        Texture2D::from_file_with_format(include_bytes!("../assets/1f435.png"), None),
        Texture2D::from_file_with_format(include_bytes!("../assets/1f981.png"), None),
        Texture2D::from_file_with_format(include_bytes!("../assets/1f42f.png"), None),
        Texture2D::from_file_with_format(include_bytes!("../assets/1f418.png"), None),
        Texture2D::from_file_with_format(include_bytes!("../assets/1f992.png"), None),
        Texture2D::from_file_with_format(include_bytes!("../assets/1f43c.png"), None),
        Texture2D::from_file_with_format(include_bytes!("../assets/1f438.png"), None),
    ];
    
    // Set nearest neighbor for pixel art / crisp look if needed, but we can leave default
    for t in &textures {
        t.set_filter(FilterMode::Linear);
    }

    let mut board = Board::new();

    loop {
        clear_background(Color::new(0.1, 0.1, 0.1, 1.0));

        let sw = screen_width();
        let sh = screen_height();
        
        let board_size = sw.min(sh * 0.8) * 0.95;
        let cell_size = board_size / COLS as f32;
        let offset_x = (sw - board_size) / 2.0;
        let offset_y = (sh - board_size) / 2.0 + (sh * 0.1);

        if board.state != GameState::GameOver {
            board.time_left -= get_frame_time();
            if board.time_left <= 0.0 {
                board.state = GameState::GameOver;
            }
        }

        match board.state {
            GameState::Idle => {
                if is_mouse_button_pressed(MouseButton::Left) {
                    let (mx, my) = mouse_position();
                    if mx >= offset_x && mx < offset_x + board_size && my >= offset_y && my < offset_y + board_size {
                        let cx = ((mx - offset_x) / cell_size) as usize;
                        let cy = ((my - offset_y) / cell_size) as usize;
                        
                        if let Some((sx, sy)) = board.selected {
                            let dx = (cx as i32 - sx as i32).abs();
                            let dy = (cy as i32 - sy as i32).abs();
                            if dx + dy == 1 {
                                board.swap(sx, sy, cx, cy);
                                let revert = board.find_matches().is_empty();
                                board.swap(cx, cy, sx, sy); // Swap back for animation
                                board.state = GameState::Swapping { x1: sx, y1: sy, x2: cx, y2: cy, timer: 0.0, revert };
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
                        board.swap(x1, y1, x2, y2); // Swap back again
                        board.state = GameState::Idle;
                    } else {
                        board.clear_matches();
                        board.state = GameState::Falling { timer: 0.0 };
                    }
                } else {
                    board.state = GameState::Swapping { x1, y1, x2, y2, timer, revert };
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

        // Draw board background
        draw_rectangle(offset_x, offset_y, board_size, board_size, Color::new(0.2, 0.2, 0.2, 1.0));

        // Draw selection highlight
        if let Some((sx, sy)) = board.selected {
            draw_rectangle(
                offset_x + sx as f32 * cell_size,
                offset_y + sy as f32 * cell_size,
                cell_size,
                cell_size,
                Color::new(1.0, 1.0, 1.0, 0.3),
            );
        }

        // Draw tiles
        for y in 0..ROWS {
            for x in 0..COLS {
                let mut draw_x = offset_x + x as f32 * cell_size;
                let mut draw_y = offset_y + y as f32 * cell_size;
                
                let tile = board.grid[y][x];

                // Handle Swapping Animation
                if let GameState::Swapping { x1, y1, x2, y2, timer, revert } = board.state {
                    let progress = timer / ANIM_DURATION;
                    // If revert is true, we go to swap target then back
                    let t = if revert { (progress * std::f32::consts::PI).sin() } else { progress };
                    
                    if x == x1 && y == y1 {
                        draw_x += (x2 as f32 - x1 as f32) * cell_size * t;
                        draw_y += (y2 as f32 - y1 as f32) * cell_size * t;
                    } else if x == x2 && y == y2 {
                        draw_x += (x1 as f32 - x2 as f32) * cell_size * t;
                        draw_y += (y1 as f32 - y2 as f32) * cell_size * t;
                    }
                }
                
                // Handle Falling Animation (simple visual slide)
                if let GameState::Falling { timer } = board.state {
                    let progress = timer / (ANIM_DURATION / 2.0);
                    // For tiles that just fell, animate them sliding down visually if they are not at the top?
                    // Actual gravity logic in apply_gravity changes the logical board.
                    // To do it correctly visually, we just let them appear as they drop, maybe slight y offset.
                    // The simplest is to render with a small slide, but since the logic shifts immediately,
                    // we can just offset all tiles that have empty space below them?
                    // For a basic clone, popping into place is often okay, but let's slide down slightly.
                    // This is complex to do perfectly without tracking individual tile drop history, so we skip complex fall anims for a pure 60FPS fast feel.
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

        // UI
        let font_size = sh * 0.05;
        draw_text(&format!("SCORE: {}", board.score), offset_x, offset_y - font_size, font_size, WHITE);
        draw_text(&format!("TIME: {:.0}", board.time_left), offset_x + board_size - font_size * 5.0, offset_y - font_size, font_size, WHITE);

        if board.state == GameState::GameOver {
            let go_text = "GAME OVER";
            let tw = measure_text(go_text, None, font_size as _, 1.0).width;
            draw_text(go_text, sw / 2.0 - tw / 2.0, sh / 2.0, font_size, RED);
            let tap_text = "Tap to restart";
            let ttw = measure_text(tap_text, None, (font_size*0.6) as _, 1.0).width;
            draw_text(tap_text, sw / 2.0 - ttw / 2.0, sh / 2.0 + font_size * 1.5, font_size * 0.6, WHITE);
        }

        next_frame().await
    }
}