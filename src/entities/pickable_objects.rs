use serde::{Deserialize, Serialize};

use crate::game_engine::{entity::Entity, state_updates::WorldStateUpdate, world::World};

use super::{known_species::SPECIES_KEY, species::SpeciesConvertible};

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum PickableObject {
    Key
}

impl SpeciesConvertible for PickableObject {
    fn get_species_id(&self) -> u32 {
        match self {
            PickableObject::Key => SPECIES_KEY,
        }
    }
}

impl Entity {
    pub fn update_pickable_object(&mut self, _: &World, _: f32) -> Vec<WorldStateUpdate> {        
        vec![]
    }
}