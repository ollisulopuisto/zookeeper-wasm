use macroquad::prelude::*;
use crate::physics::Entity;

pub const SCREEN_WIDTH: f32 = 800.0;
pub const SCREEN_HEIGHT: f32 = 600.0;
pub const TILE_SIZE: f32 = 32.0;
pub const COLS: usize = 25;
pub const ROWS: usize = 18; // 18 * 32 = 576, leaves 24 for HUD

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Empty,
    NormalBrick,
    SolidBrick,
    Ladder,
}

pub struct PhasedBrick {
    pub col: usize,
    pub row: usize,
    pub timer: f32,
}

#[derive(Clone, Copy, PartialEq)]
pub enum CollectibleType {
    Emerald,
    Fuel,
}

pub struct Collectible {
    pub col: usize,
    pub row: usize,
    pub ctype: CollectibleType,
    pub active: bool,
}

pub struct Portal {
    pub col: usize,
    pub row: usize,
    pub active: bool,
}

pub struct Player {
    pub entity: Entity,
    pub fuel: f32,
    pub is_jetting: bool,
    pub facing_right: bool,
    pub dead: bool,
    pub phase_cooldown: f32,
}

pub struct Enemy {
    pub entity: Entity,
    pub facing_right: bool,
}

pub struct Level {
    pub grid: [[TileType; COLS]; ROWS],
    pub phased_bricks: Vec<PhasedBrick>,
    pub collectibles: Vec<Collectible>,
    pub portal: Portal,
    pub emeralds_total: usize,
    pub emeralds_collected: usize,
}

impl Level {
    pub fn draw(&self) {
        for r in 0..ROWS {
            for c in 0..COLS {
                let x = c as f32 * TILE_SIZE;
                let y = r as f32 * TILE_SIZE;
                match self.grid[r][c] {
                    TileType::NormalBrick => {
                        draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, MAROON);
                        draw_rectangle(x + 2.0, y + 2.0, TILE_SIZE - 4.0, TILE_SIZE - 4.0, RED);
                        draw_line(x, y + TILE_SIZE / 2.0, x + TILE_SIZE, y + TILE_SIZE / 2.0, 2.0, MAROON);
                    }
                    TileType::SolidBrick => {
                        draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, DARKGRAY);
                        draw_rectangle(x + 2.0, y + 2.0, TILE_SIZE - 4.0, TILE_SIZE - 4.0, GRAY);
                    }
                    TileType::Ladder => {
                        draw_rectangle(x + 6.0, y, 4.0, TILE_SIZE, DARKBROWN);
                        draw_rectangle(x + TILE_SIZE - 10.0, y, 4.0, TILE_SIZE, DARKBROWN);
                        for i in 0..4 {
                            let ly = y + 4.0 + i as f32 * 8.0;
                            draw_rectangle(x + 6.0, ly, TILE_SIZE - 12.0, 4.0, DARKBROWN);
                        }
                    }
                    TileType::Empty => {}
                }
            }
        }

        // Draw phased bricks (fading effect)
        for pb in &self.phased_bricks {
            let x = pb.col as f32 * TILE_SIZE;
            let y = pb.row as f32 * TILE_SIZE;
            let alpha = if pb.timer < 1.0 { 1.0 - pb.timer } else { 0.2 };
            draw_rectangle(x, y, TILE_SIZE, TILE_SIZE, Color::new(1.0, 0.0, 0.0, alpha));
        }

        // Draw collectibles
        for col in &self.collectibles {
            if col.active {
                let x = col.col as f32 * TILE_SIZE + TILE_SIZE / 2.0;
                let y = col.row as f32 * TILE_SIZE + TILE_SIZE / 2.0;
                let t = get_time() * 5.0;
                let bounce = (t.sin() * 4.0) as f32;
                match col.ctype {
                    CollectibleType::Emerald => {
                        draw_poly(x, y + bounce, 4, 12.0, 0.0, GREEN);
                        draw_poly(x, y + bounce, 4, 6.0, 0.0, LIME);
                    }
                    CollectibleType::Fuel => {
                        draw_rectangle(x - 8.0, y - 10.0 + bounce, 16.0, 20.0, YELLOW);
                        draw_rectangle(x - 6.0, y - 8.0 + bounce, 12.0, 16.0, GOLD);
                        draw_rectangle(x - 4.0, y - 12.0 + bounce, 8.0, 4.0, GRAY);
                    }
                }
            }
        }

        // Draw Portal
        let px = self.portal.col as f32 * TILE_SIZE;
        let py = self.portal.row as f32 * TILE_SIZE;
        draw_rectangle(px, py, TILE_SIZE * 2.0, TILE_SIZE * 2.0, DARKGRAY);
        if self.portal.active {
            draw_rectangle(px + 4.0, py + 4.0, TILE_SIZE * 2.0 - 8.0, TILE_SIZE * 2.0 - 8.0, BLACK);
            let t = get_time() * 10.0;
            for i in 0..5 {
                let r = (t + i as f64).sin() as f32 * 10.0 + 15.0;
                draw_circle(px + TILE_SIZE, py + TILE_SIZE, r, Color::new(0.5, 0.0, 1.0, 0.3));
            }
            draw_circle(px + TILE_SIZE, py + TILE_SIZE, 10.0, MAGENTA);
        } else {
            // Closed doors
            draw_rectangle(px + 4.0, py + 4.0, TILE_SIZE - 4.0, TILE_SIZE * 2.0 - 8.0, RED);
            draw_rectangle(px + TILE_SIZE, py + 4.0, TILE_SIZE - 4.0, TILE_SIZE * 2.0 - 8.0, RED);
            draw_line(px + TILE_SIZE, py + 4.0, px + TILE_SIZE, py + TILE_SIZE * 2.0 - 4.0, 2.0, BLACK);
        }
    }
}

