use macroquad::prelude::*;
use crate::input::{PlayerInput, InputManager};
use crate::audio::AudioManager;
use crate::gfx::SpriteManager;

pub const VIRTUAL_WIDTH: f32 = 256.0;
pub const VIRTUAL_HEIGHT: f32 = 400.0; // Play area (224) + HUD (16) + Controls (160)
pub const PLAY_HEIGHT: f32 = 224.0;
pub const HUD_HEIGHT: f32 = 16.0;
pub const TILE_SIZE: f32 = 16.0;

// Physics Constants
const GRAVITY: f32 = 0.22;
const JUMP_FORCE: f32 = -5.2;
const ACCEL: f32 = 0.35;
const FRICTION: f32 = 0.75;
const MAX_SPEED: f32 = 2.0;
const TERMINAL_VELOCITY: f32 = 6.0;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction { Left, Right }

#[derive(Clone, Copy, PartialEq)]
pub enum UpgradeType {
    MoreBubbles,
    FasterBubbles,
    LongerDistance,
    DoubleSize,
}

pub struct Player {
    pub pos: Vec2,
    pub vel: Vec2,
    pub dir: Direction,
    pub grounded: bool,
    pub coyote_timer: f32,
    pub jump_buffer: f32,
    pub dead: bool,
    pub score: u32,
    pub lives: u8,
    pub id: usize, // 0 for Bub, 1 for Bob
    pub anim_timer: f32,
    pub blow_timer: f32,
    pub respawn_timer: f32,
    
    // Stats / Upgrades
    pub max_bubbles: usize,
    pub bubble_speed: f32,
    pub bubble_range: f32, // in seconds
    pub bubble_scale: f32,
    pub powerup_timer: f32,
}

impl Player {
    pub fn new(id: usize) -> Self {
        Self {
            pos: vec2(40.0 + (id as f32 * 160.0), 180.0),
            vel: Vec2::ZERO,
            dir: if id == 0 { Direction::Right } else { Direction::Left },
            grounded: false,
            coyote_timer: 0.0,
            jump_buffer: 0.0,
            dead: false,
            score: 0,
            lives: 3,
            id,
            anim_timer: 0.0,
            blow_timer: 0.0,
            respawn_timer: 0.0,
            max_bubbles: 10,
            bubble_speed: 3.5,
            bubble_range: 0.5,
            bubble_scale: 1.0,
            powerup_timer: 0.0,
        }
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.pos.x + 2.0, self.pos.y + 2.0, 12.0, 14.0)
    }
}

pub struct Enemy {
    pub pos: Vec2,
    pub vel: Vec2,
    pub dir: Direction,
    pub trapped: bool,
    pub trap_timer: f32,
    pub dead: bool,
    pub anim_timer: f32,
}

impl Enemy {
    pub fn new(pos: Vec2) -> Self {
        Self {
            pos,
            vel: vec2(0.5, 0.0),
            dir: Direction::Right,
            trapped: false,
            trap_timer: 0.0,
            dead: false,
            anim_timer: 0.0,
        }
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.pos.x + 2.0, self.pos.y + 2.0, 12.0, 12.0)
    }
}

pub struct Bubble {
    pub pos: Vec2,
    pub vel: Vec2,
    pub timer: f32,
    pub range_timer: f32,
    pub trapped_enemy: bool,
    pub owner_id: usize,
    pub scale: f32,
}

pub struct Item {
    pub pos: Vec2,
    pub vel: Vec2,
    pub upgrade: Option<UpgradeType>,
    pub score_val: u32,
    pub timer: f32,
}

pub struct Level {
    pub tiles: Vec<u8>,
}

impl Level {
    pub fn is_wall(&self, x: i32, y: i32) -> bool {
        if x < 0 || x >= 16 || y < 0 || y >= 14 { return false; }
        self.tiles[(y * 16 + x) as usize] == 1
    }
}

pub struct Game {
    pub players: Vec<Player>,
    pub enemies: Vec<Enemy>,
    pub bubbles: Vec<Bubble>,
    pub items: Vec<Item>,
    pub level: Level,
    pub next_level: Option<Level>,
    pub current_level_idx: usize,
    pub transition_timer: f32,
    pub game_over: bool,
}

