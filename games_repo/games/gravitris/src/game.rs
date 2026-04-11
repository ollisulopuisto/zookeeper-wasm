use macroquad::prelude::*;
use serde::{Deserialize, Serialize};

pub const COLS: usize = 10;
pub const ROWS: usize = 20;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PieceType {
    I, O, T, S, Z, J, L
}

impl PieceType {
    pub fn color(&self) -> Color {
        match self {
            PieceType::I => SKYBLUE,
            PieceType::O => YELLOW,
            PieceType::T => PURPLE,
            PieceType::S => GREEN,
            PieceType::Z => RED,
            PieceType::J => BLUE,
            PieceType::L => ORANGE,
        }
    }

    pub fn shape(&self, rotation: usize) -> Vec<(i32, i32)> {
        let base_shape = match self {
            PieceType::I => vec![(0, 1), (1, 1), (2, 1), (3, 1)],
            PieceType::O => vec![(1, 0), (2, 0), (1, 1), (2, 1)],
            PieceType::T => vec![(1, 0), (0, 1), (1, 1), (2, 1)],
            PieceType::S => vec![(1, 0), (2, 0), (0, 1), (1, 1)],
            PieceType::Z => vec![(0, 0), (1, 0), (1, 1), (2, 1)],
            PieceType::J => vec![(0, 0), (0, 1), (1, 1), (2, 1)],
            PieceType::L => vec![(2, 0), (0, 1), (1, 1), (2, 1)],
        };

        let mut current_shape = base_shape;
        for _ in 0..(rotation % 4) {
            current_shape = current_shape.into_iter().map(|(x, y)| (1 - (y - 1), x)).collect();
        }
        current_shape
    }
}

