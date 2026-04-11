use macroquad::prelude::*;

#[derive(Default, Clone, Copy)]
pub struct PlayerInput {
    pub left: bool,
    pub right: bool,
    pub down: bool,
    pub rotate: bool,
    pub drop: bool,
}

pub struct InputManager {
    pub p1: PlayerInput,
    pub any_key: bool,
    pub touch_active: bool,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            p1: PlayerInput::default(),
            any_key: false,
            touch_active: false,
        }
    }

    pub fn update(&mut self, virtual_width: f32, virtual_height: f32) {
        // Reset state for new frame
        self.p1 = PlayerInput::default();
        self.any_key = false;

        // Keyboard P1 (Arrows or WASD)
        self.p1.left = is_key_pressed(KeyCode::Left) || is_key_pressed(KeyCode::A);
        self.p1.right = is_key_pressed(KeyCode::Right) || is_key_pressed(KeyCode::D);
        self.p1.down = is_key_pressed(KeyCode::Down) || is_key_pressed(KeyCode::S);
        self.p1.rotate = is_key_pressed(KeyCode::Up) || is_key_pressed(KeyCode::W);
        self.p1.drop = is_key_pressed(KeyCode::Space);

        if get_last_key_pressed().is_some() {
            self.any_key = true;
        }

        // Touch handling
        let sw = screen_width();
        let sh = screen_height();
        let scale_x = sw / virtual_width;
        let scale_y = sh / virtual_height;
        let scale = scale_x.min(scale_y);
        
        let vx = (sw - virtual_width * scale) / 2.0;
        let vy = (sh - virtual_height * scale) / 2.0;

        let touches = touches();
        if !touches.is_empty() {
            self.touch_active = true;
        }

        for touch in touches {
            let tx = (touch.position.x - vx) / scale;
            let ty = (touch.position.y - vy) / scale;

            // Define control regions in virtual coordinates
            let dpad_y = virtual_height - 100.0;
            if ty > dpad_y {
                if touch.phase == TouchPhase::Started {
                    if tx < 60.0 {
                        self.p1.left = true;
                    } else if tx < 120.0 {
                        self.p1.right = true;
                    } else if tx > virtual_width - 60.0 {
                        self.p1.drop = true;
                    } else if tx > virtual_width - 120.0 {
                        self.p1.rotate = true;
                    } else {
                        self.p1.down = true;
                    }
                }
            }
            
            if touch.phase == TouchPhase::Started {
                self.any_key = true;
            }
        }
    }

    pub fn draw_controls(&self, vx: f32, vy: f32, scale: f32, virtual_width: f32, virtual_height: f32) {
        if !self.touch_active { return; }

        let alpha = 0.4;
        let dpad_y = vy + (virtual_height - 100.0) * scale;
        let btn_size = 50.0 * scale;
        let font_size = 18.0 * scale;

        // Draw D-Pad (L/R/D)
        draw_rectangle(vx + 10.0 * scale, dpad_y + 20.0 * scale, btn_size, btn_size, Color::new(0.5, 0.5, 0.5, alpha));
        draw_text("L", vx + 25.0 * scale, dpad_y + 55.0 * scale, font_size, WHITE);

        draw_rectangle(vx + 70.0 * scale, dpad_y + 20.0 * scale, btn_size, btn_size, Color::new(0.5, 0.5, 0.5, alpha));
        draw_text("R", vx + 85.0 * scale, dpad_y + 55.0 * scale, font_size, WHITE);

        let down_x = vx + (virtual_width / 2.0 - 25.0) * scale;
        draw_rectangle(down_x, dpad_y + 20.0 * scale, btn_size, btn_size, Color::new(0.5, 0.5, 0.5, alpha));
        draw_text("D", down_x + 15.0 * scale, dpad_y + 55.0 * scale, font_size, WHITE);

        // Draw Action Buttons (Rotate/Drop)
        let rot_x = vx + (virtual_width - 110.0) * scale;
        draw_circle(rot_x + btn_size/2.0, dpad_y + 20.0 * scale + btn_size/2.0, btn_size/2.0, Color::new(0.0, 0.8, 0.0, alpha));
        draw_text("ROT", rot_x + 5.0 * scale, dpad_y + 55.0 * scale, font_size * 0.8, WHITE);

        let drop_x = vx + (virtual_width - 55.0) * scale;
        draw_circle(drop_x + btn_size/2.0, dpad_y + 20.0 * scale + btn_size/2.0, btn_size/2.0, Color::new(0.8, 0.0, 0.0, alpha));
        draw_text("DROP", drop_x + 5.0 * scale, dpad_y + 55.0 * scale, font_size * 0.8, WHITE);
    }
}