impl Game {
    pub fn new(two_player: bool) -> Self {
        let mut players = vec![Player::new(0)];
        if two_player {
            players.push(Player::new(1));
        }

        let mut game = Self {
            players,
            enemies: Vec::new(),
            bubbles: Vec::new(),
            items: Vec::new(),
            level: Level { tiles: get_level_layout(0) },
            next_level: None,
            current_level_idx: 0,
            transition_timer: 0.0,
            game_over: false,
        };
        game.spawn_enemies(0);
        game
    }

    fn spawn_enemies(&mut self, idx: usize) {
        self.enemies = match idx % 3 {
            0 => vec![
                Enemy::new(vec2(100.0, 50.0)),
                Enemy::new(vec2(200.0, 50.0)),
                Enemy::new(vec2(50.0, 100.0)),
                Enemy::new(vec2(150.0, 150.0)),
            ],
            1 => vec![
                Enemy::new(vec2(40.0, 40.0)),
                Enemy::new(vec2(200.0, 40.0)),
                Enemy::new(vec2(120.0, 80.0)),
                Enemy::new(vec2(120.0, 140.0)),
            ],
            _ => vec![
                Enemy::new(vec2(80.0, 40.0)),
                Enemy::new(vec2(160.0, 40.0)),
                Enemy::new(vec2(40.0, 80.0)),
                Enemy::new(vec2(200.0, 80.0)),
                Enemy::new(vec2(120.0, 120.0)),
            ],
        };
    }

    fn start_transition(&mut self, next_idx: usize) {
        self.current_level_idx = next_idx;
        self.next_level = Some(Level { tiles: get_level_layout(next_idx) });
        self.transition_timer = 0.001; 
        self.bubbles.clear();
        self.items.clear();
    }

