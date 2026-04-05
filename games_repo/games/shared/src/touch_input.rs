/// Logic for handling touch-based pause/resume interactions.
///
/// Ensures that tapping to resume only occurs if the tap isn't on a UI button.
pub fn is_tap_valid_resume(mx: f32, my: f32, ui_buttons: &[(f32, f32, f32, f32)]) -> bool {
    for (x, y, w, h) in ui_buttons {
        if mx >= *x && mx <= *x + *w && my >= *y && my <= *y + *h {
            return false;
        }
    }
    true
}
