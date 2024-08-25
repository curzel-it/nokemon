use uuid::Uuid;

use crate::{constants::TILE_SIZE, entities::hero::Hero, game_engine::{entity_body::EmbodiedEntity, world::World}};

impl World {
    pub fn setup(&mut self, source: &Uuid) {
        let mut entity = Hero::new();
        if let Some(teleporter_position) = self.find_teleporter_for_destination(source) {
            entity.body_mut().frame.x = teleporter_position.x;
            entity.body_mut().frame.y = teleporter_position.y;
        } else {
            entity.center_in(&self.bounds);
        }
        entity.snap_to_nearest_tile();
        entity.body_mut().frame.y -= TILE_SIZE;
        self.add_entity(Box::new(entity));
    }    
}