    pub fn update(&mut self, inputs: &[PlayerInput], audio: &AudioManager) {
        if self.game_over { return; }

        if self.transition_timer > 0.0 {
            self.transition_timer += 0.02;
            if self.transition_timer >= 1.0 && self.next_level.is_some() {
                self.level = self.next_level.take().unwrap();
                self.spawn_enemies(self.current_level_idx);
                for p in self.players.iter_mut() {
                    p.pos = vec2(40.0 + (p.id as f32 * 160.0), 180.0);
                    p.vel = Vec2::ZERO;
                    p.dead = false;
                }
            }
            if self.transition_timer >= 2.0 { self.transition_timer = 0.0; }
            return;
        }

        for i in 0..self.players.len() {
            if self.players[i].dead {
                let p = &mut self.players[i];
                p.respawn_timer -= 0.016;
                if p.respawn_timer <= 0.0 && p.lives > 0 {
                    p.dead = false;
                    p.pos = vec2(40.0 + (p.id as f32 * 160.0), 180.0);
                    p.vel = Vec2::ZERO;
                }
                continue;
            }
            
            let input = &inputs[i];
            let p = &mut self.players[i];
            
            if input.left { p.vel.x -= ACCEL; p.dir = Direction::Left; }
            else if input.right { p.vel.x += ACCEL; p.dir = Direction::Right; }
            else { p.vel.x *= FRICTION; }
            p.vel.x = p.vel.x.clamp(-MAX_SPEED, MAX_SPEED);

            if p.grounded { p.coyote_timer = 0.1; }
            else { p.coyote_timer -= 0.016; }

            if input.jump { p.jump_buffer = 0.1; }
            else { p.jump_buffer -= 0.016; }

            if p.jump_buffer > 0.0 && p.coyote_timer > 0.0 {
                p.vel.y = JUMP_FORCE;
                p.grounded = false;
                p.coyote_timer = 0.0;
                p.jump_buffer = 0.0;
                audio.play_jump();
            }
            
            if p.blow_timer > 0.0 { p.blow_timer -= 0.016; }
            if p.vel.x.abs() > 0.1 && p.grounded { p.anim_timer += 0.15; }
            else { p.anim_timer = 0.0; }

            if p.powerup_timer > 0.0 {
                p.powerup_timer -= 0.016;
                if p.powerup_timer <= 0.0 {
                    p.max_bubbles = 10; p.bubble_speed = 3.5; p.bubble_range = 0.5; p.bubble_scale = 1.0;
                }
            }

            if input.bubble {
                let current_bubbles = self.bubbles.iter().filter(|b| b.owner_id == p.id).count();
                if current_bubbles < p.max_bubbles {
                    p.blow_timer = 0.2;
                    let bx = if p.dir == Direction::Right { p.pos.x + 16.0 } else { p.pos.x - 16.0 };
                    self.bubbles.push(Bubble {
                        pos: vec2(bx, p.pos.y),
                        vel: vec2(if p.dir == Direction::Right { p.bubble_speed } else { -p.bubble_speed }, 0.0),
                        timer: 5.0,
                        range_timer: p.bubble_range,
                        trapped_enemy: false,
                        owner_id: p.id,
                        scale: p.bubble_scale,
                    });
                    audio.play_bubble_blow();
                }
            }

            p.vel.y += GRAVITY;
            if p.vel.y > TERMINAL_VELOCITY { p.vel.y = TERMINAL_VELOCITY; }
            p.pos += p.vel;
            
            handle_player_collision(p, &self.level);
            
            if p.pos.x < -16.0 { p.pos.x = VIRTUAL_WIDTH; }
            if p.pos.x > VIRTUAL_WIDTH { p.pos.x = -16.0; }
            let tx = ((p.pos.x + 8.0) / TILE_SIZE) as i32;
            if p.pos.y > PLAY_HEIGHT {
                if !self.level.is_wall(tx, 0) && !self.level.is_wall(tx, 13) { p.pos.y = -15.0; }
                else { p.pos.y = PLAY_HEIGHT; p.vel.y = 0.0; p.grounded = true; }
            }
            if p.pos.y < -16.0 {
                if !self.level.is_wall(tx, 0) && !self.level.is_wall(tx, 13) { p.pos.y = PLAY_HEIGHT - 1.0; }
                else { p.pos.y = -16.0; p.vel.y = 0.0; }
            }
        }

        for b in self.bubbles.iter_mut() {
            b.pos += b.vel;
            if b.range_timer > 0.0 {
                b.range_timer -= 0.016;
                if b.range_timer <= 0.0 { b.vel.x = 0.0; b.vel.y = -0.6; }
            } else {
                b.pos.x += (get_time() as f32 * 5.0 + b.pos.y * 0.1).sin() * 0.5;
            }
            b.timer -= 0.016;
            let tx = ((b.pos.x + 8.0) / TILE_SIZE) as i32;
            if b.pos.y < -16.0 && !self.level.is_wall(tx, 13) { b.pos.y = PLAY_HEIGHT; }
        }
        self.bubbles.retain(|b| b.timer > 0.0);

        for i in 0..self.enemies.len() {
            let e = &mut self.enemies[i];
            e.anim_timer += 0.1;
            if e.trapped {
                e.pos.y -= 0.5;
                e.trap_timer -= 0.016;
                if e.trap_timer <= 0.0 { e.trapped = false; e.vel.x = 1.2; }
                let tx = ((e.pos.x + 8.0) / TILE_SIZE) as i32;
                if e.pos.y < -16.0 && !self.level.is_wall(tx, 13) { e.pos.y = PLAY_HEIGHT; }
            } else {
                e.pos.x += e.vel.x;
                e.vel.y += GRAVITY;
                if e.vel.y > TERMINAL_VELOCITY { e.vel.y = TERMINAL_VELOCITY; }
                e.pos.y += e.vel.y;
                handle_enemy_collision(e, &self.level);
            }
            
            if e.pos.x < -16.0 { e.pos.x = VIRTUAL_WIDTH; }
            if e.pos.x > VIRTUAL_WIDTH { e.pos.x = -16.0; }
            let tx = ((e.pos.x + 8.0) / TILE_SIZE) as i32;
            if e.pos.y > PLAY_HEIGHT {
                if !self.level.is_wall(tx, 0) && !self.level.is_wall(tx, 13) { e.pos.y = -15.0; }
                else { e.pos.y = PLAY_HEIGHT; e.vel.y = 0.0; }
            }
            if e.pos.y < -16.0 {
                if !self.level.is_wall(tx, 0) && !self.level.is_wall(tx, 13) { e.pos.y = PLAY_HEIGHT - 1.0; }
                else { e.pos.y = -16.0; e.vel.y = 0.0; }
            }
        }

        for b in self.bubbles.iter_mut().filter(|b| !b.trapped_enemy) {
            let b_rect = Rect::new(b.pos.x, b.pos.y, 16.0 * b.scale, 16.0 * b.scale);
            for e in self.enemies.iter_mut().filter(|e| !e.trapped) {
                if b_rect.overlaps(&e.rect()) {
                    e.trapped = true; e.trap_timer = 4.0;
                    b.trapped_enemy = true; b.timer = 4.0;
                    audio.play_enemy_trapped();
                    break;
                }
            }
        }

        let mut game_over_now = false;
        for i in 0..self.players.len() {
            let p_rect = self.players[i].rect();
            if self.players[i].dead { continue; }
            
            for b in self.bubbles.iter_mut() {
                if b.trapped_enemy {
                    let b_rect = Rect::new(b.pos.x, b.pos.y, 16.0 * b.scale, 16.0 * b.scale);
                    if p_rect.overlaps(&b_rect) {
                        b.timer = 0.0; audio.play_bubble_pop();
                        self.players[i].score += 500;
                        let ut = match next_rand(10) {
                            0 => Some(UpgradeType::MoreBubbles),
                            1 => Some(UpgradeType::FasterBubbles),
                            2 => Some(UpgradeType::LongerDistance),
                            3 => Some(UpgradeType::DoubleSize),
                            _ => None,
                        };
                        self.items.push(Item { pos: b.pos, vel: vec2(0.0, 0.0), upgrade: ut, score_val: 500, timer: 10.0 });
                        if let Some(e) = self.enemies.iter_mut().find(|e| e.trapped) { e.dead = true; }
                    }
                }
            }

            for e in self.enemies.iter().filter(|e| !e.trapped && !e.dead) {
                if p_rect.overlaps(&e.rect()) {
                    let p = &mut self.players[i];
                    p.dead = true; p.lives = p.lives.saturating_sub(1); p.respawn_timer = 2.0;
                    audio.play_death();
                    if self.players.iter().all(|p| p.dead && p.lives == 0) { game_over_now = true; }
                }
            }
            
            for it in self.items.iter_mut() {
                if p_rect.overlaps(&Rect::new(it.pos.x, it.pos.y, 16.0, 16.0)) {
                    let p = &mut self.players[i];
                    p.score += it.score_val;
                    if let Some(u) = it.upgrade {
                        p.powerup_timer = 10.0;
                        match u {
                            UpgradeType::MoreBubbles => p.max_bubbles = 20,
                            UpgradeType::FasterBubbles => p.bubble_speed = 6.0,
                            UpgradeType::LongerDistance => p.bubble_range = 1.2,
                            UpgradeType::DoubleSize => p.bubble_scale = 2.0,
                        }
                    }
                    it.timer = 0.0; audio.play_fruit_collect();
                }
            }
        }
        if game_over_now { self.game_over = true; }

        self.enemies.retain(|e| !e.dead);
        self.items.retain(|it| it.timer > 0.0);
        for it in self.items.iter_mut() { it.timer -= 0.016; }

        if self.enemies.is_empty() && !self.game_over && self.transition_timer == 0.0 {
            self.start_transition(self.current_level_idx + 1);
        }
    }

