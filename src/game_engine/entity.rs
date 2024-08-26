
use std::any::Any;

use crate::utils::{rect::Rect, vector::Vector2d};

use super::{entity_body::EmbodiedEntity, world::World, state_updates::WorldStateUpdate};

pub trait Entity: EmbodiedEntity {
    fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate>;
    fn texture_source_rect(&self) -> Rect;  
    fn sprite_sheet(&self) -> u32;
    fn as_any(&self) -> &dyn Any;
}

impl PartialEq for dyn Entity {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

#[derive(Debug, Copy, Clone)]
pub struct EntityProps {
    pub direction: Vector2d,
    pub frame: Rect,
    pub hittable_frame: Rect,
}

impl Default for EntityProps {
    fn default() -> Self {
        Self { 
            direction: Default::default(), 
            frame: Rect::square_from_origin(1), 
            hittable_frame: Rect::square_from_origin(1) 
        }
    }
}