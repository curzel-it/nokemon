use crate::levels::constants::{LEVELS_PATH, LEVEL_DEMO_WORLD, LEVEL_ID_HOUSE_INTERIOR};

pub fn level_tiles_paths() -> Vec<LevelTilePaths> {
    all_levels()
        .into_iter()
        .map(|id| {
            LevelTilePaths {
                level_id: id,
                biome_map_image: level_biome_image_path(id),
                biome_map_binary: level_biome_binary_path(id),
                constructions_map_image: level_constructions_image_path(id),
                constructions_map_binary: level_constructions_binary_path(id),
            }
        })
        .collect()
}

fn all_levels() -> Vec<u32> {
    vec![
        LEVEL_DEMO_WORLD,
        LEVEL_ID_HOUSE_INTERIOR,
    ]
}

fn level_name_by_id(id: u32) -> String {
    let name = match id {
        LEVEL_DEMO_WORLD => "world",
        LEVEL_ID_HOUSE_INTERIOR => "house_interior",
        _ => "unknown"
    };
    name.to_owned()
}

fn level_file(id: u32, name: &str) -> String {
    format!("{}/{}_{}", LEVELS_PATH, level_name_by_id(id), name)
}

pub fn level_biome_image_path(id: u32) -> String {
    level_file(id, "biome.png")
}

pub fn level_biome_binary_path(id: u32) -> String {
    level_file(id, "biome.bin")
}

pub fn level_constructions_image_path(id: u32) -> String {
    level_file(id, "constructions.png")
}

pub fn level_constructions_binary_path(id: u32) -> String {
    level_file(id, "constructions.bin")
}

pub struct LevelTilePaths {
    pub level_id: u32,
    pub biome_map_image: String,
    pub biome_map_binary: String,
    pub constructions_map_image: String,
    pub constructions_map_binary: String,
}