    pub fn draw(&self, gfx: &SpriteManager, input: &InputManager, vx: f32, vy: f32, scale: f32) {
        let game_vy = vy + HUD_HEIGHT * scale;
        let (warp_scale, warp_rot) = if self.transition_timer > 0.0 {
            if self.transition_timer < 1.0 { (1.0 - self.transition_timer, self.transition_timer * 5.0) }
            else { (self.transition_timer - 1.0, (2.0 - self.transition_timer) * 5.0) }
        } else { (1.0, 0.0) };

        for y in 0..14 {
            for x in 0..16 {
                if self.level.tiles[y * 16 + x] == 1 {
                    let tx = vx + (x as f32 * 16.0 + 8.0) * scale;
                    let ty = game_vy + (y as f32 * 16.0 + 8.0) * scale;
                    draw_texture_ex(&gfx.tile, tx - 8.0 * scale * warp_scale, ty - 8.0 * scale * warp_scale, WHITE, DrawTextureParams {
                        dest_size: Some(vec2(16.0 * scale * warp_scale, 16.0 * scale * warp_scale)),
                        rotation: warp_rot, ..Default::default()
                    });
                }
            }
        }

        if self.transition_timer > 0.0 { return; }

        for b in &self.bubbles {
            if b.timer < 1.5 && (get_time() * 10.0) as i32 % 2 == 0 { continue; }
            let frame_idx = (get_time() * 4.0) as usize % 2;
            let tex = if b.trapped_enemy { &gfx.zen_chan[frame_idx] } else { &gfx.bubble };
            let s = if b.trapped_enemy { 0.7 } else { 1.0 } * b.scale;
            let offset = (16.0 * (1.0 - s)) / 2.0;
            draw_texture_ex(tex, vx + (b.pos.x + offset) * scale, game_vy + (b.pos.y + offset) * scale, WHITE, DrawTextureParams {
                dest_size: Some(vec2(16.0 * s * scale, 16.0 * s * scale)), ..Default::default()
            });
            if b.trapped_enemy {
                draw_texture_ex(&gfx.bubble, vx + b.pos.x * scale, game_vy + b.pos.y * scale, Color::new(1.0, 1.0, 1.0, 0.5), DrawTextureParams {
                    dest_size: Some(vec2(16.0 * b.scale * scale, 16.0 * b.scale * scale)), ..Default::default()
                });
            }
        }

        for e in &self.enemies {
            if !e.trapped {
                let frame_idx = (e.anim_timer as usize) % 2;
                draw_texture_ex(&gfx.zen_chan[frame_idx], vx + e.pos.x * scale, game_vy + e.pos.y * scale, WHITE, DrawTextureParams {
                    dest_size: Some(vec2(16.0 * scale, 16.0 * scale)), flip_x: e.vel.x < 0.0, ..Default::default()
                });
            }
        }

        for it in &self.items {
            let color = if it.upgrade.is_some() { SKYBLUE } else { WHITE };
            draw_texture_ex(&gfx.apple, vx + it.pos.x * scale, game_vy + it.pos.y * scale, color, DrawTextureParams {
                dest_size: Some(vec2(16.0 * scale, 16.0 * scale)), ..Default::default()
            });
        }

        for p in &self.players {
            if p.dead { continue; }
            let tex = if p.id == 0 {
                if p.blow_timer > 0.0 { &gfx.bub_blow }
                else if p.anim_timer > 0.0 { &gfx.bub_walk[(p.anim_timer as usize) % 2] }
                else { &gfx.bub_idle }
            } else {
                if p.blow_timer > 0.0 { &gfx.bob_blow }
                else if p.anim_timer > 0.0 { &gfx.bob_walk[(p.anim_timer as usize) % 2] }
                else { &gfx.bob_idle }
            };
            draw_texture_ex(tex, vx + p.pos.x * scale, game_vy + p.pos.y * scale, WHITE, DrawTextureParams {
                dest_size: Some(vec2(16.0 * scale, 16.0 * scale)), flip_x: p.dir == Direction::Left, ..Default::default()
            });
        }

        let font_size = (12.0 * scale) as u16;
        draw_text(&format!("P1: {:06} L:{}", self.players[0].score, self.players[0].lives), vx + 5.0 * scale, vy + 12.0 * scale, font_size as f32, GREEN);
        if self.players.len() > 1 {
            draw_text(&format!("P2: {:06} L:{}", self.players[1].score, self.players[1].lives), vx + 170.0 * scale, vy + 12.0 * scale, font_size as f32, BLUE);
        }
        draw_text(&format!("LEVEL {:02}", self.current_level_idx + 1), vx + 105.0 * scale, vy + 12.0 * scale, font_size as f32, YELLOW);

        // Draw Touch Controls
        input.draw_controls(vx, vy, scale, VIRTUAL_WIDTH, VIRTUAL_HEIGHT);
    }
}

