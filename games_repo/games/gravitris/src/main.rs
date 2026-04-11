mod game;

use macroquad::prelude::*;
use crate::game::{Board, COLS, ROWS, GravityWell};

const VERSION: &str = "26.04.11.236";

#[macroquad::main("Gravitris")]
async fn main() {
    let mut board = Board::new();
    board.wells.push(GravityWell { x: 2, y: 15, strength: 40.0 });
    board.wells.push(GravityWell { x: 7, y: 15, strength: 40.0 });
    board.spawn_piece();

    let mut last_update = get_time();
    let mut last_gravity = get_time();
    let drop_speed = 0.5;
    let gravity_well_speed = 0.2;
    let mut game_over = false;

    loop {
        if !game_over {
            let now = get_time();

            // Handle Input
            if is_key_pressed(KeyCode::Left) {
                board.move_piece(-1, 0);
            }
            if is_key_pressed(KeyCode::Right) {
                board.move_piece(1, 0);
            }
            if is_key_pressed(KeyCode::Down) {
                board.move_piece(0, 1);
            }
            if is_key_pressed(KeyCode::Up) {
                board.rotate_piece();
            }
            if is_key_pressed(KeyCode::Space) {
                while board.move_piece(0, 1) {}
                board.lock_piece();
                board.clear_lines();
                if !board.spawn_piece() {
                    game_over = true;
                }
            }

            // Normal falling
            if now - last_update > drop_speed {
                if !board.move_piece(0, 1) {
                    board.lock_piece();
                    board.clear_lines();
                    if !board.spawn_piece() {
                        game_over = true;
                    }
                }
                last_update = now;
            }

            // Gravity well pull
            if now - last_gravity > gravity_well_speed {
                board.apply_gravity_wells();
                last_gravity = now;
            }
        } else {
            if is_key_pressed(KeyCode::R) {
                board = Board::new();
                board.wells.push(GravityWell { x: 2, y: 15, strength: 40.0 });
                board.wells.push(GravityWell { x: 7, y: 15, strength: 40.0 });
                board.spawn_piece();
                game_over = false;
            }
        }

        clear_background(BLACK);

        let sw = screen_width();
        let sh = screen_height();
        let cell_size = (sh * 0.8) / ROWS as f32;
        let board_w = cell_size * COLS as f32;
        let board_h = cell_size * ROWS as f32;
        let offset_x = (sw - board_w) / 2.0;
        let offset_y = (sh - board_h) / 2.0;

        board.draw(offset_x, offset_y, cell_size);

        if game_over {
            draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.8));
            let text = "GAME OVER";
            let font_size = 60.0;
            let dims = measure_text(text, None, font_size as u16, 1.0);
            draw_text(text, sw / 2.0 - dims.width / 2.0, sh / 2.0, font_size, RED);
            
            let sub_text = "Press R to Restart";
            let sub_font_size = 30.0;
            let sub_dims = measure_text(sub_text, None, sub_font_size as u16, 1.0);
            draw_text(sub_text, sw / 2.0 - sub_dims.width / 2.0, sh / 2.0 + 50.0, sub_font_size, WHITE);
        }

        next_frame().await;
    }
}
