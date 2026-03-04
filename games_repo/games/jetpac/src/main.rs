mod game;
mod physics;
mod audio;

use macroquad::prelude::*;
use crate::game::*;
use crate::audio::AudioManager;

#[derive(PartialEq, Clone)]
enum AppState {
    Menu,
    Playing,
    GameOver { win: bool },
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Jetpac Clone".to_owned(),
        window_width: SCREEN_WIDTH as i32,
        window_height: SCREEN_HEIGHT as i32,
        high_dpi: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut state = AppState::Menu;
    let mut level = create_test_level();
    let mut player = Player::new(2, 16);
    let mut enemies = vec![
        Enemy::new(5, 5, EnemyType::Trackbot),
        Enemy::new(15, 10, EnemyType::SteelBall),
        Enemy::new(10, 13, EnemyType::Spring),
    ];
    
    let audio = AudioManager::new().await;
    audio.play_music();

    loop {
        let sw = screen_width();
        let sh = screen_height();

        match state {
            AppState::Menu => {
                clear_background(BLACK);
                let title = "JETPAC CLONE";
                let t_dims = measure_text(title, None, 60, 1.0);
                draw_text(title, sw / 2.0 - t_dims.width / 2.0, sh * 0.2, 60.0, GREEN);

                let instructions = [
                    "ARROWS: MOVE & THRUST",
                    "Z: PHASE THROUGH BRICKS (COOLDOWN)",
                    "GOAL: COLLECT ALL EMERALDS",
                    "THEN REACH THE EXIT DOOR",
                    "WATCH YOUR FUEL!",
                ];

                for (i, line) in instructions.iter().enumerate() {
                    let dims = measure_text(line, None, 20, 1.0);
                    draw_text(line, sw / 2.0 - dims.width / 2.0, sh * 0.4 + i as f32 * 40.0, 20.0, WHITE);
                }

                let msg = "PRESS SPACE TO START";
                let m_dims = measure_text(msg, None, 30, 1.0);
                draw_text(msg, sw / 2.0 - m_dims.width / 2.0, sh * 0.8, 30.0, YELLOW);

                if is_key_pressed(KeyCode::Space) {
                    state = AppState::Playing;
                    level = create_test_level();
                    player = Player::new(2, 16);
                    enemies = vec![
                        Enemy::new(5, 5, EnemyType::Trackbot),
                        Enemy::new(15, 10, EnemyType::SteelBall),
                        Enemy::new(10, 13, EnemyType::Spring),
                    ];
                }
            }
            AppState::Playing => {
                let dt = get_frame_time();

                // Input
                let mut dx = 0.0;
                if is_key_down(KeyCode::Left) { dx -= 1.0; player.facing_right = false; }
                if is_key_down(KeyCode::Right) { dx += 1.0; player.facing_right = true; }
                player.entity.collider.x += dx * 4.0;

                if is_key_down(KeyCode::Up) && player.fuel > 0.0 {
                    player.entity.vy -= 0.5;
                    player.is_jetting = true;
                    player.fuel -= 0.1;
                } else {
                    player.is_jetting = false;
                }

                if is_key_pressed(KeyCode::Z) && player.phase_cooldown <= 0.0 {
                    // Try to phase bricks player is touching
                    let p_rect = player.entity.collider;
                    let mut phased_any = false;
                    for r in 0..ROWS {
                        for c in 0..COLS {
                            if level.grid[r][c] == TileType::NormalBrick {
                                let b_rect = Rect::new(c as f32 * TILE_SIZE, r as f32 * TILE_SIZE + HUD_HEIGHT, TILE_SIZE, TILE_SIZE);
                                if p_rect.overlaps(&b_rect) {
                                    level.phased_bricks.push(PhasedBrick { col: c, row: r, timer: 0.0 });
                                    level.grid[r][c] = TileType::Empty;
                                    phased_any = true;
                                }
                            }
                        }
                    }
                    if phased_any {
                        player.phase_cooldown = 2.0;
                        audio.play_phase();
                    }
                }
                player.phase_cooldown = (player.phase_cooldown - dt).max(0.0);

                // Gravity & Movement
                player.entity.vy += 0.2;
                player.entity.collider.y += player.entity.vy;

                // Collisions (Player)
                handle_collisions(&mut player.entity, &level);

                // Emeralds & Fuel
                for col in &mut level.collectibles {
                    if col.active && player.entity.collider.overlaps(&Rect::new(col.col as f32 * TILE_SIZE, col.row as f32 * TILE_SIZE + HUD_HEIGHT, TILE_SIZE, TILE_SIZE)) {
                        col.active = false;
                        match col.ctype {
                            CollectibleType::Emerald => {
                                level.emeralds_collected += 1;
                                audio.play_collect();
                                if level.emeralds_collected >= level.emeralds_total {
                                    level.exit_door.active = true;
                                }
                            }
                            CollectibleType::Fuel => {
                                player.fuel = (player.fuel + 30.0).min(100.0);
                                audio.play_fuel();
                            }
                        }
                    }
                }

                // Special tiles
                let px = (player.entity.collider.x + player.entity.collider.w / 2.0) / TILE_SIZE;
                let py = (player.entity.collider.y + player.entity.collider.h - 4.0 - HUD_HEIGHT) / TILE_SIZE;
                let pr = py as usize;
                let pc = px as usize;
                if pr < ROWS && pc < COLS {
                    match level.grid[pr][pc] {
                        TileType::EnergyCharger => { player.fuel = (player.fuel + 0.5).min(100.0); }
                        TileType::EnergyDrain => { player.fuel = (player.fuel - 0.5).max(0.0); }
                        TileType::Spikes => { player.dead = true; }
                        _ => {}
                    }
                }

                // Exit Door
                if level.exit_door.active {
                    if level.exit_door.opening_progress < 1.0 {
                        level.exit_door.opening_progress += 0.01;
                    }
                    if level.exit_door.opening_progress >= 1.0 {
                        let door_rect = Rect::new(level.exit_door.col as f32 * TILE_SIZE, level.exit_door.row as f32 * TILE_SIZE + HUD_HEIGHT, TILE_SIZE * 2.0, TILE_SIZE * 2.0);
                        if player.entity.collider.overlaps(&door_rect) {
                            state = AppState::GameOver { win: true };
                        }
                    }
                }

                // Phased bricks timer
                level.phased_bricks.retain_mut(|pb| {
                    pb.timer += dt;
                    if pb.timer >= 3.0 {
                        level.grid[pb.row][pb.col] = TileType::NormalBrick;
                        false
                    } else {
                        true
                    }
                });

                // Enemies
                for enemy in &mut enemies {
                    enemy.entity.collider.x += if enemy.facing_right { 2.0 } else { -2.0 };
                    
                    // Bounce off walls
                    let ec = (enemy.entity.collider.x + if enemy.facing_right { 24.0 } else { 0.0 }) / TILE_SIZE;
                    let er = (enemy.entity.collider.y + 12.0 - HUD_HEIGHT) / TILE_SIZE;
                    if ec < 0.0 || ec >= COLS as f32 || level.grid[er as usize][ec as usize] != TileType::Empty {
                        enemy.facing_right = !enemy.facing_right;
                    }

                    if enemy.entity.collider.overlaps(&player.entity.collider) {
                        player.dead = true;
                    }
                }

                if player.dead {
                    state = AppState::GameOver { win: false };
                    audio.play_death();
                }

                // Drawing
                clear_background(BLACK);
                level.draw();
                player.draw();
                for enemy in &enemies {
                    enemy.draw();
                }

                // HUD - Now at the Top
                draw_rectangle(0.0, 0.0, SCREEN_WIDTH, HUD_HEIGHT, Color::new(0.1, 0.1, 0.1, 1.0));
                draw_line(0.0, HUD_HEIGHT, SCREEN_WIDTH, HUD_HEIGHT, 2.0, DARKGRAY);
                
                draw_text(&format!("FUEL: {}%", player.fuel.round()), 20.0, 40.0, 25.0, YELLOW);
                draw_text(&format!("EMERALDS: {}/{}", level.emeralds_collected, level.emeralds_total), 250.0, 40.0, 25.0, GREEN);
                draw_text(&format!("FPS: {}", get_fps()), SCREEN_WIDTH - 120.0, 40.0, 20.0, WHITE);
            }
            AppState::GameOver { win } => {
                clear_background(BLACK);
                let (msg, color) = if win { ("MISSION SUCCESS!", GREEN) } else { ("MISSION FAILED!", RED) };
                let m_dims = measure_text(msg, None, 60, 1.0);
                draw_text(msg, sw / 2.0 - m_dims.width / 2.0, sh / 2.0, 60.0, color);

                let sub = "PRESS SPACE TO RETURN TO MENU";
                let s_dims = measure_text(sub, None, 30, 1.0);
                draw_text(sub, sw / 2.0 - s_dims.width / 2.0, sh / 2.0 + 50.0, 30.0, WHITE);

                if is_key_pressed(KeyCode::Space) {
                    state = AppState::Menu;
                }
            }
        }

        next_frame().await
    }
}

