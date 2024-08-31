use serde::{Deserialize, Serialize};

use super::vector::Vector2d;

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub w: u32,
    pub h: u32,
}

impl Rect {
    pub const fn new(x: u32, y: u32, w: u32, h: u32) -> Self {
        Rect { x, y, w, h }
    }

    pub fn from_origin(w: u32, h: u32) -> Self {
        Self::new(0, 0, w, h)
    }

    pub fn square_from_origin(size: u32) -> Self {
        Self::from_origin(size, size)
    }

    pub fn center(&self) -> Vector2d {
        Vector2d::new(
            self.x as f32 + self.w as f32 / 2.0, 
            self.y as f32 + self.h as f32 / 2.0
        )
    }

    pub fn center_in(&mut self, other: &Rect) {
        self.center_at(&other.center())
    }

    pub fn center_at(&mut self, point: &Vector2d) {
        self.x = (point.x - (self.w as f32 / 2.0)) as u32;
        self.y = (point.y - (self.h as f32 / 2.0)) as u32;
    }

    pub fn offset(&self, dx: i32, dy: i32) -> Self {
        Self::new(
            (self.x as i32 + dx).max(0) as u32, 
            (self.y as i32 + dy).max(0) as u32, 
            self.w, 
            self.h
        )
    }

    pub fn offset_x(&self, dx: i32) -> Self {
        self.offset(dx, 0)
    }

    pub fn offset_y(&self, dy: i32) -> Self {
        self.offset(0, dy)
    }

    pub fn is_around_and_pointed_at(&self, other: &Rect, direction: &Vector2d) -> bool {        
        if self.y == other.y + other.h && direction.y < 0.0 {
            return self.x >= other.x && self.x < other.x + other.w;
        }
        if self.y == other.y && direction.y > 0.0 {
            return self.x >= other.x && self.x < other.x + other.w;
        }
        if self.x == other.x + other.w && direction.x < 0.0 {
            return self.y > other.y && self.y < other.y + other.h;
        }
        if self.x == other.x.max(1) - 1 && direction.x > 0.0 {
            return self.y > other.y && self.y < other.y + other.h;
        }
        false
    }
    
    pub fn contains_or_touches_point(&self, x: u32, y: u32) -> bool {
        self.x <= x && x <= self.x + self.w && self.y <= y && y <= self.y + self.h
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_rect() {
        let rect = Rect::new(10, 20, 30, 40);
        assert_eq!(rect.x, 10);
        assert_eq!(rect.y, 20);
        assert_eq!(rect.w, 30);
        assert_eq!(rect.h, 40);
    }

    #[test]
    fn test_center_in() {
        let mut rect = Rect::new(0, 0, 10, 10);
        let outer_rect = Rect::new(10, 10, 20, 20);
        rect.center_in(&outer_rect);
        assert_eq!(rect.x, 15);
        assert_eq!(rect.y, 15);
    }
}
