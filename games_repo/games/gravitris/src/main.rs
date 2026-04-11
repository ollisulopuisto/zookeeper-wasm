mod game;
mod input;

use macroquad::prelude::*;
use crate::game::{Board, COLS, ROWS, GravityWell};
use crate::input::InputManager;

const VERSION: &str = "26.04.11.238";

#[macroquad::main("Gravitris")]
async fn main() {
    let mut board = Board::new();
    board.wells.push(GravityWell { x: 2, y: 15, strength: 40.0 });
    board.wells.push(GravityWell { x: 7, y: 15, strength: 40.0 });
    board.spawn_piece();

    let mut input = InputManager::new();
    let mut last_update = get_time();
    let mut last_gravity = get_time();
    let drop_speed = 0.5;
    let gravity_well_speed = 0.2;
    let mut game_over = false;

    loop {
        // Dynamic virtual height: reserve space for controls if touch is active
        let virtual_width = 256.0;
        let virtual_height = if input.touch_active { 400.0 } else { 224.0 };
        input.update(virtual_width, virtual_height);

        if !game_over {
            let now = get_time();

            // Handle Input
            if input.p1.left {
                board.move_piece(-1, 0);
            }
            if input.p1.right {
                board.move_piece(1, 0);
            }
            if input.p1.down {
                board.move_piece(0, 1);
            }
            if input.p1.rotate {
                board.rotate_piece();
            }
            if input.p1.drop {
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
            if input.any_key || is_mouse_button_pressed(MouseButton::Left) {
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
        let scale_x = sw / virtual_width;
        let scale_y = sh / virtual_height;
        let scale = scale_x.min(scale_y);
        
        let vx = (sw - virtual_width * scale) / 2.0;
        let vy = (sh - virtual_height * scale) / 2.0;

        let cell_size = (224.0 * scale * 0.8) / ROWS as f32;
        let board_w = cell_size * COLS as f32;
        let board_h = cell_size * ROWS as f32;
        let offset_x = vx + (virtual_width * scale - board_w) / 2.0;
        let offset_y = vy + (224.0 * scale - board_h) / 2.0;

        board.draw(offset_x, offset_y, cell_size);

        // Draw Touch Controls
        input.draw_controls(vx, vy, scale, virtual_width, virtual_height);

        // Draw Version
        let v_str = format!("v{}", VERSION);
        let v_size = 12.0 * scale;
        let v_dims = measure_text(&v_str, None, v_size as u16, 1.0);
        draw_text(&v_str, sw - v_dims.width - 5.0, sh - 5.0, v_size, GRAY);

        if game_over {
            draw_rectangle(0.0, 0.0, sw, sh, Color::new(0.0, 0.0, 0.0, 0.8));
            let text = "GAME OVER";
            let font_size = 40.0 * scale;
            let dims = measure_text(text, None, font_size as u16, 1.0);
            draw_text(text, sw / 2.0 - dims.width / 2.0, sh / 2.0, font_size, RED);
            
            let sub_text = "TAP to Restart";
            let sub_font_size = 20.0 * scale;
            let sub_dims = measure_text(sub_text, None, sub_font_size as u16, 1.0);
            draw_text(sub_text, sw / 2.0 - sub_dims.width / 2.0, sh / 2.0 + 40.0 * scale, sub_font_size, WHITE);
        }

        next_frame().await;
    }
}
