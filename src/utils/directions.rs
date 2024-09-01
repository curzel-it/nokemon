use serde_repr::{Serialize_repr, Deserialize_repr};
use super::vector::Vector2d;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Right = 2,
    Left = 3,
    Unknown = 4,
}

impl Direction {
    pub fn as_vector(&self) -> Vector2d {
        let (row, col) = self.as_row_col_offset();
        Vector2d::new(col as f32, row as f32)
    }

    pub fn as_row_col_offset(&self) -> (i32, i32) {
        match self {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Unknown => (0, 0),
        }  
    }

    pub fn from_data(up: bool, right: bool, down: bool, left: bool) -> Self {
        if up { return Direction::Up }
        if right { return Direction::Right }
        if down { return Direction::Down }
        if left { return Direction::Left }
        Direction::Unknown
    }
}

impl Default for Direction {
    fn default() -> Self {
        Direction::Unknown
    }
}