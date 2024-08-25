use super::vector::Vector2d;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
    Unknown
}

impl Direction {
    pub fn from_vector(dv: Vector2d) -> Self {
        if dv.y  < 0.0 && dv.x == 0.0 { return Direction::Up; }
        if dv.y == 0.0 && dv.x  > 0.0 { return Direction::Right; }
        if dv.y  > 0.0 && dv.x == 0.0 { return Direction::Down; }
        if dv.y == 0.0 && dv.x  < 0.0 { return Direction::Left; }
        Direction::Unknown
    }
}