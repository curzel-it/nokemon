use raylib::math::{Rectangle, Vector2};
use serde::{Deserialize, Serialize};

use super::{rect::Rect, vector::Vector2d};

#[derive(Debug, Copy, Clone, Default, Serialize, Deserialize)]
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

    pub fn all_side(value: f32) -> Self {
        Self::new(value, value, value, value)
    }

    pub fn zero() -> Self {
        Insets::new(0.0, 0.0, 0.0, 0.0)
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
    pub fn from_vector(dv: Vector2d) -> Self {
        if dv.y  < 0.0 && dv.x == 0.0 { return Direction::Up; }
        if dv.y == 0.0 && dv.x  > 0.0 { return Direction::Right; }
        if dv.y  > 0.0 && dv.x == 0.0 { return Direction::Down; }
        if dv.y == 0.0 && dv.x  < 0.0 { return Direction::Left; }
        Direction::Unknown
    }
}

pub fn is_collision_trajectory(direction: &Vector2d, source: &Rect, destination: &Rect) -> bool {
    is_collision_trajectory_points(
        direction,
        &source.center(),
        &destination.center(),
    )
}

pub fn is_collision_trajectory_points(direction: &Vector2d, source: &Vector2d, destination: &Vector2d) -> bool {
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