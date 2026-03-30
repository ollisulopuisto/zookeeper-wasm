use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Voxel {
    pub position: [i32; 3],
    pub size: [i32; 3],
    pub color: String,
}

#[derive(Serialize, Deserialize)]
pub struct Scene {
    pub voxels: Vec<Voxel>,
    pub camera_angle: f32,
    pub camera_target: [f32; 3],
    pub won: bool,
}

#[wasm_bindgen]
pub struct Game {
    player_pos: [f32; 3],
    player_vel: [f32; 3],
    world: Vec<Voxel>,
    goal_pos: [i32; 3],
    camera_angle: f32,
    time: f32,
    won: bool,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut world = Vec::new();
        // Generate procedural colorful world with mixed shapes
        for x in -12..12 {
            for z in -12..12 {
                let fx = x as f32 * 0.4;
                let fz = z as f32 * 0.4;
                let h = ((fx.sin() * fz.cos() * 3.0) + (fx * 0.5).cos() * 2.0) as i32;
                
                // Base ground
                let color = if (x + z) % 2 == 0 { 
                    format!("hsl({}, 30%, 20%)", (x * 10 + z * 10 + 180) % 360) 
                } else { 
                    format!("hsl({}, 30%, 25%)", (x * 10 + z * 10 + 200) % 360) 
                };
                
                world.push(Voxel {
                    position: [x, h - 2, z],
                    size: [1, 1, 1],
                    color,
                });

                // Occasional larger pillars
                if (x % 4 == 0 && z % 4 == 0) && (x != 0 || z != 0) {
                    world.push(Voxel {
                        position: [x, h - 1, z],
                        size: [1, (h.abs() % 4) + 2, 1],
                        color: format!("hsl({}, 60%, 40%)", (x * 20) % 360),
                    });
                }
                
                // Some big flat platforms
                if x % 6 == 0 && z % 6 == 0 {
                    world.push(Voxel {
                        position: [x - 1, h + 1, z - 1],
                        size: [3, 1, 3],
                        color: "#555".to_string(),
                    });
                }
            }
        }

        Self {
            player_pos: [0.0, 5.0, 0.0],
            player_vel: [0.0, 0.0, 0.0],
            world,
            goal_pos: [10, 4, 10],
            camera_angle: 315.0,
            time: 0.0,
            won: false,
        }
    }

    pub fn tick(&mut self, dt: f32, keys: &[u8]) -> JsValue {
        if self.won {
            let scene = Scene { 
                voxels: self.world.clone(),
                camera_angle: self.camera_angle,
                camera_target: self.player_pos,
                won: true,
            };
            return serde_wasm_bindgen::to_value(&scene).unwrap_or(JsValue::NULL);
        }
        self.time += dt;

        // Input handling
        let mut move_vec = [0.0, 0.0];
        for &key in keys {
            match key {
                87 | 38 => move_vec[1] -= 1.0, // W / Up
                83 | 40 => move_vec[1] += 1.0, // S / Down
                65 | 37 => move_vec[0] -= 1.0, // A / Left
                68 | 39 => move_vec[0] += 1.0, // D / Right
                32 => { // Space
                    if self.player_pos[1] <= self.get_floor_height(self.player_pos[0], self.player_pos[2]) + 0.1 {
                        self.player_vel[1] = 12.0;
                    }
                }
                _ => {}
            }
        }

        // Apply movement relative to camera? For now simple axis-aligned
        let speed = 7.0;
        self.player_pos[0] += move_vec[0] * speed * dt;
        self.player_pos[2] += move_vec[1] * speed * dt;

        // Gravity
        self.player_vel[1] -= 25.0 * dt;
        self.player_pos[1] += self.player_vel[1] * dt;

        // Collision with world
        let floor_h = self.get_floor_height(self.player_pos[0], self.player_pos[2]);
        if self.player_pos[1] < floor_h {
            self.player_pos[1] = floor_h;
            self.player_vel[1] = 0.0;
        }

        // Goal check
        let dist_to_goal = ((self.player_pos[0] - self.goal_pos[0] as f32).powi(2) + 
                           (self.player_pos[1] - self.goal_pos[1] as f32).powi(2) + 
                           (self.player_pos[2] - self.goal_pos[2] as f32).powi(2)).sqrt();
        if dist_to_goal < 1.5 {
            self.won = true;
        }

        // Camera follow logic
        if move_vec[0].abs() > 0.1 || move_vec[1].abs() > 0.1 {
            let target_angle = (move_vec[0].atan2(-move_vec[1])).to_degrees() + 180.0;
            let diff = (target_angle - self.camera_angle + 540.0) % 360.0 - 180.0;
            self.camera_angle += diff * dt * 3.0;
        }

        let mut voxels = self.world.clone();
        // Goal - glowing gold
        voxels.push(Voxel {
            position: self.goal_pos,
            size: [1, 1, 1],
            color: format!("hsl(50, 100%, {}%)", 50.0 + (self.time * 5.0).sin() * 20.0),
        });
        // Player - neon green
        voxels.push(Voxel {
            position: [self.player_pos[0] as i32, self.player_pos[1] as i32, self.player_pos[2] as i32],
            size: [1, 1, 1],
            color: "#0f0".to_string(),
        });

        let scene = Scene { 
            voxels,
            camera_angle: self.camera_angle,
            camera_target: self.player_pos,
            won: false,
        };
        serde_wasm_bindgen::to_value(&scene).unwrap_or(JsValue::NULL)
    }

    fn get_floor_height(&self, x: f32, z: f32) -> f32 {
        let mut max_h = -20.0;
        let px = x.round() as i32;
        let pz = z.round() as i32;

        for v in &self.world {
            // Check if player is within the horizontal bounds of the voxel
            if px >= v.position[0] && px < v.position[0] + v.size[0] &&
               pz >= v.position[2] && pz < v.position[2] + v.size[2] {
                let top = (v.position[1] + v.size[1]) as f32;
                if top > max_h {
                    max_h = top;
                }
            }
        }
        max_h
    }
}
