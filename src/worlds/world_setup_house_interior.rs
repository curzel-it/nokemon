use crate::{entities::{hero::Hero, teleporter::Teleporter}, game_engine::{entity_body::EmbodiedEntity, world::World}};

pub fn world_setup_house_interior(world: &mut World) {
    add_teleporter(world);
    add_hero(world);
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
