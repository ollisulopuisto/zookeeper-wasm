use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct RectCollider {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl RectCollider {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self { x, y, w, h }
    }

    pub fn overlaps(&self, other: &RectCollider) -> bool {
        self.x < other.x + other.w &&
        self.x + self.w > other.x &&
        self.y < other.y + other.h &&
        self.y + self.h > other.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rect_overlaps() {
        let r1 = RectCollider::new(0.0, 0.0, 10.0, 10.0);
        let r2 = RectCollider::new(5.0, 5.0, 10.0, 10.0);
        let r3 = RectCollider::new(20.0, 20.0, 10.0, 10.0);
        
        assert!(r1.overlaps(&r2));
        assert!(r2.overlaps(&r1));
        assert!(!r1.overlaps(&r3));
    }
}

pub struct Entity {
    pub collider: RectCollider,
    pub vy: f32,
}

impl Entity {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            collider: RectCollider::new(x, y, w, h),
            vy: 0.0,
        }
    }
}
