use raylib::math::Rectangle;

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
    Left
}