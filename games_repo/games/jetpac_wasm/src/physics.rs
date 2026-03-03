pub struct Entity {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub width: f32,
    pub height: f32,
}

impl Entity {
    #[must_use]
    pub fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, vx: 0.0, vy: 0.0, width, height }
    }
}

pub fn update_physics(entity: &mut Entity, dt: f32) {
    const GRAVITY: f32 = 500.0;
    entity.vy += GRAVITY * dt;
    entity.x += entity.vx * dt;
    entity.y += entity.vy * dt;
}

pub fn wrap_around(entity: &mut Entity, screen_width: f32) {
    if entity.x > screen_width {
        entity.x -= screen_width;
    } else if entity.x < 0.0 {
        entity.x += screen_width;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gravity_pull() {
        let mut e = Entity::new(100.0, 100.0, 32.0, 32.0);
        update_physics(&mut e, 1.0);
        assert!(e.vy > 0.0, "Entity should have positive vertical velocity after gravity pull");
        assert!(e.y > 100.0, "Entity should have moved down");
    }

    #[test]
    fn test_horizontal_wrap() {
        let screen_width = 800.0;
        let mut e = Entity::new(screen_width + 10.0, 100.0, 32.0, 32.0);
        wrap_around(&mut e, screen_width);
        assert!(e.x < screen_width, "Entity should wrap around left side");

        e.x = -10.0;
        wrap_around(&mut e, screen_width);
        assert!(e.x > 0.0, "Entity should wrap around right side");
    }
}
