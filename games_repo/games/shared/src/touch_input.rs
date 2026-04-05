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

/// Convert screen coordinates into board cell indices.
pub fn get_grid_coords(
    mx: f32,
    my: f32,
    offset_x: f32,
    offset_y: f32,
    board_size: f32,
    cell_size: f32,
    cols: usize,
    rows: usize,
) -> Option<(usize, usize)> {
    if cols == 0 || rows == 0 || board_size <= 0.0 || cell_size <= 0.0 {
        return None;
    }

    if mx < offset_x || mx >= offset_x + board_size || my < offset_y || my >= offset_y + board_size {
        return None;
    }

    let gx = ((mx - offset_x) / cell_size).floor() as usize;
    let gy = ((my - offset_y) / cell_size).floor() as usize;
    Some((gx.min(cols - 1), gy.min(rows - 1)))
}

/// Resolve keyboard direction input into a valid adjacent swap target.
pub fn keyboard_swap_target(
    sx: usize,
    sy: usize,
    cols: usize,
    rows: usize,
    up: bool,
    down: bool,
    left: bool,
    right: bool,
) -> Option<(usize, usize)> {
    if cols == 0 || rows == 0 || sx >= cols || sy >= rows {
        return None;
    }

    if up && sy > 0 {
        Some((sx, sy - 1))
    } else if down && sy + 1 < rows {
        Some((sx, sy + 1))
    } else if left && sx > 0 {
        Some((sx - 1, sy))
    } else if right && sx + 1 < cols {
        Some((sx + 1, sy))
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::{get_grid_coords, keyboard_swap_target};

    #[test]
    fn get_grid_coords_maps_inside_board() {
        let coords = get_grid_coords(25.0, 25.0, 0.0, 0.0, 80.0, 10.0, 8, 8);
        assert_eq!(coords, Some((2, 2)));
    }

    #[test]
    fn get_grid_coords_clamps_near_board_edge() {
        let coords = get_grid_coords(79.99, 79.99, 0.0, 0.0, 80.0, 10.0, 8, 8);
        assert_eq!(coords, Some((7, 7)));
    }

    #[test]
    fn get_grid_coords_rejects_exact_board_edge() {
        let coords = get_grid_coords(80.0, 80.0, 0.0, 0.0, 80.0, 10.0, 8, 8);
        assert_eq!(coords, None);
    }

    #[test]
    fn get_grid_coords_rejects_zero_sized_grid() {
        assert_eq!(get_grid_coords(1.0, 1.0, 0.0, 0.0, 10.0, 1.0, 0, 8), None);
        assert_eq!(get_grid_coords(1.0, 1.0, 0.0, 0.0, 10.0, 1.0, 8, 0), None);
        assert_eq!(get_grid_coords(1.0, 1.0, 0.0, 0.0, 0.0, 1.0, 8, 8), None);
        assert_eq!(get_grid_coords(1.0, 1.0, 0.0, 0.0, 10.0, 0.0, 8, 8), None);
    }

    #[test]
    fn get_grid_coords_clamps_when_board_is_larger_than_grid() {
        // board_size can be larger than cols*cell_size in responsive layouts.
        // In that case, touch coordinates still inside board bounds must clamp safely.
        let coords = get_grid_coords(95.0, 95.0, 0.0, 0.0, 100.0, 10.0, 8, 8);
        assert_eq!(coords, Some((7, 7)));
    }

    #[test]
    fn keyboard_swap_target_respects_bounds() {
        assert_eq!(keyboard_swap_target(0, 0, 8, 8, true, false, false, false), None);
        assert_eq!(keyboard_swap_target(0, 0, 8, 8, false, false, true, false), None);
        assert_eq!(keyboard_swap_target(7, 7, 8, 8, false, true, false, false), None);
        assert_eq!(keyboard_swap_target(7, 7, 8, 8, false, false, false, true), None);
    }

    #[test]
    fn keyboard_swap_target_returns_adjacent_cell() {
        assert_eq!(keyboard_swap_target(3, 3, 8, 8, true, false, false, false), Some((3, 2)));
        assert_eq!(keyboard_swap_target(3, 3, 8, 8, false, true, false, false), Some((3, 4)));
        assert_eq!(keyboard_swap_target(3, 3, 8, 8, false, false, true, false), Some((2, 3)));
        assert_eq!(keyboard_swap_target(3, 3, 8, 8, false, false, false, true), Some((4, 3)));
    }
}
