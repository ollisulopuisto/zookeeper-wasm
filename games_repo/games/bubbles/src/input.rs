use macroquad::prelude::*;

#[derive(Default)]
pub struct PlayerInput {
    pub left: bool,
    pub right: bool,
    pub jump: bool,
    pub bubble: bool,
}

pub struct InputManager {
    pub p1: PlayerInput,
    pub p2: PlayerInput,
    pub any_key: bool,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            p1: PlayerInput::default(),
            p2: PlayerInput::default(),
            any_key: false,
        }
    }

    pub fn update(&mut self, virtual_width: f32, virtual_height: f32) {
        // Keyboard P1
        self.p1.left = is_key_down(KeyCode::Left);
        self.p1.right = is_key_down(KeyCode::Right);
        self.p1.jump = is_key_pressed(KeyCode::Z) || is_key_pressed(KeyCode::Up);
        self.p1.bubble = is_key_pressed(KeyCode::X) || is_key_pressed(KeyCode::Space);

        // Keyboard P2
        self.p2.left = is_key_down(KeyCode::A);
        self.p2.right = is_key_down(KeyCode::D);
        self.p2.jump = is_key_pressed(KeyCode::W) || is_key_pressed(KeyCode::Q);
        self.p2.bubble = is_key_pressed(KeyCode::E) || is_key_pressed(KeyCode::S);

        self.any_key = get_last_key_pressed().is_some();

        // Touch handling
        let screen_width = screen_width();
        let screen_height = screen_height();
        let scale_x = screen_width / virtual_width;
        let scale_y = screen_height / virtual_height;

        for touch in touches() {
            let tx = touch.position.x / scale_x;
            let ty = touch.position.y / scale_y;

            // Simple region-based touch controls
            // Bottom left: Left/Right
            if ty > virtual_height * 0.7 {
                if tx < virtual_width * 0.2 {
                    self.p1.left = true;
                } else if tx < virtual_width * 0.4 {
                    self.p1.right = true;
                }
            }
            // Bottom right: Jump/Bubble
            if ty > virtual_height * 0.7 {
                if tx > virtual_width * 0.8 {
                    if touch.phase == TouchPhase::Started { self.p1.bubble = true; }
                } else if tx > virtual_width * 0.6 {
                    if touch.phase == TouchPhase::Started { self.p1.jump = true; }
                }
            }
            
            if touch.phase == TouchPhase::Started {
                self.any_key = true;
            }
        }
    }

    pub fn draw_debug_touch_regions(&self, virtual_width: f32, virtual_height: f32) {
        // Only draw if there are touches or for debugging
        let alpha = 0.3;
        // Left
        draw_rectangle(0.0, virtual_height * 0.8, virtual_width * 0.2, virtual_height * 0.2, Color::new(1.0, 1.0, 1.0, alpha));
        // Right
        draw_rectangle(virtual_width * 0.2, virtual_height * 0.8, virtual_width * 0.2, virtual_height * 0.2, Color::new(0.8, 0.8, 0.8, alpha));
        // Jump
        draw_rectangle(virtual_width * 0.6, virtual_height * 0.8, virtual_width * 0.2, virtual_height * 0.2, Color::new(0.0, 1.0, 0.0, alpha));
        // Bubble
        draw_rectangle(virtual_width * 0.8, virtual_height * 0.8, virtual_width * 0.2, virtual_height * 0.2, Color::new(1.0, 0.0, 0.0, alpha));
        
        draw_text("L", 10.0, virtual_height - 20.0, 30.0, WHITE);
        draw_text("R", virtual_width * 0.2 + 10.0, virtual_height - 20.0, 30.0, WHITE);
        draw_text("J", virtual_width * 0.6 + 10.0, virtual_height - 20.0, 30.0, WHITE);
        draw_text("B", virtual_width * 0.8 + 10.0, virtual_height - 20.0, 30.0, WHITE);
    }
}
