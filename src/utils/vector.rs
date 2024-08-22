pub struct Vector2d {
    pub x: f32,
    pub y: f32,
}

impl Vector2d {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn as_rv(&self) -> raylib::math::Vector2 {
        raylib::math::Vector2::new(self.x, self.y)
    }

    pub fn square(x: f32) -> Self {
        Self::new(x, x)
    }

    pub fn zero() -> Self {
        Self::new(0.0, 0.0)
    }

    pub fn one() -> Self {
        Self::new(1.0, 1.0)
    }

    pub fn scaled(&self, value: f32) -> Self {
        Self::new(self.x * value, self.y * value)
    }
}