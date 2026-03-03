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

    loop {
        // Handle input using virtual resolution
        input.update(VIRTUAL_WIDTH, VIRTUAL_HEIGHT);

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

        // Clear the whole screen
        clear_background(BLACK);

        // Set camera to scale virtual resolution to fit screen
        set_camera(&Camera2D {
            zoom: vec2(scale * 2.0 / sw, -scale * 2.0 / sh),
            target: vec2(VIRTUAL_WIDTH / 2.0, VIRTUAL_HEIGHT / 2.0),
            ..Default::default()
        });

        match state {
            AppState::Menu => {
                draw_text("BUBBLES", 60.0, 60.0, 40.0, SKYBLUE);
                draw_text("1: 1 PLAYER", 80.0, 100.0, 20.0, WHITE);
                draw_text("2: 2 PLAYERS", 80.0, 120.0, 20.0, WHITE);
                draw_text("TOUCH TO START", 75.0, 160.0, 20.0, YELLOW);
            }
            AppState::Playing | AppState::GameOver => {
                game.draw(&gfx);
                if !two_player {
                    input.draw_debug_touch_regions(VIRTUAL_WIDTH, VIRTUAL_HEIGHT);
                }
            }
            AppState::Leaderboard => {
                draw_text("HISCORES", 80.0, 40.0, 30.0, MAGENTA);
                let scores = storage::load_scores();
                for (i, s) in scores.iter().enumerate() {
                    draw_text(&format!("{}. {} {:06}", i + 1, s.name, s.score), 60.0, 70.0 + (i as f32 * 15.0), 20.0, WHITE);
                }
                draw_text("PRESS ANY KEY", 75.0, 210.0, 15.0, GRAY);
            }
        }

        set_default_camera();
        next_frame().await;
    }
}
