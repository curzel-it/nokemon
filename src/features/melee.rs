use crate::game_engine::{entity::Entity, state_updates::WorldStateUpdate, world::World};

impl Entity {
    pub fn handle_melee_attack(&mut self, world: &World, time_since_last_update: f32) -> Vec<WorldStateUpdate> {
        vec![]
    }
}