impl Player {
    pub fn new(col: usize, row: usize) -> Self {
        Self {
            entity: Entity::new(col as f32 * TILE_SIZE + 4.0, row as f32 * TILE_SIZE, 24.0, 30.0),
            fuel: 100.0,
            is_jetting: false,
            facing_right: true,
            dead: false,
            phase_cooldown: 0.0,
        }
    }

    pub fn draw(&self) {
        if self.dead {
            return;
        }
        let color_main = if self.is_jetting { YELLOW } else { GREEN };
        let color_shadow = DARKGREEN;
        
        draw_rectangle(self.entity.collider.x, self.entity.collider.y, self.entity.collider.w, self.entity.collider.h, color_shadow);
        draw_rectangle(self.entity.collider.x + 2.0, self.entity.collider.y + 2.0, self.entity.collider.w - 4.0, self.entity.collider.h - 4.0, color_main);
        
        // Visor
        let visor_x = if self.facing_right { self.entity.collider.x + 12.0 } else { self.entity.collider.x + 2.0 };
        draw_rectangle(visor_x, self.entity.collider.y + 6.0, 10.0, 8.0, SKYBLUE);

        // Jetpack
        let pack_x = if self.facing_right { self.entity.collider.x - 6.0 } else { self.entity.collider.x + self.entity.collider.w - 2.0 };
        draw_rectangle(pack_x, self.entity.collider.y + 6.0, 8.0, 16.0, GRAY);

        if self.is_jetting {
            let flame_y = self.entity.collider.y + 22.0;
            draw_triangle(
                vec2(pack_x, flame_y),
                vec2(pack_x + 8.0, flame_y),
                vec2(pack_x + 4.0, flame_y + 10.0 + rand::gen_range(0.0, 8.0)),
                ORANGE
            );
        }
    }
}

impl Enemy {
    pub fn new(col: usize, row: usize) -> Self {
        Self {
            entity: Entity::new(col as f32 * TILE_SIZE + 4.0, row as f32 * TILE_SIZE + 8.0, 24.0, 24.0),
            facing_right: true,
        }
    }

    pub fn draw(&self) {
        let t = get_time() * 10.0;
        let wobble = (t.sin() * 2.0) as f32;
        let x = self.entity.collider.x;
        let y = self.entity.collider.y;
        
        // Trackbot style
        draw_rectangle(x, y - wobble, 24.0, 24.0, PURPLE);
        draw_rectangle(x + 2.0, y + 2.0 - wobble, 20.0, 20.0, MAGENTA);
        
        // Treads
        draw_rectangle(x - 2.0, y + 20.0, 28.0, 6.0, DARKGRAY);
        
        let eye_x = if self.facing_right { x + 14.0 } else { x + 4.0 };
        draw_rectangle(eye_x, y + 6.0 - wobble, 6.0, 6.0, RED);
    }
}

pub fn create_test_level() -> Level {
    let mut grid = [[TileType::Empty; COLS]; ROWS];
    
    // Border
    for c in 0..COLS {
        grid[0][c] = TileType::SolidBrick;
        grid[ROWS-1][c] = TileType::SolidBrick;
    }
    for r in 0..ROWS {
        grid[r][0] = TileType::SolidBrick;
        grid[r][COLS-1] = TileType::SolidBrick;
    }

    // Platforms
    for c in 2..10 { grid[14][c] = TileType::NormalBrick; }
    for c in 12..20 { grid[10][c] = TileType::NormalBrick; }
    for c in 4..15 { grid[6][c] = TileType::NormalBrick; }

    // Indestructible blocks
    grid[14][5] = TileType::SolidBrick;
    grid[10][15] = TileType::SolidBrick;

    // Ladders
    for r in 11..18 { grid[r][10] = TileType::Ladder; }
    for r in 7..11 { grid[r][18] = TileType::Ladder; }

    let collectibles = vec![
        Collectible { col: 3, row: 13, ctype: CollectibleType::Emerald, active: true },
        Collectible { col: 9, row: 13, ctype: CollectibleType::Emerald, active: true },
        Collectible { col: 14, row: 9, ctype: CollectibleType::Emerald, active: true },
        Collectible { col: 19, row: 9, ctype: CollectibleType::Emerald, active: true },
        Collectible { col: 5, row: 5, ctype: CollectibleType::Emerald, active: true },
        Collectible { col: 13, row: 5, ctype: CollectibleType::Emerald, active: true },
        Collectible { col: 7, row: 17, ctype: CollectibleType::Fuel, active: true },
        Collectible { col: 15, row: 17, ctype: CollectibleType::Fuel, active: true },
    ];

    Level {
        grid,
        phased_bricks: Vec::new(),
        emeralds_total: 6,
        emeralds_collected: 0,
        collectibles,
        portal: Portal { col: 20, row: 16, active: false }, // 2x2 portal at bottom right
    }
}
