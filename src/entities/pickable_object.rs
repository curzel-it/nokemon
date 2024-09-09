use crate::game_engine::{entity::Entity, state_updates::WorldStateUpdate, world::World};

impl Entity {
    pub fn update_pickable_object(&mut self, _: &World, _: f32) -> Vec<WorldStateUpdate> {        
        vec![]
    }
}