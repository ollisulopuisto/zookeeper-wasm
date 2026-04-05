use macroquad::prelude::Color;

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub color_a: Color,
    pub color_b: Color,
    pub bg_color: Color,
    pub ui_accent: Color,
    pub bpm: f32,
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

    pub fn set_level(&mut self, level: u32) {
        if self.themes.is_empty() { return; }
        // Change theme every 4 levels as a marker of progress
        self.current_theme_idx = ((level as usize).saturating_sub(1) / 4) % self.themes.len();
    }
}
