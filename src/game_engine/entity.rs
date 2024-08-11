
use super::{entity_body::EmbodiedEntity, world::World, world_state_update::WorldStateUpdate};

pub trait Entity: EmbodiedEntity {
    fn update(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate>;
}

impl PartialEq for dyn Entity {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}