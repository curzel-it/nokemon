use serde::{Deserialize, Serialize};

use crate::game_engine::{entity::Entity, state_updates::WorldStateUpdate, world::World};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum PickableObject {
    Key
}

impl Entity {
    pub fn update_pickable_object(&mut self, _: &World, _: f32) -> Vec<WorldStateUpdate> {        
        vec![]
    }
}