fn handle_collisions(entity: &mut physics::RectCollider, level: &Level) {
    // Basic tile collision
    let left_col = (entity.x / TILE_SIZE) as usize;
    let right_col = ((entity.x + entity.w) / TILE_SIZE) as usize;
    let top_row = ((entity.y - HUD_HEIGHT) / TILE_SIZE) as usize;
    let bottom_row = ((entity.y + entity.h - HUD_HEIGHT) / TILE_SIZE) as usize;

    // Floor
    if bottom_row < ROWS {
        for c in left_col..=right_col {
            if c < COLS && (level.grid[bottom_row][c] == TileType::NormalBrick || level.grid[bottom_row][c] == TileType::SolidBrick) {
                let tile_y = bottom_row as f32 * TILE_SIZE + HUD_HEIGHT;
                if entity.y + entity.h > tile_y {
                    entity.y = tile_y - entity.h;
                    // Reset vy is handled in physics but we don't have direct access to Entity here
                }
            }
        }
    }
    
    // Ceiling
    if top_row < ROWS {
        for c in left_col..=right_col {
            if c < COLS && (level.grid[top_row][c] == TileType::NormalBrick || level.grid[top_row][c] == TileType::SolidBrick) {
                let tile_bottom = (top_row + 1) as f32 * TILE_SIZE + HUD_HEIGHT;
                if entity.y < tile_bottom {
                    entity.y = tile_bottom;
                }
            }
        }
    }

    // Walls
    let row_mid = ((entity.y + entity.h / 2.0 - HUD_HEIGHT) / TILE_SIZE) as usize;
    if row_mid < ROWS {
        if left_col < COLS && (level.grid[row_mid][left_col] == TileType::NormalBrick || level.grid[row_mid][left_col] == TileType::SolidBrick) {
            let tile_right = (left_col + 1) as f32 * TILE_SIZE;
            if entity.x < tile_right { entity.x = tile_right; }
        }
        if right_col < COLS && (level.grid[row_mid][right_col] == TileType::NormalBrick || level.grid[row_mid][right_col] == TileType::SolidBrick) {
            let tile_left = right_col as f32 * TILE_SIZE;
            if entity.x + entity.w > tile_left { entity.x = tile_left - entity.w; }
        }
    }
}
