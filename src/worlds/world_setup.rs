use uuid::Uuid;

use crate::{entities::hero::Hero, game_engine::{entity_body::EmbodiedEntity, world::World}};

impl World {
    pub fn setup(&mut self) {
        self.add_hero();
    }

    fn add_hero(&mut self) -> Uuid {
        let mut entity = Hero::new();
        entity.center_in(&self.bounds);
        entity.snap_to_nearest_tile();
        self.add_entity(Box::new(entity))
    }    
}

