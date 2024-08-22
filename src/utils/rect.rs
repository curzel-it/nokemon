use serde::{Deserialize, Serialize};

use super::{geometry_utils::Insets, vector::Vector2d};

#[derive(Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub const fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
        Rect { x, y, w, h }
    }

    pub fn as_rr(&self) -> raylib::math::Rectangle {
        raylib::math::Rectangle::new(self.x, self.y, self.w, self.h)
    }

    pub fn from_origin(w: f32, h: f32) -> Self {
        Self::new(0.0, 0.0, w, h)
    }

    pub fn square_from_origin(size: f32) -> Self {
        Self::new(0.0, 0.0, size, size)
    }

    pub fn origin(&self) -> Vector2d {
        Vector2d::new(self.x, self.y)
    }

    pub fn center(&self) -> Vector2d {
        Vector2d::new(self.x + self.w / 2.0, self.y + self.h / 2.0)
    }

    pub fn center_in(&mut self, other: &Rect) {
        self.center_at(&other.center())
    }

    pub fn center_at(&mut self, point: &Vector2d) {
        self.x = point.x - self.w / 2.0;
        self.y = point.y - self.h / 2.0;
    }

    pub fn resize(&mut self, w: f32, h: f32) {
        self.w = w;
        self.h = h;
    }

    pub fn scaled(&self, value: f32) -> Self {
        Self::new(self.x * value, self.y * value, self.w * value, self.h * value)
    }

    pub fn inset_by(&self, value: f32) -> Self {
        self.inset(Insets::all_side(value))
    }

    pub fn inset(&self, insets: Insets) -> Self {
        Self::new(
            self.x + insets.left, 
            self.y + insets.top, 
            self.w - insets.left - insets.right, 
            self.h - insets.top - insets.bottom
        )
    }

    pub fn offset(&self, dx: f32, dy: f32) -> Self {
        Self::new(self.x + dx, self.y + dy, self.w, self.h)
    }

    pub fn offset_x(&self, dx: f32) -> Self {
        self.offset(dx, 0.0)
    }

    pub fn offset_y(&self, dy: f32) -> Self {
        self.offset(0.0, dy)
    }

    pub fn collides_with_rect(&self, other: &Rect) -> bool {
        let no_overlap = self.x + self.w <= other.x ||
                         other.x + other.w <= self.x ||
                         self.y + self.h <= other.y ||
                         other.y + other.h <= self.y;
        !no_overlap
    }

    pub fn collision_area_with_rect(&self, other: &Rect) -> Option<Rect> {
        if self.collides_with_rect(other) {
            let overlap_x = self.x.max(other.x);
            let overlap_y = self.y.max(other.y);
            let overlap_w = (self.x + self.w).min(other.x + other.w) - overlap_x;
            let overlap_h = (self.y + self.h).min(other.y + other.h) - overlap_y;
            Some(Rect::new(overlap_x, overlap_y, overlap_w, overlap_h))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_rect() {
        let rect = Rect::new(10.0, 20.0, 30.0, 40.0);
        assert_eq!(rect.x, 10.0);
        assert_eq!(rect.y, 20.0);
        assert_eq!(rect.w, 30.0);
        assert_eq!(rect.h, 40.0);
    }

    #[test]
    fn test_center_in() {
        let mut rect = Rect::new(0.0, 0.0, 10.0, 10.0);
        let outer_rect = Rect::new(10.0, 10.0, 20.0, 20.0);
        rect.center_in(&outer_rect);
        assert_eq!(rect.x, 15.0);
        assert_eq!(rect.y, 15.0);
    }

    #[test]
    fn test_inset() {
        let rect = Rect::new(10.0, 10.0, 100.0, 100.0);
        let insets = Insets { top: 10.0, bottom: 20.0, left: 5.0, right: 15.0 };
        let inset_rect = rect.inset(insets);
        assert_eq!(inset_rect.x, 15.0);
        assert_eq!(inset_rect.y, 20.0);
        assert_eq!(inset_rect.w, 80.0);
        assert_eq!(inset_rect.h, 70.0);
    }

    #[test]
    fn test_collides_with_rect() {
        let rect1 = Rect::new(0.0, 0.0, 10.0, 10.0);
        let rect2 = Rect::new(5.0, 5.0, 10.0, 10.0);
        assert!(rect1.collides_with_rect(&rect2));
        
        let rect3 = Rect::new(15.0, 15.0, 5.0, 5.0);
        assert!(!rect1.collides_with_rect(&rect3));
    }

    #[test]
    fn test_collision_area_with_rect() {
        let rect1 = Rect::new(0.0, 0.0, 10.0, 10.0);
        let rect2 = Rect::new(5.0, 5.0, 10.0, 10.0);
        let collision_area = rect1.collision_area_with_rect(&rect2).unwrap();
        assert_eq!(collision_area.x, 5.0);
        assert_eq!(collision_area.y, 5.0);
        assert_eq!(collision_area.w, 5.0);
        assert_eq!(collision_area.h, 5.0);

        let rect3 = Rect::new(20.0, 20.0, 5.0, 5.0);
        assert!(rect1.collision_area_with_rect(&rect3).is_none());
    }

    #[test]
    fn test_scaled() {
        let rect = Rect::new(10.0, 20.0, 30.0, 40.0);
        let scaled_rect = rect.scaled(2.0);
        assert_eq!(scaled_rect.x, 20.0);
        assert_eq!(scaled_rect.y, 40.0);
        assert_eq!(scaled_rect.w, 60.0);
        assert_eq!(scaled_rect.h, 80.0);
    }
}
