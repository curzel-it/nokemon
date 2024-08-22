use super::geometry_utils::Insets;

pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Rect {
    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Self {
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
}