fn handle_player_collision(p: &mut Player, level: &Level) {
    let ty = (p.pos.y / TILE_SIZE) as i32;
    let ground_tile_x = (p.pos.x + 8.0) / TILE_SIZE;
    let ground_tile_y = (p.pos.y + 16.0) / TILE_SIZE;
    if level.is_wall(ground_tile_x as i32, ground_tile_y as i32) {
        if p.vel.y > 0.0 { p.pos.y = (ground_tile_y as i32 * 16) as f32 - 16.0; p.vel.y = 0.0; p.grounded = true; }
    } else { p.grounded = false; }
    if level.is_wall((p.pos.x + 4.0) as i32 / 16, ty) || level.is_wall((p.pos.x + 4.0) as i32 / 16, (p.pos.y + 14.0) as i32 / 16) {
        if p.vel.x < 0.0 { p.pos.x = (p.pos.x as i32 / 16 * 16 + 16) as f32; p.vel.x = 0.0; }
    }
    if level.is_wall((p.pos.x + 12.0) as i32 / 16, ty) || level.is_wall((p.pos.x + 12.0) as i32 / 16, (p.pos.y + 14.0) as i32 / 16) {
        if p.vel.x > 0.0 { p.pos.x = (p.pos.x as i32 / 16 * 16) as f32; p.vel.x = 0.0; }
    }
}

