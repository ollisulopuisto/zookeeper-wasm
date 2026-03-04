mod audio;
mod gfx;
mod input;
mod storage;
mod game;

use macroquad::prelude::*;
use crate::audio::AudioManager;
use crate::gfx::SpriteManager;
use crate::input::InputManager;
use crate::game::{Game, VIRTUAL_WIDTH, VIRTUAL_HEIGHT};

#[derive(PartialEq)]
enum AppState {
    Menu,
    Playing,
    GameOver,
    EnteringName { name: String },
    Leaderboard,
}

#[macroquad::main("Bubbles")]
async fn main() {
    let audio = AudioManager::new().await;
    let gfx = SpriteManager::new().await;
    let mut input = InputManager::new();
    let mut game = Game::new(false);
    let mut state = AppState::Menu;
    let mut two_player = false;
    let mut touch_detected = false;

    loop {
        // Handle input using virtual resolution
        input.update(VIRTUAL_WIDTH, VIRTUAL_HEIGHT);
        if !touches().is_empty() {
            touch_detected = true;
        }

        match state {
            AppState::Menu => {
                if is_key_pressed(KeyCode::Key1) {
                    two_player = false;
                    game = Game::new(false);
                    state = AppState::Playing;
                    audio.play_music();
                } else if is_key_pressed(KeyCode::Key2) {
                    two_player = true;
                    game = Game::new(true);
                    state = AppState::Playing;
                    audio.play_music();
                } else if input.any_key {
                    // Default to 1P on touch
                    two_player = false;
                    game = Game::new(false);
                    state = AppState::Playing;
                    audio.play_music();
                }
            }
            AppState::Playing => {
                let inputs = if two_player {
                    vec![input.p1, input.p2]
                } else {
                    vec![input.p1]
                };
                game.update(&inputs, &audio);
                if game.game_over {
                    state = AppState::GameOver;
                    audio.stop_music();
                }
            }
            AppState::GameOver => {
                if input.any_key {
                    state = AppState::EnteringName { name: String::new() };
                }
            }
            AppState::EnteringName { ref mut name } => {
                while let Some(c) = get_char_pressed() {
                    if (c.is_alphanumeric() || c == ' ') && name.len() < 10 {
                        name.push(c);
                    }
                }
                if is_key_pressed(KeyCode::Backspace) { name.pop(); }
                
                let mut submitted = false;
                if is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter) {
                    submitted = true;
                }

                // UI calculations for button
                let sw = screen_width();
                let sh = screen_height();
                let scale = (sw / VIRTUAL_WIDTH).min(sh / VIRTUAL_HEIGHT);
                let vx = (sw - VIRTUAL_WIDTH * scale) / 2.0;
                let vy = (sh - VIRTUAL_HEIGHT * scale) / 2.0;
                let btn_w = 100.0 * scale;
                let btn_h = 30.0 * scale;
                let btn_x = vx + (VIRTUAL_WIDTH / 2.0) * scale - btn_w / 2.0;
                let btn_y = vy + 180.0 * scale;

                if is_mouse_button_pressed(MouseButton::Left) {
                    let (mx, my) = mouse_position();
                    if mx >= btn_x && mx <= btn_x + btn_w && my >= btn_y && my <= btn_y + btn_h {
                        submitted = true;
                    }
                    
                    // JS Prompt button
                    let js_btn_y = vy + 140.0 * scale;
                    if mx >= btn_x && mx <= btn_x + btn_w && my >= js_btn_y && my <= js_btn_y + btn_h {
                        *name = storage::ask_name_js();
                    }
                }

                if submitted {
                    let final_name = if name.is_empty() { "BUB".to_string() } else { name.clone() };
                    for p in &game.players {
                        storage::add_score(final_name.clone(), p.score);
                    }
                    state = AppState::Leaderboard;
                }
            }
            AppState::Leaderboard => {
                if input.any_key {
                    state = AppState::Menu;
                }
            }
        }

        // Scaling calculations
        let sw = screen_width();
        let sh = screen_height();
        let scale = (sw / VIRTUAL_WIDTH).min(sh / VIRTUAL_HEIGHT);
        let vx = (sw - VIRTUAL_WIDTH * scale) / 2.0;
        let vy = (sh - VIRTUAL_HEIGHT * scale) / 2.0;

        clear_background(BLACK);

        match state {
            AppState::Menu => {
                let title_size = 40.0 * scale;
                let text_size = 20.0 * scale;
                draw_text("BUBBLES", vx + 60.0 * scale, vy + 60.0 * scale, title_size, SKYBLUE);
                draw_text("1: 1 PLAYER", vx + 80.0 * scale, vy + 100.0 * scale, text_size, WHITE);
                draw_text("2: 2 PLAYERS", vx + 80.0 * scale, vy + 120.0 * scale, text_size, WHITE);
                draw_text("TOUCH TO START", vx + 75.0 * scale, vy + 160.0 * scale, text_size, YELLOW);
            }
            AppState::Playing => {
                game.draw(&gfx, vx, vy, scale);
                if !two_player && touch_detected {
                    input.draw_debug_touch_regions(vx, vy, scale, VIRTUAL_WIDTH, VIRTUAL_HEIGHT);
                }
            }
            AppState::GameOver => {
                let title_size = 40.0 * scale;
                let text_size = 25.0 * scale;
                draw_text("GAME OVER", vx + 45.0 * scale, vy + 60.0 * scale, title_size, RED);
                draw_text(&format!("P1 SCORE: {:06}", game.players[0].score), vx + 40.0 * scale, vy + 100.0 * scale, text_size, GREEN);
                if game.players.len() > 1 {
                    draw_text(&format!("P2 SCORE: {:06}", game.players[1].score), vx + 40.0 * scale, vy + 135.0 * scale, text_size, BLUE);
                }
                let blink = (get_time() * 2.0) as i32 % 2 == 0;
                if blink {
                    draw_text("PRESS ANY KEY", vx + 55.0 * scale, vy + 190.0 * scale, 20.0 * scale, YELLOW);
                }
            }
            AppState::EnteringName { ref name } => {
                let title_size = 30.0 * scale;
                draw_text("NEW HIGH SCORE!", vx + 40.0 * scale, vy + 60.0 * scale, title_size, YELLOW);
                draw_text("TYPE YOUR NAME:", vx + 50.0 * scale, vy + 100.0 * scale, 20.0 * scale, WHITE);
                let display_name = if name.is_empty() { "_".to_string() } else { format!("{}_", name) };
                draw_text(&display_name, vx + 80.0 * scale, vy + 130.0 * scale, 30.0 * scale, SKYBLUE);
                
                let btn_w = 100.0 * scale;
                let btn_h = 30.0 * scale;
                let btn_x = vx + (VIRTUAL_WIDTH / 2.0) * scale - btn_w / 2.0;
                
                // JS Popup button for mobile
                draw_rectangle(btn_x, vy + 140.0 * scale, btn_w, btn_h, Color::new(0.2, 0.2, 0.2, 1.0));
                draw_text("TAP FOR POPUP", btn_x + 5.0 * scale, vy + 160.0 * scale, 12.0 * scale, WHITE);

                // OK button
                draw_rectangle(btn_x, vy + 180.0 * scale, btn_w, btn_h, Color::new(0.3, 0.8, 0.3, 1.0));
                draw_text("OK", btn_x + 40.0 * scale, vy + 200.0 * scale, 20.0 * scale, WHITE);
            }
            AppState::Leaderboard => {
                let title_size = 30.0 * scale;
                let text_size = 15.0 * scale;
                draw_text("HISCORES", vx + 80.0 * scale, vy + 40.0 * scale, title_size, MAGENTA);
                let scores = storage::load_scores();
                for (i, s) in scores.iter().enumerate() {
                    draw_text(&format!("{}. {} {:06}", i + 1, s.name, s.score), vx + 60.0 * scale, vy + 70.0 * scale + (i as f32 * 15.0 * scale), text_size, WHITE);
                }
                draw_text("PRESS ANY KEY", vx + 75.0 * scale, vy + 210.0 * scale, text_size, GRAY);
            }
        }

        next_frame().await;
    }
}
