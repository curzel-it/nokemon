use crate::{entities::{hero::Hero, teleporter::Teleporter}, game_engine::{entity_body::EmbodiedEntity, world::World}};

use super::constants::WORLD_ID_DEMO;

impl World {
    pub fn setup(&mut self) {
        match self.id {
            WORLD_ID_DEMO => {
                add_hero(self);
            },
            _ => {
                add_teleporter(self);
                add_hero(self);
            }
        }
    }
}

fn add_teleporter(world: &mut World) -> u32 {
    let mut entity = Teleporter::new();
    entity.center_in(&world.bounds);
    entity.body_mut().frame.x -= 50.0;
    entity.body_mut().frame.y -= 50.0;
    entity.snap_to_nearest_tile();
    world.add_entity(Box::new(entity))
}

fn add_hero(world: &mut World) -> u32 {
    let mut entity = Hero::new();
    entity.center_in(&world.bounds);
    entity.snap_to_nearest_tile();
    world.add_entity(Box::new(entity))
}
