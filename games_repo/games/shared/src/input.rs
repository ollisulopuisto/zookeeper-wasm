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
