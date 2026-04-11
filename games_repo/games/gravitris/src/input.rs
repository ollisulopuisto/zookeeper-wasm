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
    touch_start_pos: Option<Vec2>,
    swipe_handled: bool,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            p1: PlayerInput::default(),
            any_key: false,
            touch_active: false,
            touch_start_pos: None,
            swipe_handled: false,
        }
    }

    pub fn update(&mut self, _virtual_width: f32, _virtual_height: f32) {
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

        // Gesture-based Touch handling
        let touches = touches();
        if !touches.is_empty() {
            self.touch_active = true;
        }

        for touch in touches {
            match touch.phase {
                TouchPhase::Started => {
                    self.touch_start_pos = Some(touch.position);
                    self.swipe_handled = false;
                    self.any_key = true;
                }
                TouchPhase::Moved => {
                    if let Some(start_pos) = self.touch_start_pos {
                        if !self.swipe_handled {
                            let diff = touch.position - start_pos;
                            let dist_x = diff.x.abs();
                            let dist_y = diff.y.abs();
                            
                            // Threshold for swipe detection (adjust as needed)
                            let threshold = 25.0; 

                            if dist_x > threshold || dist_y > threshold {
                                if dist_x > dist_y {
                                    if diff.x > 0.0 {
                                        self.p1.right = true;
                                    } else {
                                        self.p1.left = true;
                                    }
                                } else {
                                    if diff.y > 0.0 {
                                        self.p1.down = true;
                                    }
                                }
                                self.swipe_handled = true;
                            }
                        }
                    }
                }
                TouchPhase::Ended => {
                    if let Some(_start_pos) = self.touch_start_pos {
                        if !self.swipe_handled {
                            // Tap detection: if no swipe was triggered, it's a tap
                            self.p1.rotate = true;
                        }
                    }
                    self.touch_start_pos = None;
                }
                _ => {}
            }
        }
    }

    pub fn draw_controls(&self, _vx: f32, _vy: f32, _scale: f32, _virtual_width: f32, _virtual_height: f32) {
        // Virtual gamepad removed as requested.
    }
}
