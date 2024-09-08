
use std::any::Any;

use crate::utils::{directions::Direction, rect::Rect};

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
    pub direction: Direction,
    pub frame: Rect,
    pub speed: f32,
    pub hittable_frame: Rect,
}

impl Default for EntityProps {
    fn default() -> Self {
        Self { 
            direction: Default::default(), 
            frame: Rect::square_from_origin(1), 
            speed: 0.0,
            hittable_frame: Rect::square_from_origin(1) 
        }
    }
}

pub trait EntityConvertible {
    fn make_entity(&self) -> Box<dyn Entity>;
}