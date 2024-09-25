use serde_repr::{Serialize_repr, Deserialize_repr};
use super::{rect::Rect, vector::Vector2d};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
#[derive(Default)]
pub enum Direction {
    Up = 0,
    Down = 1,
    Right = 2,
    Left = 3,
    #[default]
    Unknown = 4,
}

impl Direction {
    pub fn as_vector(&self) -> Vector2d {
        let (col, row) = self.as_col_row_offset();
        Vector2d::new(col as f32, row as f32)
    }

    pub fn as_col_row_offset(&self) -> (i32, i32) {
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

    pub fn opposite(&self) -> Direction {
        match self {
            Direction::Up => Direction::Down,
            Direction::Right => Direction::Left,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Unknown => Direction::Unknown,
        }
    }

    pub fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Unknown => Direction::Unknown,
        }
    }

    pub fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Unknown => Direction::Unknown,
        }
    }
}

pub fn direction_between_rects(source: &Rect, other: &Rect) -> Direction {
    if source.x > other.x { return Direction::Left }
    if source.x < other.x { return Direction::Right }
    if source.y > other.y { return Direction::Up }
    if source.y < other.y { return Direction::Down }
    Direction::Unknown
}