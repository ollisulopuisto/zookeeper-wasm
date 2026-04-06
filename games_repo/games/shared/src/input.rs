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

    /// Extends update with touch-based mobile prompting for WASM.
    /// `prompt_rect` is where the user taps to open native OS input.
    /// `ok_rect` is the OK/Submit button.
    pub fn update_with_touch(
        &mut self,
        prompt_rect: (f32, f32, f32, f32),
        ok_rect: (f32, f32, f32, f32),
        is_mobile: bool,
    ) -> bool {
        let mut submitted = self.update();

        let (mx, my) = mouse_position();
        let tapped = is_mouse_button_pressed(MouseButton::Left);

        if tapped {
            // Check OK button
            if mx >= ok_rect.0
                && mx <= ok_rect.0 + ok_rect.2
                && my >= ok_rect.1
                && my <= ok_rect.1 + ok_rect.3
            {
                submitted = true;
            }

            // Check native prompt trigger (only on mobile WASM)
            #[cfg(target_arch = "wasm32")]
            {
                if is_mobile {
                    if mx >= prompt_rect.0
                        && mx <= prompt_rect.0 + prompt_rect.2
                        && my >= prompt_rect.1
                        && my <= prompt_rect.1 + prompt_rect.3
                    {
                        self.content = crate::leaderboard::ask_player_name(&self.content);
                    }
                }
            }

            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = (prompt_rect, is_mobile);
            }
        }

        submitted
    }
}
