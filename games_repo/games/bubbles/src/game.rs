use macroquad::prelude::*;
use crate::input::PlayerInput;
use crate::audio::AudioManager;
use crate::gfx::SpriteManager;

pub const VIRTUAL_WIDTH: f32 = 256.0;
pub const VIRTUAL_HEIGHT: f32 = 224.0;
pub const TILE_SIZE: f32 = 16.0;

#[derive(Clone, Copy, PartialEq)]
pub enum Direction { Left, Right }

pub struct Player {
    pub pos: Vec2,
    pub vel: Vec2,
    pub dir: Direction,
    pub grounded: bool,
    #[allow(dead_code)]
    pub jump_timer: f32,
    pub dead: bool,
    pub score: u32,
    pub id: usize, // 0 for Bub, 1 for Bob
}

impl Player {
    pub fn new(id: usize) -> Self {
        Self {
            pos: vec2(40.0 + (id as f32 * 160.0), 180.0),
            vel: Vec2::ZERO,
            dir: if id == 0 { Direction::Right } else { Direction::Left },
            grounded: false,
            jump_timer: 0.0,
            dead: false,
            score: 0,
            id,
        }
    }

    pub fn rect(&self) -> Rect {
        Rect::new(self.pos.x + 2.0, self.pos.y + 2.0, 12.0, 14.0)
    }
}

pub struct Enemy {
    pub pos: Vec2,
    pub vel: Vec2,
    #[allow(dead_code)]
    pub dir: Direction,
    pub trapped: bool,
    pub trap_timer: f32,
    pub dead: bool,
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
    pub trapped_enemy: bool,
}

pub struct Fruit {
    pub pos: Vec2,
    #[allow(dead_code)]
    pub vel: Vec2,
    pub score_val: u32,
    pub timer: f32,
}

pub struct Game {
    pub players: Vec<Player>,
    pub enemies: Vec<Enemy>,
    pub bubbles: Vec<Bubble>,
    pub fruits: Vec<Fruit>,
    pub level: Vec<u8>, // 0: empty, 1: wall
    pub game_over: bool,
}

fn is_wall(level: &[u8], x: i32, y: i32) -> bool {
    if x < 0 || x >= 16 || y < 0 || y >= 14 { return false; }
    level[(y * 16 + x) as usize] == 1
}

impl Game {
    pub fn new(two_player: bool) -> Self {
        let mut players = vec![Player::new(0)];
        if two_player {
            players.push(Player::new(1));
        }

        let level = generate_level();
        let enemies = vec![
            Enemy::new(vec2(100.0, 50.0)),
            Enemy::new(vec2(200.0, 50.0)),
            Enemy::new(vec2(50.0, 100.0)),
            Enemy::new(vec2(150.0, 150.0)),
        ];

        Self {
            players,
            enemies,
            bubbles: Vec::new(),
            fruits: Vec::new(),
            level,
            game_over: false,
        }
    }

