
use raylib::math::{Rectangle, Vector2};

use super::{entity_body::EmbodiedEntity, world::World, state_updates::WorldStateUpdate};

pub trait Entity: EmbodiedEntity {
    fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate>;
    fn texture_source_rect(&self) -> Rectangle;  
    fn sprite_sheet_path(&self) -> &str;
}

impl PartialEq for dyn Entity {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct EntityProps {
    pub direction: Vector2,
    pub frame: Rectangle,
    pub speed: f32,
}

impl EntityProps {
    pub fn position(&self) -> Vector2 {
        Vector2::new(self.frame.x, self.frame.y)
    }

    pub fn center(&self) -> Vector2 {
        Vector2::new(
            self.frame.x + self.frame.width / 2.0, 
            self.frame.y + self.frame.height / 2.0
        )
    }
}