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

    let mut game_over = false;
    let mut victory = false;

    loop {
        if victory {
            clear_background(BLACK);
            draw_text("ROCKET LAUNCHED!", SCREEN_WIDTH / 2.0 - 150.0, SCREEN_HEIGHT / 2.0, 40.0, GREEN);
            draw_text("Press R to play again", SCREEN_WIDTH / 2.0 - 100.0, SCREEN_HEIGHT / 2.0 + 40.0, 20.0, WHITE);
            if is_key_pressed(KeyCode::R) {
                player = Player::new();
                lasers.clear();
                enemies = vec![
                    Enemy::new(0.0, 100.0, 200.0),
                    Enemy::new(SCREEN_WIDTH, 300.0, -150.0),
                ];
                rocket = Rocket::new(600.0, SCREEN_HEIGHT - 20.0);
                parts = vec![
                    RocketPart::new(100.0, 50.0, PartType::Middle),
                    RocketPart::new(500.0, 50.0, PartType::Top),
                ];
                game_over = false;
                victory = false;
            }
            next_frame().await;
            continue;
        }

        if game_over {
            clear_background(BLACK);
            draw_text("GAME OVER", SCREEN_WIDTH / 2.0 - 80.0, SCREEN_HEIGHT / 2.0, 40.0, RED);
            draw_text("Press R to restart", SCREEN_WIDTH / 2.0 - 90.0, SCREEN_HEIGHT / 2.0 + 40.0, 20.0, WHITE);
            if is_key_pressed(KeyCode::R) {
                player = Player::new();
                lasers.clear();
                enemies = vec![
                    Enemy::new(0.0, 100.0, 200.0),
                    Enemy::new(SCREEN_WIDTH, 300.0, -150.0),
                ];
                rocket = Rocket::new(600.0, SCREEN_HEIGHT - 20.0);
                parts = vec![
                    RocketPart::new(100.0, 50.0, PartType::Middle),
                    RocketPart::new(500.0, 50.0, PartType::Top),
                ];
                game_over = false;
            }
            next_frame().await;
            continue;
        }

        let dt = get_frame_time();

        // Update
        if player.update(dt, &mut lasers) {
            audio.play_laser();
        }
        if player.is_jetting {
            audio.play_jet();
        }

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
                game_over = true;
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

        // Draw
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
                victory = true;
                audio.play_win();
            }
        }

        draw_text(&format!("FPS: {}", get_fps()), 10.0, 20.0, 20.0, WHITE);
        
        next_frame().await;
    }
}
