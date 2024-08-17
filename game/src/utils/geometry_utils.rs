use raylib::math::{Rectangle, Vector2};

#[derive(Debug, Copy, Clone, Default)]
pub struct Insets {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32
}

impl Insets {
    pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Insets { 
            top, 
            right, 
            bottom, 
            left 
        }
    }

    pub fn zero() -> Self {
        Insets::new(0.0, 0.0, 0.0, 0.0)
    }

    pub fn apply_to_rect(&self, rect: &Rectangle) -> Rectangle {
        Rectangle::new(
            rect.x + self.left, 
            rect.y + self.top, 
            rect.width - self.left - self.right, 
            rect.height - self.top - self.bottom
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Right,
    Left,
    Unknown
}

impl Direction {
    pub fn from_vector(dv: Vector2) -> Self {
        if dv.y  < 0.0 && dv.x == 0.0 { return Direction::Up; }
        if dv.y == 0.0 && dv.x  > 0.0 { return Direction::Right; }
        if dv.y  > 0.0 && dv.x == 0.0 { return Direction::Down; }
        if dv.y == 0.0 && dv.x  < 0.0 { return Direction::Left; }
        Direction::Unknown
    }
}

pub trait Scalable {
    fn scaled(&self, value: f32) -> Self;
}

impl Scalable for Rectangle {
    fn scaled(&self, value: f32) -> Self {
        Rectangle::new(
            self.x * value, 
            self.y * value, 
            self.width * value, 
            self.height * value
        )
    }
}
