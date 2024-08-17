
use raylib::math::Rectangle;

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