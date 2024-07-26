use raylib::math::Rectangle;

pub struct RenderedItem {
    pub id: u32,
    pub sprite_path: String,
    pub frame: Rectangle,
    pub is_flipped: bool,
    pub z_rotation: f32,
}

impl RenderedItem {
    pub fn new(id: u32, sprite_path: String, frame: Rectangle, is_flipped: bool, z_rotation: f32) -> Self {
        RenderedItem {
            id,
            sprite_path,
            frame,
            is_flipped,
            z_rotation,
        }
    }
}
