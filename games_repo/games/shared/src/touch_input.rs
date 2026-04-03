pub fn get_grid_coords(
    mx: f32,
    my: f32,
    ox: f32,
    oy: f32,
    size: f32,
    cell: f32,
    cols: usize,
    rows: usize,
) -> Option<(usize, usize)> {
    let buffer = 4.0;
    if mx < ox - buffer || mx >= ox + size + buffer || my < oy - buffer || my >= oy + size + buffer {
        return None;
    }
    if cols == 0 || rows == 0 {
        return None;
    }

    let gx = ((mx - ox) / cell).floor() as i32;
    let gy = ((my - oy) / cell).floor() as i32;
    let gx = gx.clamp(0, (cols - 1) as i32) as usize;
    let gy = gy.clamp(0, (rows - 1) as i32) as usize;
    Some((gx, gy))
}

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
    fn maps_center_and_clamps_edges() {
        assert_eq!(get_grid_coords(50.0, 50.0, 0.0, 0.0, 100.0, 12.5, 8, 8), Some((4, 4)));
        assert_eq!(get_grid_coords(-2.0, -2.0, 0.0, 0.0, 100.0, 12.5, 8, 8), Some((0, 0)));
        assert_eq!(get_grid_coords(200.0, 200.0, 0.0, 0.0, 100.0, 12.5, 8, 8), None);
    }

    #[test]
    fn returns_none_for_zero_sized_grid() {
        assert_eq!(get_grid_coords(5.0, 5.0, 0.0, 0.0, 100.0, 10.0, 0, 8), None);
        assert_eq!(get_grid_coords(5.0, 5.0, 0.0, 0.0, 100.0, 10.0, 8, 0), None);
    }

    #[test]
    fn keyboard_target_respects_bounds() {
        assert_eq!(keyboard_swap_target(0, 0, 8, 8, true, false, false, false), None);
        assert_eq!(keyboard_swap_target(0, 0, 8, 8, false, true, false, false), Some((0, 1)));
        assert_eq!(keyboard_swap_target(7, 7, 8, 8, false, false, false, true), None);
        assert_eq!(keyboard_swap_target(7, 7, 8, 8, false, false, true, false), Some((6, 7)));
    }
}
