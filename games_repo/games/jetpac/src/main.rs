mod physics;
mod game;
mod audio;

use macroquad::prelude::*;
use game::{Player, Enemy, EnemyType, Level, TileType, CollectibleType, SCREEN_WIDTH, SCREEN_HEIGHT, TILE_SIZE, COLS, ROWS, create_test_level};
use physics::RectCollider;
use audio::AudioManager;

fn conf() -> Conf {
    Conf {
        window_title: "Jetpack DOS Clone".to_string(),
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

fn check_collision(level: &Level, collider: &RectCollider) -> bool {
    let left_col = (collider.x / TILE_SIZE).floor() as i32;
    let right_col = ((collider.x + collider.w - 0.1) / TILE_SIZE).floor() as i32;
    let top_row = (collider.y / TILE_SIZE).floor() as i32;
    let bottom_row = ((collider.y + collider.h - 0.1) / TILE_SIZE).floor() as i32;

    for r in top_row..=bottom_row {
        for c in left_col..=right_col {
            if c >= 0 && c < COLS as i32 && r >= 0 && r < ROWS as i32 {
                let tile = level.grid[r as usize][c as usize];
                if tile == TileType::NormalBrick || tile == TileType::SolidBrick {
                    return true;
                }
            } else {
                return true; // Out of bounds is solid
            }
        }
    }
    false
}

fn check_spikes(level: &Level, collider: &RectCollider) -> bool {
    let left_col = (collider.x / TILE_SIZE).floor() as i32;
    let right_col = ((collider.x + collider.w - 0.1) / TILE_SIZE).floor() as i32;
    let top_row = (collider.y / TILE_SIZE).floor() as i32;
    let bottom_row = ((collider.y + collider.h - 0.1) / TILE_SIZE).floor() as i32;

    for r in top_row..=bottom_row {
        for c in left_col..=right_col {
            if c >= 0 && c < COLS as i32 && r >= 0 && r < ROWS as i32 {
                if level.grid[r as usize][c as usize] == TileType::Spikes {
                    return true;
                }
            }
        }
    }
    false
}

fn get_tile_at(level: &Level, x: f32, y: f32) -> TileType {
    let c = (x / TILE_SIZE).floor() as i32;
    let r = (y / TILE_SIZE).floor() as i32;
    if c >= 0 && c < COLS as i32 && r >= 0 && r < ROWS as i32 {
        level.grid[r as usize][c as usize]
    } else {
        TileType::SolidBrick
    }
}

#[macroquad::main(conf)]
async fn main() {
    let audio = AudioManager::new().await;
    let tex_rocket = load_texture("assets/1f680.png").await.ok();
    if let Some(ref tex) = tex_rocket { tex.set_filter(FilterMode::Linear); }
    let mut state = GameState::Menu;
    let mut frame_count = 0;

    let mut level = create_test_level();
    let mut player = Player::new(2, 13);
    let mut enemies = vec![
        Enemy::new(5, 5, EnemyType::Trackbot),
        Enemy::new(12, 5, EnemyType::Trackbot),
        Enemy::new(15, 17, EnemyType::SteelBall),
        Enemy::new(5, 17, EnemyType::Spring),
    ];

    loop {
        frame_count += 1;
        let sw = screen_width();
        let sh = screen_height();

        match state {
            GameState::Menu => {
                let start_keyed = is_key_pressed(KeyCode::Space) || is_key_pressed(KeyCode::Enter);
                let start_clicked = frame_count > 5 && is_mouse_button_pressed(MouseButton::Left);
                let start_touched = frame_count > 5 && touches().iter().any(|t| t.phase == TouchPhase::Started);

                if start_keyed || start_clicked || start_touched {
                    state = GameState::Playing;
                }

                clear_background(BLACK);
                let title = "JETPACK";
                let t_w = measure_text(title, None, 60, 1.0).width;
                draw_text(title, sw / 2.0 - t_w / 2.0, sh * 0.2, 60.0, GREEN);
                
                let instr = [
                    "ARROWS / WASD: Move & Climb",
                    "SPACE / J: Jetpack (uses fuel)",
                    "X / K: Phase Shifter (destroy bricks)",
                    "--- TOUCH CONTROLS ---",
                    "LEFT COL: Climb | MID COL: Move",
                    "RIGHT TOP: Jet | RIGHT BTM: Phase",
                    "Collect all Emeralds to open the Portal!"
                ];
                for (i, line) in instr.iter().enumerate() {
                    let w = measure_text(line, None, 20, 1.0).width;
                    draw_text(line, sw / 2.0 - w / 2.0, sh * 0.4 + i as f32 * 40.0, 20.0, WHITE);
                }
                
                if (get_time() * 3.0) as i32 % 2 == 0 {
                    let msg = "TAP OR PRESS SPACE TO START";
                    let mw = measure_text(msg, None, 30, 1.0).width;
                    draw_text(msg, sw / 2.0 - mw / 2.0, sh * 0.8, 30.0, YELLOW);
                }
            }
            GameState::Playing => {
                let dt = get_frame_time().min(0.05); // Cap delta time

                let camera = Camera2D {
                    target: vec2(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
                    zoom: vec2(2.0 / SCREEN_WIDTH, 2.0 / SCREEN_HEIGHT),
                    ..Default::default()
                };
                set_camera(&camera);

                // --- Player Update ---
                if !player.dead {
                    let mut dx = 0.0;
                    let mut dy = 0.0;
                    
                    // Input mapping
                    let mut move_left = is_key_down(KeyCode::Left) || is_key_down(KeyCode::A);
                    let mut move_right = is_key_down(KeyCode::Right) || is_key_down(KeyCode::D);
                    let mut move_up = is_key_down(KeyCode::Up) || is_key_down(KeyCode::W);
                    let mut move_down = is_key_down(KeyCode::Down) || is_key_down(KeyCode::S);
                    let mut jet_btn = is_key_down(KeyCode::Space) || is_key_down(KeyCode::J);
                    let mut phase_btn = is_key_pressed(KeyCode::X) || is_key_pressed(KeyCode::K);

                    // Touch mapping
                    for touch in touches() {
                        let tx = touch.position.x / sw * SCREEN_WIDTH;
                        let ty = touch.position.y / sh * SCREEN_HEIGHT;

                        if tx < SCREEN_WIDTH / 3.0 {
                            if ty < SCREEN_HEIGHT / 2.0 { move_up = true; }
                            else { move_down = true; }
                        } else if tx < 2.0 * SCREEN_WIDTH / 3.0 {
                            if tx < SCREEN_WIDTH / 2.0 { move_left = true; }
                            else { move_right = true; }
                        } else {
                            if ty < SCREEN_HEIGHT / 2.0 { jet_btn = true; }
                            else if touch.phase == TouchPhase::Started { phase_btn = true; }
                        }
                    }

                    // Ground Check
                    player.entity.collider.y += 1.0;
                    let is_on_ground = check_collision(&level, &player.entity.collider);
                    player.entity.collider.y -= 1.0;

                    // Jetpack logic
                    player.is_jetting = false;
                    if jet_btn && player.fuel > 0.0 {
                        player.is_jetting = true;
                        player.fuel -= 30.0 * dt;
                        if player.entity.vy > -200.0 {
                            player.entity.vy -= 1200.0 * dt; // Stronger thrust
                        }
                    } else {
                        // Gravity
                        player.entity.vy += 800.0 * dt;
                    }

                    // Jumping logic
                    if move_up && is_on_ground && !player.is_jetting {
                        player.entity.vy = -350.0;
                        audio.play_jump();
                    }

                    // --- Ladder logic ---
                    let center_x = player.entity.collider.x + player.entity.collider.w / 2.0;
                    let center_y = player.entity.collider.y + player.entity.collider.h / 2.0;
                    let current_tile = get_tile_at(&level, center_x, center_y);
                    let on_ladder = current_tile == TileType::Ladder || current_tile == TileType::UpLadder;
                    
                    if on_ladder && !player.is_jetting {
                        let climb_speed = if current_tile == TileType::UpLadder { 250.0 } else { 150.0 };
                        player.entity.vy = 0.0;
                        if move_up { dy -= climb_speed * dt; }
                        if move_down { dy += climb_speed * dt; }
                    }

                    // --- Energy Charger/Drain Logic ---
                    if current_tile == TileType::EnergyCharger {
                        player.fuel = (player.fuel + 60.0 * dt).min(100.0);
                    } else if current_tile == TileType::EnergyDrain {
                        player.fuel = (player.fuel - 40.0 * dt).max(0.0);
                    }

                    // Horizontal movement
                    let speed = 180.0;
                    if move_left { dx -= speed * dt; player.facing_right = false; }
                    if move_right { dx += speed * dt; player.facing_right = true; }

                    // Apply horizontal physics
                    player.entity.collider.x += dx;
                    if check_collision(&level, &player.entity.collider) {
                        player.entity.collider.x -= dx;
                    }

                    // Apply vertical physics
                    player.entity.collider.y += dy + player.entity.vy * dt;
                    if check_collision(&level, &player.entity.collider) {
                        player.entity.collider.y -= dy + player.entity.vy * dt;
                        player.entity.vy = 0.0;
                    }

                    // Spike Death
                    if check_spikes(&level, &player.entity.collider) {
                        player.dead = true;
                        state = GameState::GameOver;
                        audio.stop_jet();
                        audio.play_game_over();
                    }

                    // Phase Shifter logic
                    if player.phase_cooldown > 0.0 {
                        player.phase_cooldown -= dt;
                    }
                    if phase_btn && player.phase_cooldown <= 0.0 {
                        // Find tile directly in front
                        let pc_x = player.entity.collider.x + if player.facing_right { TILE_SIZE } else { -TILE_SIZE/2.0 };
                        let pc_y = player.entity.collider.y + player.entity.collider.h / 2.0; 
                        let c = (pc_x / TILE_SIZE).floor() as usize;
                        let r = (pc_y / TILE_SIZE).floor() as usize;
                        
                        if c < COLS && r < ROWS && level.grid[r][c] == TileType::NormalBrick {
                            level.grid[r][c] = TileType::Empty;
                            level.phased_bricks.push(game::PhasedBrick { col: c, row: r, timer: 4.0 });
                            player.phase_cooldown = 0.4;
                            audio.play_phase();
                        }
                    }

                    // Audio
                    if player.is_jetting { audio.start_jet(); }
                    else { audio.stop_jet(); }

                    // Collectibles
                    for col in &mut level.collectibles {
                        if col.active {
                            let cx = col.col as f32 * TILE_SIZE;
                            let cy = col.row as f32 * TILE_SIZE;
                            let col_rect = RectCollider::new(cx, cy, TILE_SIZE, TILE_SIZE);
                            
                            if player.entity.collider.overlaps(&col_rect) {
                                col.active = false;
                                match col.ctype {
                                    CollectibleType::Emerald => {
                                        level.emeralds_collected += 1;
                                        audio.play_gem();
                                        if level.emeralds_collected >= level.emeralds_total {
                                            level.exit_door.active = true;
                                            audio.play_portal();
                                        }
                                    }
                                    CollectibleType::Fuel => {
                                        player.fuel = (player.fuel + 50.0).min(100.0);
                                        audio.play_fuel();
                                    }
                                }
                            }
                        }
                    }

                    // Exit Door completion
                    if level.exit_door.active {
                        if level.exit_door.opening_progress < 1.0 {
                            level.exit_door.opening_progress = (level.exit_door.opening_progress + dt).min(1.0);
                        }
                        let ex = level.exit_door.col as f32 * TILE_SIZE + TILE_SIZE / 2.0;
                        let ey = level.exit_door.row as f32 * TILE_SIZE + TILE_SIZE / 2.0;
                        let exit_rect = RectCollider::new(ex, ey, TILE_SIZE, TILE_SIZE * 2.0);
                        if player.entity.collider.overlaps(&exit_rect) && level.exit_door.opening_progress >= 1.0 {
                            state = GameState::Victory;
                            audio.stop_jet();
                        }
                    }
                }

                // --- Environment Update ---
                let mut p_idx = 0;
                while p_idx < level.phased_bricks.len() {
                    level.phased_bricks[p_idx].timer -= dt;
                    if level.phased_bricks[p_idx].timer <= 0.0 {
                        let c = level.phased_bricks[p_idx].col;
                        let r = level.phased_bricks[p_idx].row;
                        level.grid[r][c] = TileType::NormalBrick;
                        
                        // Check if player is telefragged
                        let brick_rect = RectCollider::new(c as f32 * TILE_SIZE, r as f32 * TILE_SIZE, TILE_SIZE, TILE_SIZE);
                        if player.entity.collider.overlaps(&brick_rect) {
                            player.dead = true;
                            state = GameState::GameOver;
                            audio.stop_jet();
                            audio.play_game_over();
                        }
                        level.phased_bricks.remove(p_idx);
                    } else {
                        p_idx += 1;
                    }
                }

                // --- Enemy Update ---
                for enemy in &mut enemies {
                    match enemy.etype {
                        EnemyType::Trackbot => {
                            let dx = if enemy.facing_right { 100.0 * dt } else { -100.0 * dt };
                            enemy.entity.vy += 600.0 * dt; // Gravity
                            
                            enemy.entity.collider.x += dx;
                            if check_collision(&level, &enemy.entity.collider) {
                                enemy.entity.collider.x -= dx;
                                enemy.facing_right = !enemy.facing_right;
                            }
                            
                            enemy.entity.collider.y += enemy.entity.vy * dt;
                            if check_collision(&level, &enemy.entity.collider) {
                                enemy.entity.collider.y -= enemy.entity.vy * dt;
                                enemy.entity.vy = 0.0;
                            }
                        }
                        EnemyType::SteelBall => {
                            if enemy.entity.vy == 0.0 { enemy.entity.vy = 150.0; } // Initial vertical speed
                            let dx = if enemy.facing_right { 150.0 * dt } else { -150.0 * dt };
                            enemy.entity.collider.x += dx;
                            if check_collision(&level, &enemy.entity.collider) {
                                enemy.entity.collider.x -= dx;
                                enemy.facing_right = !enemy.facing_right;
                            }
                            let dy = enemy.entity.vy * dt;
                            enemy.entity.collider.y += dy;
                            if check_collision(&level, &enemy.entity.collider) {
                                enemy.entity.collider.y -= dy;
                                enemy.entity.vy = -enemy.entity.vy; // Bounce vertically
                            }
                        }
                        EnemyType::Spring => {
                            enemy.entity.vy += 800.0 * dt; // Gravity
                            enemy.entity.collider.y += enemy.entity.vy * dt;
                            if check_collision(&level, &enemy.entity.collider) {
                                enemy.entity.collider.y -= enemy.entity.vy * dt;
                                enemy.entity.vy = -450.0; // Bounce up
                            }
                        }
                    }

                    if player.entity.collider.overlaps(&enemy.entity.collider) && !player.dead {
                        player.dead = true;
                        state = GameState::GameOver;
                        audio.stop_jet();
                        audio.play_game_over();
                    }
                }

                // --- Render ---
                clear_background(Color::new(0.05, 0.05, 0.1, 1.0));
                level.draw();
                for enemy in &enemies {
                    enemy.draw();
                }
                player.draw();

                // --- HUD ---
                set_default_camera();
                draw_rectangle(0.0, SCREEN_HEIGHT - 24.0, SCREEN_WIDTH, 24.0, DARKGRAY);
                draw_text(&format!("FUEL: {}%", player.fuel.round()), 10.0, SCREEN_HEIGHT - 6.0, 20.0, YELLOW);
                draw_text(&format!("EMERALDS: {}/{}", level.emeralds_collected, level.emeralds_total), 200.0, SCREEN_HEIGHT - 6.0, 20.0, GREEN);
                draw_text(&format!("FPS: {}", get_fps()), SCREEN_WIDTH - 100.0, SCREEN_HEIGHT - 6.0, 20.0, WHITE);
            }
            GameState::GameOver | GameState::Victory => {
                let restart = is_key_pressed(KeyCode::R) || is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left);
                if restart {
                    level = create_test_level();
                    player = Player::new(2, 13);
                    enemies = vec![
                        Enemy::new(5, 5, EnemyType::Trackbot),
                        Enemy::new(12, 5, EnemyType::Trackbot),
                        Enemy::new(15, 17, EnemyType::SteelBall),
                        Enemy::new(5, 17, EnemyType::Spring),
                    ];
                    state = GameState::Playing;
                }

                set_default_camera();
                if matches!(state, GameState::Victory) {
                    if let Some(ref tex) = tex_rocket {
                        draw_texture_ex(tex, sw / 2.0 - 50.0, sh / 2.0 - 150.0, WHITE, DrawTextureParams {
                            dest_size: Some(vec2(100.0, 100.0)),
                            ..Default::default()
                        });
                    }
                }
                let msg = if matches!(state, GameState::Victory) { "LEVEL COMPLETE!" } else { "GAME OVER" };
                let color = if matches!(state, GameState::Victory) { GREEN } else { RED };
                let mw = measure_text(msg, None, 60, 1.0).width;
                draw_text(msg, sw / 2.0 - mw / 2.0, sh / 2.0, 60.0, color);
                
                let sub = "TAP OR PRESS R TO RESTART";
                let s_width = measure_text(sub, None, 30, 1.0).width;
                draw_text(sub, sw / 2.0 - s_width / 2.0, sh / 2.0 + 50.0, 30.0, WHITE);
            }
        }
        
        next_frame().await;
    }
}
