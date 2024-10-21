use std::ops::Add;

use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, Serialize, Deserialize)]
pub struct Vector2d {
    pub x: f32,
    pub y: f32,
}

impl Vector2d {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn as_rv(&self) -> raylib::math::Vector2 {
        raylib::math::Vector2::new(self.x, self.y)
    }

    pub const fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn scaled(&self, value: f32) -> Self {
        Self::new(self.x * value, self.y * value)
    }
}

impl Add for Vector2d {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}