fn handle_enemy_collision(e: &mut Enemy, level: &Level) {
    let ground_tile_x = (e.pos.x + 8.0) / TILE_SIZE;
    let ground_tile_y = (e.pos.y + 16.0) / TILE_SIZE;
    if level.is_wall(ground_tile_x as i32, ground_tile_y as i32) {
        if e.vel.y > 0.0 { e.pos.y = (ground_tile_y as i32 * 16) as f32 - 16.0; e.vel.y = 0.0; }
    }
    if level.is_wall((e.pos.x + if e.vel.x > 0.0 { 16.0 } else { 0.0 }) as i32 / 16, (e.pos.y + 8.0) as i32 / 16) { e.vel.x = -e.vel.x; }
}

fn next_rand(max: usize) -> usize {
    static mut SEED: u32 = 42;
    unsafe { SEED = SEED.wrapping_mul(1103515245).wrapping_add(12345); ((SEED >> 16) as usize) % max }
}

fn get_level_layout(idx: usize) -> Vec<u8> {
    let mut lvl = vec![0u8; 16 * 14];
    for x in 0..16 { lvl[x] = 1; lvl[13 * 16 + x] = 1; }
    for y in 0..14 { lvl[y * 16] = 1; lvl[y * 16 + 15] = 1; }
    match idx % 3 {
        0 => {
            lvl[7] = 0; lvl[8] = 0; lvl[13 * 16 + 7] = 0; lvl[13 * 16 + 8] = 0;
            let platforms = [(2, 4, 12), (2, 7, 5), (9, 7, 5), (2, 10, 12)];
            for (px, py, pw) in platforms { for x in 0..pw { lvl[py * 16 + px + x] = 1; } }
        }
        1 => {
            lvl[4] = 0; lvl[11] = 0; lvl[13 * 16 + 4] = 0; lvl[13 * 16 + 11] = 0;
            let platforms = [(2, 4, 4), (10, 4, 4), (5, 7, 6), (2, 10, 4), (10, 10, 4)];
            for (px, py, pw) in platforms { for x in 0..pw { lvl[py * 16 + px + x] = 1; } }
        }
        _ => {
            lvl[2] = 0; lvl[13] = 0; lvl[13 * 16 + 2] = 0; lvl[13 * 16 + 13] = 0;
            let platforms = [(2, 3, 3), (11, 3, 3), (6, 6, 4), (2, 9, 3), (11, 9, 3), (5, 11, 6)];
            for (px, py, pw) in platforms { for x in 0..pw { lvl[py * 16 + px + x] = 1; } }
        }
    }
    lvl
}
