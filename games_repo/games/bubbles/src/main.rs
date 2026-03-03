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
                    // Save high scores
                    for p in &game.players {
                        storage::add_score("BUB".to_string(), p.score);
                    }
                }
            }
            AppState::GameOver => {
                if input.any_key {
                    state = AppState::Leaderboard;
                }
            }
            AppState::Leaderboard => {
                if input.any_key {
                    state = AppState::Menu;
                }
            }
        }

        // Calculate scaling to fit screen while maintaining aspect ratio
        let sw = screen_width();
        let sh = screen_height();
        let scale = (sw / VIRTUAL_WIDTH).min(sh / VIRTUAL_HEIGHT);
        let vx = (sw - VIRTUAL_WIDTH * scale) / 2.0;
        let vy = (sh - VIRTUAL_HEIGHT * scale) / 2.0;

        // Clear the whole screen
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
            AppState::Playing | AppState::GameOver => {
                game.draw(&gfx, vx, vy, scale);
                if !two_player && touch_detected {
                    input.draw_debug_touch_regions(vx, vy, scale, VIRTUAL_WIDTH, VIRTUAL_HEIGHT);
                }
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
