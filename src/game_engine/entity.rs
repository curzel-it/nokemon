
use crate::utils::{rect::Rect, vector::Vector2d};

use super::{entity_body::EmbodiedEntity, world::World, state_updates::WorldStateUpdate};

pub trait Entity: EmbodiedEntity {
    fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate>;
    fn texture_source_rect(&self) -> Rect;  
    fn sprite_sheet_path(&self) -> &str;
}

impl PartialEq for dyn Entity {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct EntityProps {
    pub direction: Vector2d,
    pub frame: Rect,
    pub speed: f32,
}

impl EntityProps {
    pub fn position(&self) -> Vector2d {
        self.frame.origin()
    }

    pub fn center(&self) -> Vector2d {
        self.frame.center()
    }
}