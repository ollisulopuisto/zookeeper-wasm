use macroquad::prelude::*;

/// Consumes all pending keyboard events in the Macroquad input buffer.
/// This prevents keys pressed just before a game transition (e.g. game over)
/// from leaking into high score name entry or the next game state.
pub fn clear_keyboard_buffer() {
    // Consume character queue
    while get_char_pressed().is_some() {}
    // Consume key queue
    while get_last_key_pressed().is_some() {}
}

#[derive(Clone, PartialEq, Debug)]
pub struct TextInput {
    pub content: String,
    pub max_len: usize,
    cleared: bool,
}

impl TextInput {
    pub fn new(max_len: usize, initial_content: String) -> Self {
        Self {
            content: initial_content,
            max_len,
            cleared: false,
        }
    }

    /// Updates the text input from keyboard events.
    /// Returns true if Enter was pressed.
    pub fn update(&mut self) -> bool {
        if !self.cleared {
            clear_keyboard_buffer();
            self.cleared = true;
            // We skip the rest of the update in the first frame to ensure
            // that any keys pressed *exactly* during the transition are ignored.
            return false;
        }

        while let Some(c) = get_char_pressed() {
            if (c.is_alphanumeric() || c == ' ') && self.content.len() < self.max_len {
                self.content.push(c);
            }
        }
        if is_key_pressed(KeyCode::Backspace) {
            self.content.pop();
        }
        is_key_pressed(KeyCode::Enter) || is_key_pressed(KeyCode::KpEnter)
    }
}