    pub fn update(&mut self, inputs: &[PlayerInput], audio: &AudioManager) {
        if self.game_over { return; }

        for i in 0..self.players.len() {
            if self.players[i].dead { continue; }
            let input = &inputs[i];
            
            {
                let p = &mut self.players[i];
                // Horizontal movement
                if input.left {
                    p.vel.x = -1.5;
                    p.dir = Direction::Left;
                } else if input.right {
                    p.vel.x = 1.5;
                    p.dir = Direction::Right;
                } else {
                    p.vel.x *= 0.8;
                }

                // Jumping
                if input.jump && p.grounded {
                    p.vel.y = -4.0;
                    p.grounded = false;
                    audio.play_jump();
                }
            }

            // Blowing bubbles
            if input.bubble {
                let p = &self.players[i];
                let bx = if p.dir == Direction::Right { p.pos.x + 16.0 } else { p.pos.x - 16.0 };
                self.bubbles.push(Bubble {
                    pos: vec2(bx, p.pos.y),
                    vel: vec2(if p.dir == Direction::Right { 3.0 } else { -3.0 }, 0.0),
                    timer: 5.0,
                    trapped_enemy: false,
                });
                audio.play_bubble_blow();
            }

            {
                let p = &mut self.players[i];
                // Apply gravity
                p.vel.y += 0.2;

                // Physics and screen wrap
                p.pos += p.vel;
            }
            self.handle_player_collision(i);
            
            {
                let p = &mut self.players[i];
                if p.pos.x < -16.0 { p.pos.x = VIRTUAL_WIDTH; }
                if p.pos.x > VIRTUAL_WIDTH { p.pos.x = -16.0; }
                if p.pos.y > VIRTUAL_HEIGHT { p.pos.y = -16.0; }
                if p.pos.y < -16.0 { p.pos.y = VIRTUAL_HEIGHT; }
            }
        }

        // Update Bubbles
        for b in self.bubbles.iter_mut() {
            b.pos += b.vel;
            b.vel.x *= 0.9;
            if b.vel.x.abs() < 0.1 {
                b.vel.x = 0.0;
                b.vel.y = -0.5; // Float up
            }
            b.timer -= 0.016;
        }
        self.bubbles.retain(|b| b.timer > 0.0);

        // Update Enemies
        for i in 0..self.enemies.len() {
            {
                let e = &mut self.enemies[i];
                if e.trapped {
                    e.pos.y -= 0.5;
                    e.trap_timer -= 0.016;
                    if e.trap_timer <= 0.0 {
                        e.trapped = false;
                        e.vel.x = 1.0; // Angry
                    }
                } else {
                    e.pos.x += e.vel.x;
                    e.vel.y += 0.2;
                    e.pos.y += e.vel.y;
                }
            }
            if !self.enemies[i].trapped {
                self.handle_enemy_collision(i);
            }
            
            {
                let e = &mut self.enemies[i];
                if e.pos.x < -16.0 { e.pos.x = VIRTUAL_WIDTH; }
                if e.pos.x > VIRTUAL_WIDTH { e.pos.x = -16.0; }
                if e.pos.y > VIRTUAL_HEIGHT { e.pos.y = -16.0; }
                if e.pos.y < -16.0 { e.pos.y = VIRTUAL_HEIGHT; }
            }
        }

        // Collision: Bubbles vs Enemies
        for b in self.bubbles.iter_mut().filter(|b| !b.trapped_enemy) {
            let b_rect = Rect::new(b.pos.x, b.pos.y, 16.0, 16.0);
            for e in self.enemies.iter_mut().filter(|e| !e.trapped) {
                if b_rect.overlaps(&e.rect()) {
                    e.trapped = true;
                    e.trap_timer = 4.0;
                    b.trapped_enemy = true;
                    b.timer = 4.0;
                    audio.play_enemy_trapped();
                    break;
                }
            }
        }

        // Collision: Players vs Bubbles/Enemies
        let mut game_over_now = false;
        for i in 0..self.players.len() {
            if self.players[i].dead { continue; }
            let p_rect = self.players[i].rect();
            
            // Pop trapped bubbles
            for b in self.bubbles.iter_mut() {
                if b.trapped_enemy {
                    let b_rect = Rect::new(b.pos.x, b.pos.y, 16.0, 16.0);
                    if p_rect.overlaps(&b_rect) {
                        b.timer = 0.0; // Mark for removal
                        audio.play_bubble_pop();
                        // Turn trapped enemy into fruit
                        self.fruits.push(Fruit {
                            pos: b.pos,
                            vel: vec2(0.0, 0.0),
                            score_val: 500,
                            timer: 10.0,
                        });
                        // Find and remove the actual enemy (this is simplified)
                        if let Some(e) = self.enemies.iter_mut().find(|e| e.trapped) {
                            e.dead = true;
                        }
                    }
                }
            }

            // Player vs Enemies
            for e in self.enemies.iter_mut().filter(|e| !e.trapped && !e.dead) {
                if p_rect.overlaps(&e.rect()) {
                    self.players[i].dead = true;
                    audio.play_death();
                    if self.players.iter().all(|p| p.dead) {
                        game_over_now = true;
                    }
                }
            }
            
            // Player vs Fruits
            for f in self.fruits.iter_mut() {
                let f_rect = Rect::new(f.pos.x, f.pos.y, 16.0, 16.0);
                if p_rect.overlaps(&f_rect) {
                    self.players[i].score += f.score_val;
                    f.timer = 0.0;
                    audio.play_fruit_collect();
                }
            }
        }
        if game_over_now { self.game_over = true; }

        self.enemies.retain(|e| !e.dead);
        self.fruits.retain(|f| f.timer > 0.0);
        for f in self.fruits.iter_mut() { f.timer -= 0.016; }

        if self.enemies.is_empty() && !self.game_over {
            self.enemies = vec![
                Enemy::new(vec2(100.0, 50.0)),
                Enemy::new(vec2(200.0, 50.0)),
            ];
        }
    }

    fn handle_player_collision(&mut self, player_idx: usize) {
        let p = &mut self.players[player_idx];
        let ty = (p.pos.y / TILE_SIZE) as i32;
        
        // Ground collision
        let ground_tile_x = (p.pos.x + 8.0) / TILE_SIZE;
        let ground_tile_y = (p.pos.y + 16.0) / TILE_SIZE;
        if is_wall(&self.level, ground_tile_x as i32, ground_tile_y as i32) {
            if p.vel.y > 0.0 {
                p.pos.y = (ground_tile_y as i32 * 16) as f32 - 16.0;
                p.vel.y = 0.0;
                p.grounded = true;
            }
        } else {
            p.grounded = false;
        }
        
        // Wall collisions
        if is_wall(&self.level, (p.pos.x + 4.0) as i32 / 16, ty) || is_wall(&self.level, (p.pos.x + 4.0) as i32 / 16, (p.pos.y + 14.0) as i32 / 16) {
            if p.vel.x < 0.0 { p.pos.x = (p.pos.x as i32 / 16 * 16 + 16) as f32; p.vel.x = 0.0; }
        }
        if is_wall(&self.level, (p.pos.x + 12.0) as i32 / 16, ty) || is_wall(&self.level, (p.pos.x + 12.0) as i32 / 16, (p.pos.y + 14.0) as i32 / 16) {
            if p.vel.x > 0.0 { p.pos.x = (p.pos.x as i32 / 16 * 16) as f32; p.vel.x = 0.0; }
        }
    }

