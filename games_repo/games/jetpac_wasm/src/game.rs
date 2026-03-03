use macroquad::prelude::*;
use crate::physics::{Entity, update_physics, wrap_around};

pub const SCREEN_WIDTH: f32 = 800.0;
pub const SCREEN_HEIGHT: f32 = 600.0;

pub struct Player {
    pub entity: Entity,
    pub is_jetting: bool,
    pub facing_right: bool,
    pub shoot_cooldown: f32,
    pub holding_part: Option<PartType>,
}

use macroquad::color::SKYBLUE;

impl Player {
    pub fn new() -> Self {
        Self {
            entity: Entity::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT - 50.0, 32.0, 48.0),
            is_jetting: false,
            facing_right: true,
            shoot_cooldown: 0.0,
            holding_part: None,
        }
    }

    pub fn update(&mut self, dt: f32, lasers: &mut Vec<Laser>) -> bool {
        let mut fired = false;
        self.is_jetting = is_key_down(KeyCode::Up);
        
        if self.is_jetting {
            const THRUST: f32 = 800.0;
            self.entity.vy -= THRUST * dt;
        }

        if is_key_down(KeyCode::Left) {
            self.entity.vx = -300.0;
            self.facing_right = false;
        } else if is_key_down(KeyCode::Right) {
            self.entity.vx = 300.0;
            self.facing_right = true;
        } else {
            self.entity.vx = 0.0;
        }

        if self.shoot_cooldown > 0.0 {
            self.shoot_cooldown -= dt;
        }

        if is_key_down(KeyCode::Space) && self.shoot_cooldown <= 0.0 {
            let laser_x = if self.facing_right { self.entity.x + self.entity.width } else { self.entity.x - 20.0 };
            let laser_vx = if self.facing_right { 600.0 } else { -600.0 };
            lasers.push(Laser::new(laser_x, self.entity.y + 20.0, laser_vx));
            self.shoot_cooldown = 0.2;
            fired = true;
        }

        update_physics(&mut self.entity, dt);
        wrap_around(&mut self.entity, SCREEN_WIDTH);

        // Ground collision (simple)
        if self.entity.y > SCREEN_HEIGHT - self.entity.height {
            self.entity.y = SCREEN_HEIGHT - self.entity.height;
            self.entity.vy = 0.0;
        }
        fired
    }

    pub fn draw(&self) {
        let color = if self.is_jetting { YELLOW } else { RED };
        draw_rectangle(self.entity.x, self.entity.y, self.entity.width, self.entity.height, color);
        
        // Draw "eye" to show direction
        let eye_x = if self.facing_right { self.entity.x + 20.0 } else { self.entity.x + 4.0 };
        draw_rectangle(eye_x, self.entity.y + 10.0, 8.0, 8.0, WHITE);

        // Draw held part
        if let Some(part_type) = self.holding_part {
            let part_color = match part_type {
                PartType::Base => DARKGRAY,
                PartType::Middle => GRAY,
                PartType::Top => LIGHTGRAY,
            };
            draw_rectangle(self.entity.x - 4.0, self.entity.y + self.entity.height, 40.0, 10.0, part_color);
        }
    }
}

pub struct Laser {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub lifetime: f32,
}

impl Laser {
    pub fn new(x: f32, y: f32, vx: f32) -> Self {
        Self { x, y, vx, lifetime: 1.0 }
    }

    pub fn update(&mut self, dt: f32) {
        self.x += self.vx * dt;
        self.lifetime -= dt;
    }

    pub fn draw(&self) {
        draw_line(self.x, self.y, self.x + 10.0 * self.vx.signum(), self.y, 2.0, SKYBLUE);
    }
}

pub struct Platform {
    pub x: f32,
    pub y: f32,
    pub width: f32,
}

impl Platform {
    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, self.width, 10.0, GREEN);
    }

    pub fn check_collision(&self, entity: &mut Entity) {
        if entity.vy > 0.0 && 
           entity.y + entity.height > self.y && 
           entity.y + entity.height < self.y + 20.0 &&
           entity.x + entity.width > self.x && 
           entity.x < self.x + self.width {
            entity.y = self.y - entity.height;
            entity.vy = 0.0;
        }
    }
}

pub struct Enemy {
    pub entity: Entity,
    pub speed: f32,
}

impl Enemy {
    pub fn new(x: f32, y: f32, speed: f32) -> Self {
        Self {
            entity: Entity::new(x, y, 24.0, 24.0),
            speed,
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.entity.vx = self.speed;
        update_physics(&mut self.entity, dt);
        self.entity.vy = 0.0; 
        wrap_around(&mut self.entity, SCREEN_WIDTH);
    }

    pub fn draw(&self) {
        draw_circle(self.entity.x + 12.0, self.entity.y + 12.0, 12.0, PURPLE);
    }
}

pub struct Item {
    pub entity: Entity,
    pub collected: bool,
}

impl Item {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            entity: Entity::new(x, y, 20.0, 20.0),
            collected: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        update_physics(&mut self.entity, dt);
    }

    pub fn draw(&self) {
        if !self.collected {
            draw_rectangle(self.entity.x, self.entity.y, 20.0, 20.0, BLUE);
        }
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum PartType {
    Base,
    Middle,
    Top,
}

pub struct RocketPart {
    pub entity: Entity,
    pub part_type: PartType,
    pub is_held: bool,
    pub is_attached: bool,
}

impl RocketPart {
    pub fn new(x: f32, y: f32, part_type: PartType) -> Self {
        Self {
            entity: Entity::new(x, y, 40.0, 20.0),
            part_type,
            is_held: false,
            is_attached: false,
        }
    }

    pub fn update(&mut self, dt: f32) {
        if !self.is_held && !self.is_attached {
            update_physics(&mut self.entity, dt);
        }
    }

    pub fn draw(&self) {
        if !self.is_attached {
            let color = match self.part_type {
                PartType::Base => DARKGRAY,
                PartType::Middle => GRAY,
                PartType::Top => LIGHTGRAY,
            };
            draw_rectangle(self.entity.x, self.entity.y, self.entity.width, self.entity.height, color);
        }
    }
}

pub struct Rocket {
    pub x: f32,
    pub y: f32,
    pub parts_attached: Vec<PartType>,
    pub fuel_level: f32,
}

impl Rocket {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            parts_attached: vec![PartType::Base], // Base starts at site
            fuel_level: 0.0,
        }
    }

    pub fn draw(&self) {
        for (i, part) in self.parts_attached.iter().enumerate() {
            let color = match part {
                PartType::Base => DARKGRAY,
                PartType::Middle => GRAY,
                PartType::Top => LIGHTGRAY,
            };
            draw_rectangle(self.x, self.y - (i as f32 * 20.0), 40.0, 20.0, color);
        }

        // Draw fuel bar if fully assembled
        if self.parts_attached.len() == 3 {
            draw_rectangle(self.x + 45.0, self.y - 40.0, 10.0, 60.0, DARKBLUE);
            draw_rectangle(self.x + 45.0, self.y + 20.0 - (self.fuel_level * 60.0), 10.0, self.fuel_level * 60.0, SKYBLUE);
        }
    }
}
