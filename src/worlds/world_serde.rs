use std::{fs::File, io::{BufReader, Write}};

use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Error;
use crate::{constants::{SPRITE_SHEET_BIOME_TILES, SPRITE_SHEET_CONSTRUCTION_TILES}, entities::building::Building, game_engine::world::World, maps::{biome_tiles::BiomeTile, constructions_tiles::ConstructionTile, tiles::TileSet}, worlds::constants::WORLD_DEMO_WORLD};

const save_file_path: &str = "save_game.json";    

impl World {
    pub fn load(id: u32) -> Option<Self> {
        if let Ok(file) = File::open(save_file_path) {
            let reader = BufReader::new(file);        
            let result: Result<Self, Error> = serde_json::from_reader(reader);

            if let Ok(world) = result {
                println!("Game saved successfully!");
                return Some(world)
            } else {
                println!("Failed to parse game {}: {:#?}", id, result.err());
            } 
        } else {
            println!("Failed to load game file");
        }
        None
    }

    pub fn load_or_default(id: u32) -> Self {
        Self::load(id).unwrap_or_else(Self::empty)
    }

    pub fn save(&self) {
        if let Ok(serialized_world) = serde_json::to_string(self) {
            if let Ok(mut file) = File::create(save_file_path) {
                if let Err(e) = file.write_all(serialized_world.as_bytes()) {
                    eprintln!("Failed to write save file: {}", e);
                } else {
                    println!("Game saved successfully to {}", save_file_path);
                }
            } else {
                eprintln!("Failed to create save file");
            }
        } else {
            eprintln!("Failed to serialize game world");
        }
    }

    fn empty() -> Self {
        let mut world = World::new(WORLD_DEMO_WORLD);

        let biome_tile_set = TileSet::<BiomeTile>::with_tiles(
            SPRITE_SHEET_BIOME_TILES, 
            (0..200).map(|row| {
                (0..150).map(|column| {
                    let mut tile = BiomeTile::from_data(row as usize, column as usize, 1);
                    tile.setup_neighbors(tile.tile_type, tile.tile_type, tile.tile_type, tile.tile_type);
                    tile
                }).collect()
            }).collect()
        );
        world.load_biome_tiles(biome_tile_set);

        let construction_tile_set = TileSet::<ConstructionTile>::with_tiles(
            SPRITE_SHEET_CONSTRUCTION_TILES, 
            (0..200).map(|row| {
                (0..150).map(|column| {
                    let mut tile = ConstructionTile::from_data(row as usize, column as usize, 0);
                    tile.setup_neighbors(tile.tile_type, tile.tile_type, tile.tile_type, tile.tile_type);
                    tile
                }).collect()
            }).collect()
        );
        world.load_construction_tiles(construction_tile_set);

        world
    }    
}

#[derive(Serialize, Deserialize)]
struct WorldData {
    world_id: u32,
    biome_tiles: TileSet<BiomeTile>,
    constructions_tiles: TileSet<ConstructionTile>,
    buildings: Vec<Building>,
}

impl Serialize for World {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let entities = self.entities.borrow();
        let buildings: Vec<&Building> = entities.values()
            .filter_map(|entity| {
                entity.as_ref().as_any().downcast_ref::<Building>()
            })
            .collect();

        let mut state = serializer.serialize_struct("World", 4)?;
        state.serialize_field("world_id", &self.world_id)?;
        state.serialize_field("biome_tiles", &self.biome_tiles)?;
        state.serialize_field("constructions_tiles", &self.constructions_tiles)?;
        state.serialize_field("buildings", &buildings)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for World {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        #[derive(Deserialize)]
        struct WorldData {
            world_id: u32,
            biome_tiles: TileSet<BiomeTile>,
            constructions_tiles: TileSet<ConstructionTile>,
            buildings: Vec<Building>,
        }

        let WorldData {
            world_id,
            biome_tiles,
            constructions_tiles,
            buildings,
        } = WorldData::deserialize(deserializer)?;

        let mut world = World::new(world_id);
        for building in buildings {
            world.add_entity(Box::new(building));
        }
        world.load_biome_tiles(biome_tiles);
        world.load_construction_tiles(constructions_tiles);
        Ok(world)
    }
}