use crate::{game_engine::world::World, worlds::constants::{WORLDS_PATH, WORLD_DEMO_WORLD, WORLD_ID_HOUSE_INTERIOR}};

use super::{world_setup_demo::world_setup_demo, world_setup_house_interior::world_setup_house_interior};

pub fn world_tiles_paths() -> Vec<WorldTilePaths> {
    all_worlds()
        .into_iter()
        .map(|id| {
            WorldTilePaths {
                world_id: id,
                biome_map_image: world_biome_image_path(id),
                biome_map_binary: world_biome_binary_path(id),
                constructions_map_image: world_constructions_image_path(id),
                constructions_map_binary: world_constructions_binary_path(id),
            }
        })
        .collect()
}

pub fn setup_world(world: &mut World) {
    match world.world_id {
        WORLD_ID_HOUSE_INTERIOR => world_setup_house_interior(world),
        _ => world_setup_demo(world)
    }
}

fn all_worlds() -> Vec<u32> {
    vec![
        WORLD_DEMO_WORLD,
        WORLD_ID_HOUSE_INTERIOR,
    ]
}

fn world_name_by_id(id: u32) -> String {
    let name = match id {
        WORLD_DEMO_WORLD => "world",
        WORLD_ID_HOUSE_INTERIOR => "house_interior",
        _ => "unknown"
    };
    name.to_owned()
}

fn world_file(id: u32, name: &str) -> String {
    format!("{}/{}_{}", WORLDS_PATH, world_name_by_id(id), name)
}

pub fn world_biome_image_path(id: u32) -> String {
    world_file(id, "biome.png")
}

pub fn world_biome_binary_path(id: u32) -> String {
    world_file(id, "biome.bin")
}

pub fn world_constructions_image_path(id: u32) -> String {
    world_file(id, "constructions.png")
}

pub fn world_constructions_binary_path(id: u32) -> String {
    world_file(id, "constructions.bin")
}

pub struct WorldTilePaths {
    pub world_id: u32,
    pub biome_map_image: String,
    pub biome_map_binary: String,
    pub constructions_map_image: String,
    pub constructions_map_binary: String,
}