    fn handle_enemy_collision(&mut self, enemy_idx: usize) {
        let e = &mut self.enemies[enemy_idx];
        let ground_tile_x = (e.pos.x + 8.0) / TILE_SIZE;
        let ground_tile_y = (e.pos.y + 16.0) / TILE_SIZE;
        if is_wall(&self.level, ground_tile_x as i32, ground_tile_y as i32) {
            if e.vel.y > 0.0 {
                e.pos.y = (ground_tile_y as i32 * 16) as f32 - 16.0;
                e.vel.y = 0.0;
            }
        }
        
        // Bounce off walls
        if is_wall(&self.level, (e.pos.x + if e.vel.x > 0.0 { 16.0 } else { 0.0 }) as i32 / 16, (e.pos.y + 8.0) as i32 / 16) {
            e.vel.x = -e.vel.x;
        }
    }

    pub fn draw(&self, gfx: &SpriteManager, vx: f32, vy: f32, scale: f32) {
        // Draw Level
        for y in 0..14 {
            for x in 0..16 {
                if self.level[y * 16 + x] == 1 {
                    draw_texture_ex(&gfx.tile, vx + x as f32 * 16.0 * scale, vy + y as f32 * 16.0 * scale, WHITE, DrawTextureParams {
                        dest_size: Some(vec2(16.0 * scale, 16.0 * scale)),
                        ..Default::default()
                    });
                }
            }
        }

        // Draw Bubbles
        for b in &self.bubbles {
            let tex = if b.trapped_enemy { &gfx.zen_chan } else { &gfx.bubble };
            draw_texture_ex(tex, vx + b.pos.x * scale, vy + b.pos.y * scale, WHITE, DrawTextureParams {
                dest_size: Some(vec2(16.0 * scale, 16.0 * scale)),
                ..Default::default()
            });
            if b.trapped_enemy {
                draw_texture_ex(&gfx.bubble, vx + b.pos.x * scale, vy + b.pos.y * scale, Color::new(1.0, 1.0, 1.0, 0.5), DrawTextureParams {
                    dest_size: Some(vec2(16.0 * scale, 16.0 * scale)),
                    ..Default::default()
                });
            }
        }

        // Draw Enemies
        for e in &self.enemies {
            if !e.trapped {
                draw_texture_ex(&gfx.zen_chan, vx + e.pos.x * scale, vy + e.pos.y * scale, WHITE, DrawTextureParams {
                    dest_size: Some(vec2(16.0 * scale, 16.0 * scale)),
                    ..Default::default()
                });
            }
        }

        // Draw Fruits
        for f in &self.fruits {
            draw_texture_ex(&gfx.apple, vx + f.pos.x * scale, vy + f.pos.y * scale, WHITE, DrawTextureParams {
                dest_size: Some(vec2(16.0 * scale, 16.0 * scale)),
                ..Default::default()
            });
        }

        // Draw Players
        for p in &self.players {
            if !p.dead {
                let tex = if p.id == 0 { &gfx.bub } else { &gfx.bob };
                draw_texture_ex(tex, vx + p.pos.x * scale, vy + p.pos.y * scale, WHITE, DrawTextureParams {
                    dest_size: Some(vec2(16.0 * scale, 16.0 * scale)),
                    flip_x: p.dir == Direction::Left,
                    ..Default::default()
                });
            }
        }

        // UI
        let font_size = (20.0 * scale) as u16;
        draw_text(&format!("P1: {:06}", self.players[0].score), vx + 10.0 * scale, vy + 15.0 * scale, font_size as f32, GREEN);
        if self.players.len() > 1 {
            draw_text(&format!("P2: {:06}", self.players[1].score), vx + 160.0 * scale, vy + 15.0 * scale, font_size as f32, BLUE);
        }
    }
}

fn generate_level() -> Vec<u8> {
    let mut lvl = vec![0u8; 16 * 14];
    // Borders
    for x in 0..16 { lvl[x] = 1; lvl[13 * 16 + x] = 1; }
    for y in 0..14 { lvl[y * 16] = 1; lvl[y * 16 + 15] = 1; }
    
    // Platforms
    let platforms = [
        (2, 4, 12), (2, 7, 5), (9, 7, 5), (2, 10, 12)
    ];
    for (px, py, pw) in platforms {
        for x in 0..pw {
            lvl[py * 16 + px + x] = 1;
        }
    }
    lvl
}
