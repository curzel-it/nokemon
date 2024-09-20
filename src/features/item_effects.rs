use crate::{entities::species::SpeciesId, game_engine::world::World};

impl World {
    pub fn use_item(&mut self, species_id: SpeciesId) {
        match species_id {
            _ => println!("Don't know how to use {}", species_id)
        }
    }
}