pub struct ActivePiece {
    pub piece_type: PieceType,
    pub x: i32,
    pub y: i32,
    pub rotation: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct GravityWell {
    pub x: i32,
    pub y: i32,
    pub strength: f32,
}

pub struct Board {
    pub grid: [[Option<PieceType>; COLS]; ROWS],
    pub active: Option<ActivePiece>,
    pub wells: Vec<GravityWell>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            grid: [[None; COLS]; ROWS],
            active: None,
            wells: Vec::new(),
        }
    }

    pub fn spawn_piece(&mut self) -> bool {
        let piece_types = [
            PieceType::I, PieceType::O, PieceType::T,
            PieceType::S, PieceType::Z, PieceType::J, PieceType::L
        ];
        let piece_type = piece_types[quad_rand::gen_range(0, piece_types.len())];
        let x = COLS as i32 / 2 - 2;
        let y = -1;
        
        if !self.collides(piece_type, x, y, 0) {
            self.active = Some(ActivePiece {
                piece_type,
                x,
                y,
                rotation: 0,
            });
            true
        } else {
            false
        }
    }

    pub fn apply_gravity_wells(&mut self) -> bool {
        if let Some(active) = &self.active {
            let mut total_dx = 0.0;
            let mut total_dy = 0.0;

            for well in &self.wells {
                let dx = well.x as f32 - active.x as f32;
                let dy = well.y as f32 - active.y as f32;
                let dist_sq = dx * dx + dy * dy;
                if dist_sq > 0.1 {
                    let force = well.strength / dist_sq;
                    total_dx += (dx / dist_sq.sqrt()) * force;
                    total_dy += (dy / dist_sq.sqrt()) * force;
                }
            }

            let dx = if total_dx > 0.5 { 1 } else if total_dx < -0.5 { -1 } else { 0 };
            let dy = if total_dy > 0.5 { 1 } else if total_dy < -0.5 { -1 } else { 0 };

            if dx != 0 || dy != 0 {
                return self.move_piece(dx, dy);
            }
        }
        false
    }

    pub fn draw(&self, offset_x: f32, offset_y: f32, cell_size: f32) {
        // Draw grid background
        draw_rectangle(offset_x, offset_y, COLS as f32 * cell_size, ROWS as f32 * cell_size, Color::new(0.1, 0.1, 0.1, 1.0));
        
        // Draw grid lines
        for x in 0..=COLS {
            draw_line(offset_x + x as f32 * cell_size, offset_y, offset_x + x as f32 * cell_size, offset_y + ROWS as f32 * cell_size, 1.0, Color::new(0.2, 0.2, 0.2, 1.0));
        }
        for y in 0..=ROWS {
            draw_line(offset_x, offset_y + y as f32 * cell_size, offset_x + COLS as f32 * cell_size, offset_y + y as f32 * cell_size, 1.0, Color::new(0.2, 0.2, 0.2, 1.0));
        }

        // Draw placed blocks
        for y in 0..ROWS {
            for x in 0..COLS {
                if let Some(piece_type) = self.grid[y][x] {
                    draw_rectangle(offset_x + x as f32 * cell_size, offset_y + y as f32 * cell_size, cell_size, cell_size, piece_type.color());
                    draw_rectangle_lines(offset_x + x as f32 * cell_size, offset_y + y as f32 * cell_size, cell_size, cell_size, 2.0, BLACK);
                }
            }
        }

        // Draw active piece
        if let Some(active) = &self.active {
            for (dx, dy) in active.piece_type.shape(active.rotation) {
                let gx = active.x + dx;
                let gy = active.y + dy;
                if gy >= 0 {
                    draw_rectangle(offset_x + gx as f32 * cell_size, offset_y + gy as f32 * cell_size, cell_size, cell_size, active.piece_type.color());
                    draw_rectangle_lines(offset_x + gx as f32 * cell_size, offset_y + gy as f32 * cell_size, cell_size, cell_size, 2.0, WHITE);
                }
            }
        }

        // Draw gravity wells
        for well in &self.wells {
            let cx = offset_x + well.x as f32 * cell_size + cell_size * 0.5;
            let cy = offset_y + well.y as f32 * cell_size + cell_size * 0.5;
            let pulse = (get_time() * 5.0).sin() as f32 * 0.2 + 0.8;
            draw_circle(cx, cy, cell_size * 0.4 * pulse, Color::new(1.0, 0.0, 1.0, 0.5));
            draw_circle_lines(cx, cy, cell_size * 0.5 * pulse, 2.0, MAGENTA);
        }
    }

    pub fn move_piece(&mut self, dx: i32, dy: i32) -> bool {
        if let Some(active) = &self.active {
            let nx = active.x + dx;
            let ny = active.y + dy;
            if !self.collides(active.piece_type, nx, ny, active.rotation) {
                if let Some(active) = &mut self.active {
                    active.x = nx;
                    active.y = ny;
                }
                return true;
            }
        }
        false
    }

    pub fn rotate_piece(&mut self) -> bool {
        if let Some(active) = &self.active {
            let nr = (active.rotation + 1) % 4;
            if !self.collides(active.piece_type, active.x, active.y, nr) {
                if let Some(active) = &mut self.active {
                    active.rotation = nr;
                }
                return true;
            }
        }
        false
    }

    pub fn lock_piece(&mut self) {
        if let Some(active) = self.active.take() {
            for (dx, dy) in active.piece_type.shape(active.rotation) {
                let gx = active.x + dx;
                let gy = active.y + dy;
                if gx >= 0 && gx < COLS as i32 && gy >= 0 && gy < ROWS as i32 {
                    self.grid[gy as usize][gx as usize] = Some(active.piece_type);
                }
            }
        }
    }

    pub fn collides(&self, piece_type: PieceType, x: i32, y: i32, rotation: usize) -> bool {
        for (dx, dy) in piece_type.shape(rotation) {
            let gx = x + dx;
            let gy = y + dy;
            if gx < 0 || gx >= COLS as i32 || gy >= ROWS as i32 {
                return true;
            }
            if gy >= 0 && self.grid[gy as usize][gx as usize].is_some() {
                return true;
            }
        }
        false
    }

    pub fn clear_lines(&mut self) -> usize {
        let mut cleared = 0;
        let mut y = ROWS - 1;
        while y < ROWS {
            let mut full = true;
            for x in 0..COLS {
                if self.grid[y][x].is_none() {
                    full = false;
                    break;
                }
            }

            if full {
                cleared += 1;
                for row in (1..=y).rev() {
                    for x in 0..COLS {
                        self.grid[row][x] = self.grid[row - 1][x];
                    }
                }
                for x in 0..COLS {
                    self.grid[0][x] = None;
                }
            } else {
                if y == 0 { break; }
                y -= 1;
            }
        }
        cleared
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_move_piece() {
        let mut board = Board::new();
        board.active = Some(ActivePiece {
            piece_type: PieceType::O,
            x: 4,
            y: 0,
            rotation: 0,
        });
        
        let moved = board.move_piece(1, 0);
        assert!(moved);
        let active = board.active.as_ref().unwrap();
        assert_eq!(active.x, 5);
    }

    #[test]
    fn test_rotate_piece() {
        let mut board = Board::new();
        board.active = Some(ActivePiece {
            piece_type: PieceType::T,
            x: 4,
            y: 5,
            rotation: 0,
        });
        
        let rotated = board.rotate_piece();
        assert!(rotated);
        let active = board.active.as_ref().unwrap();
        assert_eq!(active.rotation, 1);
    }

    #[test]
    fn test_clear_lines() {
        let mut board = Board::new();
        for x in 0..COLS {
            board.grid[ROWS - 1][x] = Some(PieceType::O);
        }
        
        let cleared = board.clear_lines();
        assert_eq!(cleared, 1);
        for x in 0..COLS {
            assert!(board.grid[ROWS - 1][x].is_none());
        }
    }

    #[test]
    fn test_gravity_well_pull() {
        let mut board = Board::new();
        board.active = Some(ActivePiece {
            piece_type: PieceType::O,
            x: 0,
            y: 5,
            rotation: 0,
        });
        board.wells.push(GravityWell { x: 9, y: 5, strength: 50.0 });
        
        let pulled = board.apply_gravity_wells();
        assert!(pulled);
        let active = board.active.as_ref().unwrap();
        // Should move towards the well (at x=9)
        assert!(active.x > 0);
    }
}
