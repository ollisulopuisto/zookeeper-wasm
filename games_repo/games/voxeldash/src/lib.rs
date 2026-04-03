use wasm_bindgen::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Scene {
    #[serde(skip)]
    pub height_map_js: JsValue,
    pub player_pos: [f32; 3],
    pub goal_pos: [i32; 3],
    pub camera_yaw: f32,
    pub camera_y: f32,
    pub camera_distance: f32,
    pub is_lifting: bool,
    pub is_carving: bool,
    pub won: bool,
}

// Custom manual serialization for Scene to include the height_map typed array efficiently
impl Scene {
    fn to_js_value(&self) -> JsValue {
        let val = serde_wasm_bindgen::to_value(&self).unwrap();
        js_sys::Reflect::set(&val, &JsValue::from_str("height_map"), &self.height_map_js).unwrap();
        val
    }
}

#[wasm_bindgen]
pub struct Game {
    player_pos: [f32; 3],
    player_vel: [f32; 3],
    height_map: Vec<i32>,
    goal_pos: [i32; 3],
    camera_yaw: f32,
    camera_y: f32,
    camera_distance: f32,
    time: f32,
    is_lifting: bool,
    is_carving: bool,
    won: bool,
}

#[wasm_bindgen]
impl Game {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut height_map = vec![0; 31 * 31];
        for x in 0..31 {
            for z in 0..31 {
                let fx = x as f32;
                let fz = z as f32;
                let noise = ((fx * 0.2).sin() * 3.0 + (fz * 0.2).cos() * 3.0 + (fx * 0.5).sin() * 2.0) as i32;
                height_map[x * 31 + z] = noise.max(-2);
            }
        }

        Self {
            player_pos: [0.0, 5.0, 0.0],
            player_vel: [0.0, 0.0, 0.0],
            height_map,
            goal_pos: [12, 2, -12], 
            camera_yaw: 315.0,
            camera_y: -20.0,
            camera_distance: 40.0,
            time: 0.0,
            is_lifting: false,
            is_carving: false,
            won: false,
        }
    }

    pub fn tick(&mut self, dt: f32, keys: &[u8], cam_yaw: f32, cam_y: f32, cam_dist: f32) -> JsValue {
        if self.won {
            let scene = Scene { 
                height_map_js: js_sys::Int32Array::from(&self.height_map[..]).into(),
                player_pos: self.player_pos,
                goal_pos: self.goal_pos,
                camera_yaw: self.camera_yaw,
                camera_y: self.camera_y,
                camera_distance: self.camera_distance,
                is_lifting: self.is_lifting,
                is_carving: self.is_carving,
                won: true,
            };
            return scene.to_js_value();
        }
        self.time += dt;

        // Input
        self.is_lifting = false;
        self.is_carving = false;
        let mut move_vec = [0.0, 0.0];
        for &key in keys {
            match key {
                87 | 38 => move_vec[1] -= 1.0, // W
                83 | 40 => move_vec[1] += 1.0, // S
                65 | 37 => move_vec[0] -= 1.0, // A
                68 | 39 => move_vec[0] += 1.0, // D
                32 => { // Space
                    if self.player_pos[1] <= self.get_floor_height(self.player_pos[0], self.player_pos[2]) + 0.1 {
                        self.player_vel[1] = 13.0;
                    }
                }
                16 => self.is_lifting = true, // Shift
                69 => self.is_carving = true, // E
                _ => {}
            }
        }

        // Carving Logic
        if self.is_carving {
            let px = (self.player_pos[0].round() as i32 + 15).clamp(0, 30);
            let pz = (self.player_pos[2].round() as i32 + 15).clamp(0, 30);
            for ox in -2..=2 {
                for oz in -2..=2 {
                    let nx = (px + ox).clamp(0, 30) as usize;
                    let nz = (pz + oz).clamp(0, 30) as usize;
                    let idx = nx * 31 + nz;
                    if self.height_map[idx] > -3 {
                        if self.time % 0.1 < dt {
                            self.height_map[idx] -= 1;
                        }
                    }
                }
            }
        }

        // Physics
        let speed = 8.0;
        let yaw_rad = (self.camera_yaw - 90.0).to_radians();
        let world_dx = move_vec[0] * yaw_rad.cos() - move_vec[1] * yaw_rad.sin();
        let world_dz = move_vec[0] * yaw_rad.sin() + move_vec[1] * yaw_rad.cos();
        self.player_pos[0] += world_dx * speed * dt;
        self.player_pos[2] += world_dz * speed * dt;
        self.player_vel[1] -= 35.0 * dt;
        self.player_pos[1] += self.player_vel[1] * dt;

        let floor_h = self.get_floor_height(self.player_pos[0], self.player_pos[2]);
        if self.player_pos[1] < floor_h {
            self.player_pos[1] = floor_h;
            self.player_vel[1] = 0.0;
        }

        // Goal
        let dist = ((self.player_pos[0]-self.goal_pos[0] as f32).powi(2) + 
                    (self.player_pos[1]-self.goal_pos[1] as f32).powi(2) + 
                    (self.player_pos[2]-self.goal_pos[2] as f32).powi(2)).sqrt();
        if dist < 2.0 { self.won = true; }

        self.camera_yaw = cam_yaw;
        self.camera_y = cam_y;
        self.camera_distance = cam_dist;

        let scene = Scene { 
            height_map_js: js_sys::Int32Array::from(&self.height_map[..]).into(),
            player_pos: self.player_pos,
            goal_pos: self.goal_pos,
            camera_yaw: self.camera_yaw,
            camera_y: self.camera_y,
            camera_distance: self.camera_distance,
            is_lifting: self.is_lifting,
            is_carving: self.is_carving,
            won: false,
        };
        scene.to_js_value()
    }

    fn get_floor_height(&self, x: f32, z: f32) -> f32 {
        let ix = (x.round() as i32 + 15).clamp(0, 30);
        let iz = (z.round() as i32 + 15).clamp(0, 30);
        let base_h = self.height_map[(ix * 31 + iz) as usize] as f32;
        if self.is_lifting { base_h + 3.5 } else { base_h }
    }
}
