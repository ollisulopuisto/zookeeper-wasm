use macroquad::prelude::Color;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum BlockColor {
    ColorA,
    ColorB,
}

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

impl Theme {
    pub fn get_color(&self, color: BlockColor) -> Color {
        match color {
            BlockColor::ColorA => self.color_a,
            BlockColor::ColorB => self.color_b,
        }
    }

    pub fn get_shape(&self, color: BlockColor) -> BlockShape {
        match color {
            BlockColor::ColorA => self.shape_a,
            BlockColor::ColorB => self.shape_b,
        }
    }
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

    pub fn get_suggested_theme_idx(&self, level: u32) -> usize {
        if self.themes.is_empty() {
            return 0;
        }
        // Change theme every 10 levels (100 squares deleted) as a marker of progress
        ((level as usize).saturating_sub(1) / 10) % self.themes.len()
    }
}
