//! Lumines WASM: A rhythm-puzzle game clone in Rust using Macroquad.

use macroquad::prelude::*;
use quad_rand as qrand;
use serde::{Deserialize, Serialize};

const COLS: usize = 16;
const ROWS: usize = 10;
const VERSION: &str = "26.04.03.1";

#[derive(Clone, Copy, PartialEq, Debug, Serialize, Deserialize)]
enum BlockColor {
    ColorA,
    ColorB,
}

struct ActiveBlock {
    x: i32,
    y: f32,
    colors: [[BlockColor; 2]; 2],
}

impl ActiveBlock {
    fn new() -> Self {
        let mut colors = [[BlockColor::ColorA; 2]; 2];
        for r in 0..2 {
            for c in 0..2 {
                colors[r][c] = if qrand::gen_range(0, 2) == 0 { BlockColor::ColorA } else { BlockColor::ColorB };
            }
        }
        Self {
            x: COLS as i32 / 2 - 1,
            y: -2.0,
            colors,
        }
    }

    fn rotate_cw(&mut self) {
        let tmp = self.colors[0][0];
        self.colors[0][0] = self.colors[1][0];
        self.colors[1][0] = self.colors[1][1];
        self.colors[1][1] = self.colors[0][1];
        self.colors[0][1] = tmp;
    }

    fn rotate_ccw(&mut self) {
        let tmp = self.colors[0][0];
        self.colors[0][0] = self.colors[0][1];
        self.colors[0][1] = self.colors[1][1];
        self.colors[1][1] = self.colors[1][0];
        self.colors[1][0] = tmp;
    }
}

struct Game {
    grid: [[Option<BlockColor>; COLS]; ROWS],
    marked: [[bool; COLS]; ROWS],
    active: ActiveBlock,
    next_block: [[BlockColor; 2]; 2],
    timeline_x: f32,
    timeline_speed: f32,
    score: u32,
    game_over: bool,
    waiting_to_start: bool,
    drop_timer: f32,
    drop_interval: f32,
}

impl Game {
    fn new() -> Self {
        let mut next_block = [[BlockColor::ColorA; 2]; 2];
        for r in 0..2 {
            for c in 0..2 {
                next_block[r][c] = if qrand::gen_range(0, 2) == 0 { BlockColor::ColorA } else { BlockColor::ColorB };
            }
        }

        Self {
            grid: [[None; COLS]; ROWS],
            marked: [[false; COLS]; ROWS],
            active: ActiveBlock::new(),
            next_block,
            timeline_x: 0.0,
            timeline_speed: 2.0, // columns per second
            score: 0,
            game_over: false,
            waiting_to_start: true,
            drop_timer: 0.0,
            drop_interval: 1.0,
        }
    }

