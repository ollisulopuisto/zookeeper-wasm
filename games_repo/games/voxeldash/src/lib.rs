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
        // Generate procedural floor
        for x in -10..10 {
            for z in -10..10 {
                let h = ((x as f32 * 0.5).sin() * (z as f32 * 0.5).cos() * 2.0) as i32;
                world.push(Voxel {
                    position: [x, h - 1, z],
                    size: [1, 1, 1],
                    color: if (x + z) % 2 == 0 { "#333".to_string() } else { "#444".to_string() },
                });
            }
        }

        Self {
            player_pos: [0.0, 5.0, 0.0],
            player_vel: [0.0, 0.0, 0.0],
            world,
            goal_pos: [8, 2, 8],
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
                won: true,
            };
            return serde_wasm_bindgen::to_value(&scene).unwrap_or(JsValue::NULL);
        }
        self.time += dt;

        // Input handling
        let mut move_vec = [0.0, 0.0];
        for &key in keys {
            match key {
                87 => move_vec[1] -= 1.0, // W
                83 => move_vec[1] += 1.0, // S
                65 => move_vec[0] -= 1.0, // A
                68 => move_vec[0] += 1.0, // D
                32 => { // Space
                    if self.player_pos[1] <= self.get_floor_height(self.player_pos[0], self.player_pos[2]) + 0.1 {
                        self.player_vel[1] = 10.0;
                    }
                }
                _ => {}
            }
        }

        // Apply movement
        let speed = 5.0;
        self.player_pos[0] += move_vec[0] * speed * dt;
        self.player_pos[2] += move_vec[1] * speed * dt;

        // Gravity
        self.player_vel[1] -= 20.0 * dt;
        self.player_pos[1] += self.player_vel[1] * dt;

        // Collision with floor
        let floor_h = self.get_floor_height(self.player_pos[0], self.player_pos[2]);
        if self.player_pos[1] < floor_h {
            self.player_pos[1] = floor_h;
            self.player_vel[1] = 0.0;
        }

        // Goal check
        let dist_to_goal = ((self.player_pos[0] - self.goal_pos[0] as f32).powi(2) + 
                           (self.player_pos[1] - self.goal_pos[1] as f32).powi(2) + 
                           (self.player_pos[2] - self.goal_pos[2] as f32).powi(2)).sqrt();
        if dist_to_goal < 1.0 {
            self.won = true;
        }

        // Dynamic Camera
        if move_vec[0].abs() > 0.1 || move_vec[1].abs() > 0.1 {
            let target_angle = (move_vec[0].atan2(-move_vec[1])).to_degrees() + 180.0;
            // Smoothly rotate camera towards movement direction
            let diff = (target_angle - self.camera_angle + 540.0) % 360.0 - 180.0;
            self.camera_angle += diff * dt * 2.0;
        }

        let mut voxels = self.world.clone();
        // Goal
        voxels.push(Voxel {
            position: self.goal_pos,
            size: [1, 1, 1],
            color: "#ff0".to_string(), // Yellow goal
        });
        // Player
        voxels.push(Voxel {
            position: [self.player_pos[0] as i32, self.player_pos[1] as i32, self.player_pos[2] as i32],
            size: [1, 1, 1],
            color: "#0f0".to_string(),
        });

        let scene = Scene { 
            voxels,
            camera_angle: self.camera_angle,
            won: false,
        };
        serde_wasm_bindgen::to_value(&scene).unwrap_or(JsValue::NULL)
    }

    fn get_floor_height(&self, x: f32, z: f32) -> f32 {
        let ix = x.round() as i32;
        let iz = z.round() as i32;
        for v in &self.world {
            if v.position[0] == ix && v.position[2] == iz {
                return (v.position[1] + v.size[1]) as f32;
            }
        }
        -10.0
    }
}
