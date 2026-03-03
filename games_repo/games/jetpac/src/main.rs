mod physics;
mod game;
mod audio;

use macroquad::prelude::*;
use game::{Player, Platform, Enemy, Item, Laser, Rocket, RocketPart, PartType, SCREEN_WIDTH, SCREEN_HEIGHT};
use audio::AudioManager;

fn conf() -> Conf {
    Conf {
        window_title: "Jetpac Clone".to_string(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        ..Default::default()
    }
}

enum GameState {
    Menu,
    Playing,
    GameOver,
    Victory,
}

#[macroquad::main(conf)]
async fn main() {
    let audio = AudioManager::new().await;
    let mut player = Player::new();
    let mut lasers: Vec<Laser> = Vec::new();
    let platforms = vec![
        Platform { x: 100.0, y: 150.0, width: 150.0 },
        Platform { x: 500.0, y: 250.0, width: 200.0 },
        Platform { x: 200.0, y: 400.0, width: 150.0 },
    ];
    let mut enemies = vec![
        Enemy::new(0.0, 100.0, 200.0),
        Enemy::new(SCREEN_WIDTH, 300.0, -150.0),
    ];
    let mut item = Item::new(400.0, 50.0);
    
    let mut rocket = Rocket::new(600.0, SCREEN_HEIGHT - 20.0);
    let mut parts = vec![
        RocketPart::new(100.0, 50.0, PartType::Middle),
        RocketPart::new(500.0, 50.0, PartType::Top),
    ];

    let mut state = GameState::Menu;
    let mut frame_count = 0;

    loop {
        frame_count += 1;
        let camera = Camera2D {
            target: vec2(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
            zoom: vec2(2.0 / SCREEN_WIDTH, -2.0 / SCREEN_HEIGHT),
            ..Default::default()
        };

        match state {
            GameState::Menu => {
                // Input polling (with default camera for reliability)
                let start_keyed = is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter);
                let start_clicked = is_mouse_button_pressed(MouseButton::Left) || (frame_count > 60 && is_mouse_button_down(MouseButton::Left));
                let start_touched = touches().iter().any(|t| t.phase == TouchPhase::Started);

                if start_keyed || start_clicked || start_touched {
                    state = GameState::Playing;
                }

                // Drawing (with world camera)
                set_camera(&camera);
                clear_background(BLACK);
                draw_text("JETPAC", SCREEN_WIDTH / 2.0 - 60.0, 80.0, 50.0, YELLOW);
                
                let instr = [
                    "INSTRUCTIONS",
                    "ARROWS/WASD or Touch Left side: Move",
                    "(Left 1/4 = Left, 2/4 = Right)",
                    "SPACE/ENTER or Touch Right side: Shoot",
                    "(Top Right = Fly, Bottom Right = Shoot)",
                    "Build rocket at launch site",
                    "Collect blue fuel squares",
                    "Fly into fueled rocket to launch!",
                ];

                let start_y = 150.0;
                for (i, line) in instr.iter().enumerate() {
                    let size = if i == 0 { 30.0 } else if line.starts_with("(") { 16.0 } else { 20.0 };
                    let color = if i == 0 { WHITE } else if line.starts_with("(") { GRAY } else { LIGHTGRAY };
                    let t_measure = measure_text(line, None, size as u16, 1.0);
                    draw_text(line, SCREEN_WIDTH / 2.0 - t_measure.width / 2.0, start_y + i as f32 * 35.0, size, color);
                }

                let start_text = "Press SPACE or TAP to START";
                let t_width = measure_text(start_text, None, 30, 1.0).width;
                draw_text(start_text, SCREEN_WIDTH / 2.0 - t_width / 2.0, 500.0, 30.0, GREEN);
            }
            GameState::Victory => {
                // Input polling
                let restart_keyed = is_key_pressed(KeyCode::R) || is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter);
                let restart_clicked = is_mouse_button_pressed(MouseButton::Left);
                let restart_touched = touches().iter().any(|t| t.phase == TouchPhase::Started);

                if restart_keyed || restart_clicked || restart_touched {
                    player = Player::new();
                    lasers.clear();
                    enemies = vec![
                        Enemy::new(0.0, 100.0, 200.0),
                        Enemy::new(SCREEN_WIDTH, 300.0, -150.0),
                    ];
                    item = Item::new(400.0, 50.0);
                    rocket = Rocket::new(600.0, SCREEN_HEIGHT - 20.0);
                    parts = vec![
                        RocketPart::new(100.0, 50.0, PartType::Middle),
                        RocketPart::new(500.0, 50.0, PartType::Top),
                    ];
                    state = GameState::Playing;
                }

                // Drawing
                set_camera(&camera);
                clear_background(BLACK);
                let msg = "ROCKET LAUNCHED!";
                let m_width = measure_text(msg, None, 40, 1.0).width;
                draw_text(msg, SCREEN_WIDTH / 2.0 - m_width / 2.0, SCREEN_HEIGHT / 2.0, 40.0, GREEN);
                
                let sub = "Press R or TAP to play again";
                let s_width = measure_text(sub, None, 20, 1.0).width;
                draw_text(sub, SCREEN_WIDTH / 2.0 - s_width / 2.0, SCREEN_HEIGHT / 2.0 + 40.0, 20.0, WHITE);
            }
            GameState::GameOver => {
                // Input polling
                let restart_keyed = is_key_pressed(KeyCode::R) || is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter);
                let restart_clicked = is_mouse_button_pressed(MouseButton::Left);
                let restart_touched = touches().iter().any(|t| t.phase == TouchPhase::Started);

                if restart_keyed || restart_clicked || restart_touched {
                    player = Player::new();
                    lasers.clear();
                    enemies = vec![
                        Enemy::new(0.0, 100.0, 200.0),
                        Enemy::new(SCREEN_WIDTH, 300.0, -150.0),
                    ];
                    item = Item::new(400.0, 50.0);
                    rocket = Rocket::new(600.0, SCREEN_HEIGHT - 20.0);
                    parts = vec![
                        RocketPart::new(100.0, 50.0, PartType::Middle),
                        RocketPart::new(500.0, 50.0, PartType::Top),
                    ];
                    state = GameState::Playing;
                }

                // Drawing
                set_camera(&camera);
                clear_background(BLACK);
                let msg = "GAME OVER";
                let m_width = measure_text(msg, None, 40, 1.0).width;
                draw_text(msg, SCREEN_WIDTH / 2.0 - m_width / 2.0, SCREEN_HEIGHT / 2.0, 40.0, RED);
                
                let sub = "Press R or TAP to restart";
                let s_width = measure_text(sub, None, 20, 1.0).width;
                draw_text(sub, SCREEN_WIDTH / 2.0 - s_width / 2.0, SCREEN_HEIGHT / 2.0 + 40.0, 20.0, WHITE);
            }
            GameState::Playing => {
                let dt = get_frame_time();

                // Update (Input polling)
                if player.update(dt, &mut lasers) {
                    audio.play_laser();
                }
                if player.is_jetting {
                    audio.play_jet();
                }
                
                // ... (rest of update logic)
                item.update(dt);
                
                for platform in &platforms {
                    platform.check_collision(&mut player.entity);
                    platform.check_collision(&mut item.entity);
                    for part in &mut parts {
                        platform.check_collision(&mut part.entity);
                    }
                }

                for enemy in &mut enemies {
                    enemy.update(dt);
                    // Collision with player
                    if player.entity.x < enemy.entity.x + enemy.entity.width &&
                       player.entity.x + player.entity.width > enemy.entity.x &&
                       player.entity.y < enemy.entity.y + enemy.entity.height &&
                       player.entity.y + player.entity.height > enemy.entity.y {
                        state = GameState::GameOver;
                        audio.play_game_over();
                    }
                }

                for laser in &mut lasers {
                    laser.update(dt);
                }
                lasers.retain(|l| l.lifetime > 0.0);

                for part in &mut parts {
                    part.update(dt);
                }

                // Simple collision with item
                if !item.collected && 
                   player.entity.x < item.entity.x + item.entity.width &&
                   player.entity.x + player.entity.width > item.entity.x &&
                   player.entity.y < item.entity.y + item.entity.height &&
                   player.entity.y + player.entity.height > item.entity.y {
                    item.collected = true;
                    audio.play_pickup();
                    // If rocket is assembled, item counts as fuel
                    if rocket.parts_attached.len() == 3 {
                        rocket.fuel_level = (rocket.fuel_level + 0.2).min(1.0);
                        item.collected = false; // "respawn" item for more fuel
                        item.entity.x = rand::gen_range(0.0, SCREEN_WIDTH - 20.0);
                        item.entity.y = 0.0;
                    }
                }

                // Picking up rocket parts
                if player.holding_part.is_none() {
                    for part in &mut parts {
                        if !part.is_attached && !part.is_held &&
                           player.entity.x < part.entity.x + part.entity.width &&
                           player.entity.x + player.entity.width > part.entity.x &&
                           player.entity.y < part.entity.y + part.entity.height &&
                           player.entity.y + player.entity.height > part.entity.y {
                            player.holding_part = Some(part.part_type);
                            part.is_held = true;
                            audio.play_pickup();
                            break;
                        }
                    }
                }

                // Dropping off parts at rocket
                if let Some(held_part) = player.holding_part {
                    // If player is above rocket launch site
                    if player.entity.x > rocket.x - 20.0 && player.entity.x < rocket.x + 60.0 {
                        let next_needed = match rocket.parts_attached.len() {
                            1 => Some(PartType::Middle),
                            2 => Some(PartType::Top),
                            _ => None,
                        };

                        if Some(held_part) == next_needed {
                            rocket.parts_attached.push(held_part);
                            player.holding_part = None;
                            audio.play_pickup();
                            // Find and mark the part as attached
                            if let Some(p) = parts.iter_mut().find(|p| p.part_type == held_part) {
                                p.is_attached = true;
                            }
                        }
                    }
                }

                // Collision: Lasers vs Enemies
                enemies.retain(|enemy| {
                    let mut hit = false;
                    for laser in &lasers {
                        if laser.x > enemy.entity.x && laser.x < enemy.entity.x + enemy.entity.width &&
                           laser.y > enemy.entity.y && laser.y < enemy.entity.y + enemy.entity.height {
                            hit = true;
                            audio.play_explosion();
                            break;
                        }
                    }
                    !hit
                });

                // Drawing
                set_camera(&camera);
                clear_background(BLACK);
                
                for platform in &platforms {
                    platform.draw();
                }
                
                rocket.draw();
                item.draw();
                
                for part in &parts {
                    part.draw();
                }
                
                for enemy in &enemies {
                    enemy.draw();
                }
                
                for laser in &lasers {
                    laser.draw();
                }
                
                player.draw();

                if rocket.fuel_level >= 1.0 {
                    draw_text("READY TO LAUNCH!", SCREEN_WIDTH / 2.0 - 100.0, SCREEN_HEIGHT / 2.0, 30.0, GREEN);
                    // Check for player entry to launch
                    if player.entity.x > rocket.x - 20.0 && player.entity.x < rocket.x + 60.0 &&
                       player.entity.y > rocket.y - 60.0 {
                        state = GameState::Victory;
                        audio.play_win();
                    }
                }

                draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 20.0, WHITE);
            }
        }
        
        set_default_camera();
        next_frame().await;
    }
}

