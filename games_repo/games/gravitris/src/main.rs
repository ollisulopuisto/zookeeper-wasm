mod game;
mod input;

use macroquad::prelude::*;
use crate::game::{Board, COLS, ROWS, Difficulty};
use crate::input::InputManager;

const VERSION: &str = "26.04.11.240";

#[derive(Clone, PartialEq, Debug)]
enum AppState {
    Menu,
    Playing,
    GameOver,
}

#[macroquad::main("Gravitris")]
async fn main() {
    let mut board = Board::new(Difficulty::Normal);
    let mut input = InputManager::new();
    let mut state = AppState::Menu;
    
    let mut last_update = get_time();
    let mut last_gravity = get_time();
    let drop_speed = 0.5;
    let gravity_well_speed = 0.2;

    loop {
        // Dynamic virtual height: reserve space for controls if touch is active
        let virtual_width = 256.0;
        let virtual_height = if input.touch_active { 400.0 } else { 224.0 };
        input.update(virtual_width, virtual_height);

        let sw = screen_width();
        let sh = screen_height();
        let scale_x = sw / virtual_width;
        let scale_y = sh / virtual_height;
        let scale = scale_x.min(scale_y);
        
        let vx = (sw - virtual_width * scale) / 2.0;
        let vy = (sh - virtual_height * scale) / 2.0;

        match state {
            AppState::Menu => {
                if is_key_pressed(KeyCode::Key1) {
                    board = Board::new(Difficulty::Easy);
                    board.spawn_piece();
                    state = AppState::Playing;
                } else if is_key_pressed(KeyCode::Key2) {
                    board = Board::new(Difficulty::Normal);
                    board.spawn_piece();
                    state = AppState::Playing;
                } else if is_key_pressed(KeyCode::Key3) {
                    board = Board::new(Difficulty::Hard);
                    board.spawn_piece();
                    state = AppState::Playing;
                } else if is_mouse_button_pressed(MouseButton::Left) {
                    let mx = mouse_position().0;
                    let my = mouse_position().1;
                    
                    let btn_w = 100.0 * scale;
                    let _btn_h = 30.0 * scale;
                    let btn_x = vx + (virtual_width / 2.0) * scale - btn_w / 2.0;
                    
                    if mx >= btn_x && mx <= btn_x + btn_w {
                        if my >= vy + 80.0 * scale && my <= vy + 110.0 * scale {
                            board = Board::new(Difficulty::Easy);
                            board.spawn_piece();
                            state = AppState::Playing;
                        } else if my >= vy + 120.0 * scale && my <= vy + 150.0 * scale {
                            board = Board::new(Difficulty::Normal);
                            board.spawn_piece();
                            state = AppState::Playing;
                        } else if my >= vy + 160.0 * scale && my <= vy + 190.0 * scale {
                            board = Board::new(Difficulty::Hard);
                            board.spawn_piece();
                            state = AppState::Playing;
                        }
                    }
                }
            }
            AppState::Playing => {
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
                    let cleared = board.clear_lines();
                    if cleared > 0 {
                        board.add_lines_cleared(cleared as u32);
                    }
                    if !board.spawn_piece() {
                        state = AppState::GameOver;
                    }
                }

                // Normal falling
                if now - last_update > drop_speed {
                    if !board.move_piece(0, 1) {
                        board.lock_piece();
                        let cleared = board.clear_lines();
                        if cleared > 0 {
                            board.add_lines_cleared(cleared as u32);
                        }
                        if !board.spawn_piece() {
                            state = AppState::GameOver;
                        }
                    }
                    last_update = now;
                }

                // Gravity well pull
                if now - last_gravity > gravity_well_speed {
                    board.apply_gravity_wells();
                    last_gravity = now;
                }
            }
            AppState::GameOver => {
                if input.any_key || is_mouse_button_pressed(MouseButton::Left) {
                    state = AppState::Menu;
                }
            }
        }

        clear_background(BLACK);

        if state == AppState::Playing || state == AppState::GameOver {
            let cell_size = (224.0 * scale * 0.8) / ROWS as f32;
            let board_w = cell_size * COLS as f32;
            let board_h = cell_size * ROWS as f32;
            let offset_x = vx + (virtual_width * scale - board_w) / 2.0;
            let offset_y = vy + (224.0 * scale - board_h) / 2.0;

            board.draw(offset_x, offset_y, cell_size);

            // Draw HUD
            let hud_font_size = 15.0 * scale;
            draw_text(&format!("SCORE: {:06}", board.score), vx + 10.0 * scale, vy + 20.0 * scale, hud_font_size, GREEN);
            draw_text(&format!("LEVEL: {}", board.level), vx + 10.0 * scale, vy + 40.0 * scale, hud_font_size, YELLOW);
            draw_text(&format!("LINES: {}", board.lines_cleared_total), vx + 10.0 * scale, vy + 60.0 * scale, hud_font_size, WHITE);
            draw_text(&format!("MODE: {:?}", board.difficulty), vx + 10.0 * scale, vy + 80.0 * scale, hud_font_size, GRAY);

            // Draw Touch Controls
            input.draw_controls(vx, vy, scale, virtual_width, virtual_height);
        }

        if state == AppState::Menu {
            let title_size = 40.0 * scale;
            let text_size = 20.0 * scale;
            let center_x = vx + (virtual_width / 2.0) * scale;
            
            let title = "GRAVITRIS";
            let t_dims = measure_text(title, None, title_size as u16, 1.0);
            draw_text(title, center_x - t_dims.width / 2.0, vy + 60.0 * scale, title_size, MAGENTA);
            
            let btn_w = 100.0 * scale;
            let btn_h = 30.0 * scale;
            let btn_x = center_x - btn_w / 2.0;

            // Easy
            draw_rectangle(btn_x, vy + 80.0 * scale, btn_w, btn_h, GREEN);
            let e_text = "1: EASY";
            let e_dims = measure_text(e_text, None, text_size as u16, 1.0);
            draw_text(e_text, center_x - e_dims.width / 2.0, vy + 102.0 * scale, text_size, BLACK);

            // Normal
            draw_rectangle(btn_x, vy + 120.0 * scale, btn_w, btn_h, YELLOW);
            let n_text = "2: NORMAL";
            let n_dims = measure_text(n_text, None, text_size as u16, 1.0);
            draw_text(n_text, center_x - n_dims.width / 2.0, vy + 142.0 * scale, text_size, BLACK);

            // Hard
            draw_rectangle(btn_x, vy + 160.0 * scale, btn_w, btn_h, RED);
            let h_text = "3: HARD";
            let h_dims = measure_text(h_text, None, text_size as u16, 1.0);
            draw_text(h_text, center_x - h_dims.width / 2.0, vy + 182.0 * scale, text_size, BLACK);
        }

        // Draw Version
        let v_str = format!("v{}", VERSION);
        let v_size = 12.0 * scale;
        let v_dims = measure_text(&v_str, None, v_size as u16, 1.0);
        draw_text(&v_str, sw - v_dims.width - 5.0, sh - 5.0, v_size, GRAY);

        if state == AppState::GameOver {
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
