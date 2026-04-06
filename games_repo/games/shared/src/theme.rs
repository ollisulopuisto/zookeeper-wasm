use macroquad::prelude::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlockShape {
    Square,
    Circle,
    Diamond,
    Cross,
}

#[derive(Clone, Debug)]
pub struct Theme {
    pub name: String,
    pub color_a: Color,
    pub color_b: Color,
    pub bg_color: Color,
    pub ui_accent: Color,
    pub bpm: f32,
    pub shape_a: BlockShape,
    pub shape_b: BlockShape,
}

pub struct ThemeEngine {
    pub themes: Vec<Theme>,
    pub current_theme_idx: usize,
}

impl ThemeEngine {
    pub fn new(themes: Vec<Theme>) -> Self {
        Self {
            themes,
            current_theme_idx: 0,
        }
    }

    pub fn current(&self) -> &Theme {
        &self.themes[self.current_theme_idx]
    }

    pub fn set_level(&mut self, level: u32) -> bool {
        if self.themes.is_empty() { return false; }
        let old_idx = self.current_theme_idx;
        // Change theme every 10 levels (100 squares deleted) as a marker of progress
        self.current_theme_idx = (((level as usize).saturating_sub(1) / 10) % self.themes.len());
        self.current_theme_idx != old_idx
    }
}
