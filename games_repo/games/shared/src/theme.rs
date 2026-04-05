use macroquad::prelude::Color;

#[derive(Clone, Debug)]
pub struct Theme {
    pub name: String,
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

    pub fn set_score(&mut self, score: u32) -> bool {
        if self.themes.len() < 5 { return false; }
        
        let old_idx = self.current_theme_idx;
        self.current_theme_idx = if score >= 10000 {
            4 // Inferno
        } else if score >= 5000 {
            3 // Crystal
        } else if score >= 2000 {
            2 // Retro
        } else if score >= 500 {
            1 // Neon
        } else {
            0 // Classic
        };
        
        self.current_theme_idx != old_idx
    }

    pub fn set_level(&mut self, level: u32) -> bool {
        if self.themes.is_empty() { return false; }
        let old_idx = self.current_theme_idx;
        // Change theme every 4 levels as a marker of progress
        self.current_theme_idx = ((level as usize).saturating_sub(1) / 4) % self.themes.len();
        self.current_theme_idx != old_idx
    }
}