    fn update(&mut self, dt: f32) {
        if self.game_over || self.waiting_to_start {
            return;
        }

        // Update Timeline
        let old_x = self.timeline_x;
        self.timeline_x += self.timeline_speed * dt;
        
        // Clear marked blocks when timeline passes
        let start_col = old_x.floor() as usize;
        let end_col = self.timeline_x.floor() as usize;
        
        for col in start_col..=end_col {
            let actual_col = col % COLS;
            for row in 0..ROWS {
                if self.marked[row][actual_col] {
                    self.grid[row][actual_col] = None;
                    self.marked[row][actual_col] = false;
                    self.score += 10;
                }
            }
        }

        if self.timeline_x >= COLS as f32 {
            self.timeline_x -= COLS as f32;
            // Apply gravity after a full sweep
            self.apply_gravity();
        }

        // Mark matches
        self.update_matches();

        // Handle Active Block
        self.drop_timer += dt;
        if self.drop_timer >= self.drop_interval {
            self.drop_timer -= self.drop_interval;
            if !self.move_active(0, 1) {
                self.lock_active();
            }
        }

        // Input Handling
        if is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A) {
            self.move_active(-1, 0);
        }
        if is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D) {
            self.move_active(1, 0);
        }
        if is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S) {
            if !self.move_active(0, 1) {
                self.lock_active();
            }
            self.drop_timer = 0.0;
        }
        if is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::K) {
            self.active.rotate_cw();
            if self.collides(self.active.x, self.active.y.floor() as i32) {
                self.active.rotate_ccw();
            }
        }
        if is_key_pressed(KeyCode::Space) {
            while self.move_active(0, 1) {}
            self.lock_active();
        }
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
        for r in 0..2 {
            for c in 0..2 {
                let gx = self.active.x + c as i32;
                let gy = self.active.y.floor() as i32 + r as i32;
                if gy >= 0 && gy < ROWS as i32 && gx >= 0 && gx < COLS as i32 {
                    self.grid[gy as usize][gx as usize] = Some(self.active.colors[r][c]);
                } else if gy < 0 {
                    self.game_over = true;
                }
            }
        }
        
        if !self.game_over {
            self.apply_gravity();
            self.update_matches();
            
            // New block
            let mut colors = [[BlockColor::ColorA; 2]; 2];
            for r in 0..2 {
                for c in 0..2 {
                    colors[r][c] = self.next_block[r][c];
                    self.next_block[r][c] = if qrand::gen_range(0, 2) == 0 { BlockColor::ColorA } else { BlockColor::ColorB };
                }
            }
            self.active = ActiveBlock {
                x: COLS as i32 / 2 - 1,
                y: -1.0,
                colors,
            };
            if self.collides(self.active.x, self.active.y.floor() as i32) {
                self.game_over = true;
            }
        }
    }

    fn apply_gravity(&mut self) {
        for x in 0..COLS {
            let mut write_y = ROWS - 1;
            for y in (0..ROWS).rev() {
                if let Some(color) = self.grid[y][x] {
                    self.grid[y][x] = None;
                    self.grid[write_y][x] = Some(color);
                    write_y = write_y.saturating_sub(1);
                }
            }
        }
    }

    fn update_matches(&mut self) {
        // A 2x2 square of the same color forms a match.
        // Matches can overlap and expand.
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
                    }
                }
            }
        }
        
        // Retain marked status if it hasn't been swept yet.
        // If a block was marked, it stays marked until the timeline passes.
        // However, we only mark *new* 2x2 squares.
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
        
        let cell_size = (sw / COLS as f32).min(sh / (ROWS as f32 + 4.0));
        let board_w = cell_size * COLS as f32;
        let board_h = cell_size * ROWS as f32;
        let offset_x = (sw - board_w) / 2.0;
        let offset_y = (sh - board_h) / 2.0;

        // Draw Background
        draw_rectangle(offset_x, offset_y, board_w, board_h, Color::new(0.1, 0.1, 0.15, 1.0));

        // Draw Grid Blocks
        for y in 0..ROWS {
            for x in 0..COLS {
                if let Some(color) = self.grid[y][x] {
                    let mut c = match color {
                        BlockColor::ColorA => WHITE,
                        BlockColor::ColorB => ORANGE,
                    };
                    if self.marked[y][x] {
                        // Highlight marked blocks
                        c.r *= 1.2; c.g *= 1.2; c.b *= 1.2;
                        draw_rectangle(offset_x + x as f32 * cell_size, offset_y + y as f32 * cell_size, cell_size, cell_size, c);
                        draw_rectangle_lines(offset_x + x as f32 * cell_size, offset_y + y as f32 * cell_size, cell_size, cell_size, 2.0, YELLOW);
                    } else {
                        draw_rectangle(offset_x + x as f32 * cell_size, offset_y + y as f32 * cell_size, cell_size, cell_size, c);
                        draw_rectangle_lines(offset_x + x as f32 * cell_size, offset_y + y as f32 * cell_size, cell_size, cell_size, 1.0, BLACK);
                    }
                }
            }
        }

        // Draw Active Block
        if !self.game_over && !self.waiting_to_start {
            for r in 0..2 {
                for c in 0..2 {
                    let gx = self.active.x + c as i32;
                    let gy = self.active.y + r as f32;
                    if gy >= -1.0 {
                        let color = match self.active.colors[r][c] {
                            BlockColor::ColorA => WHITE,
                            BlockColor::ColorB => ORANGE,
                        };
                        draw_rectangle(offset_x + gx as f32 * cell_size, offset_y + gy * cell_size, cell_size, cell_size, color);
                        draw_rectangle_lines(offset_x + gx as f32 * cell_size, offset_y + gy * cell_size, cell_size, cell_size, 2.0, SKYBLUE);
                    }
                }
            }
        }

        // Draw Timeline
        let tx = offset_x + self.timeline_x * cell_size;
        draw_line(tx, offset_y, tx, offset_y + board_h, 3.0, WHITE);
        draw_rectangle(tx - 2.0, offset_y, 4.0, board_h, Color::new(1.0, 1.0, 1.0, 0.3));

        // Draw HUD
        draw_text(&format!("SCORE: {}", self.score), 20.0, 30.0, 30.0, WHITE);
        draw_text(&format!("VERSION: {}", VERSION), sw - 150.0, 20.0, 15.0, GRAY);

        // Next Block
        let next_x = sw - 100.0;
        let next_y = 50.0;
        let small_cell = 20.0;
        draw_text("NEXT:", next_x, next_y - 10.0, 20.0, WHITE);
        for r in 0..2 {
            for c in 0..2 {
                let color = match self.next_block[r][c] {
                    BlockColor::ColorA => WHITE,
                    BlockColor::ColorB => ORANGE,
                };
                draw_rectangle(next_x + c as f32 * small_cell, next_y + r as f32 * small_cell, small_cell, small_cell, color);
                draw_rectangle_lines(next_x + c as f32 * small_cell, next_y + r as f32 * small_cell, small_cell, small_cell, 1.0, BLACK);
            }
        }

        if self.waiting_to_start {
            draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.8));
            draw_text("LUMINES WASM", sw / 2.0 - 120.0, sh / 2.0 - 40.0, 40.0, ORANGE);
            draw_text("PRESS SPACE TO START", sw / 2.0 - 130.0, sh / 2.0 + 20.0, 25.0, WHITE);
            draw_text("WASD to Move/Rotate", sw / 2.0 - 100.0, sh / 2.0 + 60.0, 20.0, GRAY);
        }

        if self.game_over {
            draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.8));
            draw_text("GAME OVER", sw / 2.0 - 100.0, sh / 2.0, 40.0, RED);
            draw_text(&format!("FINAL SCORE: {}", self.score), sw / 2.0 - 80.0, sh / 2.0 + 40.0, 25.0, WHITE);
            draw_text("PRESS SPACE TO RESTART", sw / 2.0 - 120.0, sh / 2.0 + 80.0, 20.0, YELLOW);
        }
    }
}

#[macroquad::main("Lumines WASM")]
async fn main() {
    qrand::srand(macroquad::miniquad::date::now() as _);
    let mut game = Game::new();

    loop {
        clear_background(BLACK);

        let dt = get_frame_time();
        game.update(dt);
        game.draw();

        if (game.game_over || game.waiting_to_start) && is_key_pressed(KeyCode::Space) {
            game = Game::new();
            game.waiting_to_start = false;
        }

        next_frame().await
    }
}
