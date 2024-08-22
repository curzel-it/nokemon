use crate::{entities::{building::{Building, BuildingType}, creep_spawn_point::CreepSpawnPoint, hero::Hero, teleporter::Teleporter, tower::Tower}, game_engine::{entity_body::EmbodiedEntity, world::World}};

use super::constants::LEVEL_ID_HOUSE_INTERIOR;

pub fn level_setup_demo(world: &mut World) {
    // add_creep_spawn_point(world);
    // add_tower(world);
    // add_house(world);
    add_hero(world);
}

fn add_house(world: &mut World) -> u32 {
    let mut entity = Building::new(BuildingType::House, LEVEL_ID_HOUSE_INTERIOR);
    entity.center_in(&world.bounds);
    entity.body_mut().frame.x -= 150.0;
    entity.body_mut().frame.y -= 50.0;
    entity.snap_to_nearest_tile();
    world.add_entity(Box::new(entity))
}

fn add_teleporter(world: &mut World) -> u32 {
    let mut entity = Teleporter::new();
    entity.center_in(&world.bounds);
    entity.body_mut().frame.x -= 50.0;
    entity.body_mut().frame.y -= 50.0;
    entity.snap_to_nearest_tile();
    world.add_entity(Box::new(entity))
}

fn add_creep_spawn_point(world: &mut World) -> u32 {
    let mut entity = CreepSpawnPoint::new();
    entity.center_in(&world.bounds);
    entity.body_mut().frame.x -= 80.0;
    entity.body_mut().frame.y -= 120.0;
    entity.snap_to_nearest_tile();
    world.add_entity(Box::new(entity))
}

fn add_tower(world: &mut World) -> u32 {
    let mut entity = Tower::new();
    entity.center_in(&world.bounds);
    entity.body_mut().frame.x -= 100.0;
    entity.body_mut().frame.y += 100.0;
    entity.snap_to_nearest_tile();
    world.add_entity(Box::new(entity))
}

fn add_hero(world: &mut World) -> u32 {
    let mut entity = Hero::new();
    entity.center_in(&world.bounds);
    entity.snap_to_nearest_tile();
    world.add_entity(Box::new(entity))
}
