use raylib::math::{Rectangle, Vector2};

#[derive(Debug, Copy, Clone, Default)]
pub struct Insets {
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub left: i32
}

impl Insets {
    pub fn new(top: i32, right: i32, bottom: i32, left: i32) -> Self {
        Insets { 
            top, 
            right, 
            bottom, 
            left 
        }
    }

    pub fn zero() -> Self {
        Insets::new(0, 0, 0, 0)
    }

    pub fn apply_to_rect(&self, rect: &IntRect) -> IntRect {
        IntRect::new(
            rect.x + self.left, 
            rect.y + self.top, 
            (rect.width as i32 - self.left - self.right).max(0) as u32, 
            (rect.height as i32 - self.top - self.bottom).max(0) as u32
        )
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct IntRect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

impl IntRect {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self {
        IntRect {
            x, y, width, height
        }
    }

    pub fn zero() -> Self {
        Self::new(0, 0, 0, 0)
    }

    pub fn square(x: i32, y: i32, size: i32) -> Self {
        Self::new(x, y, size, size)
    }

    pub fn offset(&mut self, dx: i32, dy: i32) {
        self.x += dx;
        self.y += dy;
    }

    pub fn center(&self) -> (i32, i32) {
        (self.x + self.width as i32 / 2, self.y + self.height as i32 / 2)
    }

    pub fn center_at(&mut self, x: i32, y: i32) {
        self.x = x - self.width as i32 / 2;
        self.y = y - self.height as i32 / 2;
    }

    pub fn resize(&mut self, width: i32, height: i32) {
        self.width = width;
        self.height = height;
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

pub fn center_of_rec(frame: &Rectangle) -> Vector2 {
    Vector2::new(
        frame.x + frame.width / 2.0,
        frame.y + frame.height / 2.0
    )
}

pub fn is_collision_trajectory(direction: &Vector2, source: &Rectangle, destination: &Rectangle) -> bool {
    is_collision_trajectory_points(
        direction,
        &center_of_rec(source),
        &center_of_rec(destination),
    )
}

pub fn is_collision_trajectory_points(direction: &Vector2, source: &Vector2, destination: &Vector2) -> bool {
    if direction.x > 0.0 && destination.x > source.x {
        return true;
    }
    if direction.x < 0.0 && destination.x < source.x {
        return true;
    }
    if direction.y > 0.0 && destination.y > source.y {
        return true;
    }
    if direction.y < 0.0 && destination.y < source.y {
        return true;
    }
    false
}