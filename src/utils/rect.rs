use serde::{Deserialize, Serialize};

use super::{geometry_utils::Insets, vector::Vector2d};

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

    pub fn as_rr(&self) -> raylib::math::Rectangle {
        raylib::math::Rectangle::new(self.x as f32, self.y as f32, self.w as f32, self.h as f32)
    }

    pub fn from_origin(w: u32, h: u32) -> Self {
        Self::new(0, 0, w, h)
    }

    pub fn zero() -> Self {
        Self::square_from_origin(0)
    }

    pub fn square_from_origin(size: u32) -> Self {
        Self::new(0, 0, size, size)
    }

    pub fn origin(&self) -> Vector2d {
        Vector2d::new(self.x as f32, self.y as f32)
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

    pub fn resize(&mut self, w: u32, h: u32) {
        self.w = w;
        self.h = h;
    }

    pub fn scaled(&self, value: u32) -> Self {
        Self::new(self.x * value, self.y * value, self.w * value, self.h * value)
    }
/*
    pub fn inset_by(&self, value: u32) -> Self {
        self.inset(Insets::all_side(value))
    }

    pub fn inset(&self, insets: Insets) -> Self {
        Self::new(
            self.x + insets.left, 
            self.y + insets.top, 
            self.w - insets.left - insets.right, 
            self.h - insets.top - insets.bottom
        )
    } */

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

    pub fn collides_with_rect(&self, other: &Rect) -> bool {
        self.x < other.x + other.w &&
        other.x < self.x + self.w &&
        self.y < other.y + other.h &&
        other.y < self.y + self.h
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
/*
    #[test]
    fn test_inset() {
        let rect = Rect::new(10, 10, 100, 100);
        let insets = Insets { top: 10, bottom: 20, left: 5, right: 15 };
        let inset_rect = rect.inset(insets);
        assert_eq!(inset_rect.x, 15);
        assert_eq!(inset_rect.y, 20);
        assert_eq!(inset_rect.w, 80);
        assert_eq!(inset_rect.h, 70);
    } */

    #[test]
    fn test_collides_with_rect() {
        let rect1 = Rect::new(0, 0, 10, 10);
        let rect2 = Rect::new(5, 5, 10, 10);
        assert!(rect1.collides_with_rect(&rect2));
        
        let rect3 = Rect::new(15, 15, 5, 5);
        assert!(!rect1.collides_with_rect(&rect3));
    }

    #[test]
    fn test_scaled() {
        let rect = Rect::new(10, 20, 30, 40);
        let scaled_rect = rect.scaled(2);
        assert_eq!(scaled_rect.x, 20);
        assert_eq!(scaled_rect.y, 40);
        assert_eq!(scaled_rect.w, 60);
        assert_eq!(scaled_rect.h, 80);
    }
}
