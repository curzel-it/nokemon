use std::{fs::File, io::{BufReader, Write}};

use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Error;
use crate::{constants::{SPRITE_SHEET_BIOME_TILES, SPRITE_SHEET_CONSTRUCTION_TILES, WORLD_SIZE_COLUMNS, WORLD_SIZE_ROWS}, entities::{known_species::SPECIES_HERO, species::Species}, game_engine::{entity::Entity, world::World}, maps::{biome_tiles::BiomeTile, constructions_tiles::ConstructionTile, tiles::TileSet}};

use super::utils::world_path;

impl World {
    pub fn load(id: u32) -> Option<Self> {
        let path = world_path(id);

        if let Ok(file) = File::open(path.clone()) {
            let reader = BufReader::new(file);        
            let result: Result<Self, Error> = serde_json::from_reader(reader);

            if let Ok(world) = result {
                println!("Game saved successfully!");
                return Some(world)
            } else {
                println!("Failed to parse game {}: {:#?}", path, result.err());
            } 
        } else {
            println!("Failed to load game file at {}", path);
        }
        None
    }

    pub fn load_or_create(id: u32) -> Self {
        Self::load(id).unwrap_or_else(|| {
            let new = Self::new_with_default_tiles(id);
            new.save();
            new
        })
    }

    pub fn save(&self) {
        let path = world_path(self.id);

        if let Ok(serialized_world) = serde_json::to_string_pretty(self) {
            if let Ok(mut file) = File::create(path.clone()) {
                if let Err(e) = file.write_all(serialized_world.as_bytes()) {
                    eprintln!("Failed to write save file: {}", e);
                } else {
                    println!("Game saved successfully to {}", path);
                }
            } else {
                eprintln!("Failed to create save file");
            }
        } else {
            eprintln!("Failed to serialize game world");
        }
    }

    fn new_with_default_tiles(id: u32) -> Self {
        let mut world = World::new(id);

        let biome_tile_set = TileSet::<BiomeTile>::with_tiles(
            SPRITE_SHEET_BIOME_TILES, 
            (0..WORLD_SIZE_ROWS).map(|row| {
                (0..WORLD_SIZE_COLUMNS).map(|column| {
                    let mut tile = BiomeTile::from_data(row, column, '0');
                    tile.setup_neighbors(tile.tile_type, tile.tile_type, tile.tile_type, tile.tile_type);
                    tile
                }).collect()
            }).collect()
        );
        world.load_biome_tiles(biome_tile_set);

        let construction_tile_set = TileSet::<ConstructionTile>::with_tiles(
            SPRITE_SHEET_CONSTRUCTION_TILES, 
            (0..WORLD_SIZE_ROWS).map(|row| {
                (0..WORLD_SIZE_COLUMNS).map(|column| {
                    let mut tile = ConstructionTile::from_data(row, column, '0');
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
    id: u32,
    biome_tiles: TileSet<BiomeTile>,
    constructions_tiles: TileSet<ConstructionTile>,
    entities: Vec<Entity>
}

impl Serialize for World {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {       
        let borrowed_entities = self.entities.borrow();
        let entities: Vec<&Entity> = borrowed_entities.iter()
            .filter(|e| e.species.id != SPECIES_HERO)
            .collect();

        let mut state = serializer.serialize_struct("World", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("biome_tiles", &self.biome_tiles)?;
        state.serialize_field("constructions_tiles", &self.constructions_tiles)?;
        state.serialize_field("entities", &entities)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for World {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let WorldData {
            id,
            biome_tiles,
            constructions_tiles,
            entities,
        } = WorldData::deserialize(deserializer)?;

        let mut world = World::new(id);                
        entities.into_iter().for_each(|e| _ = world.add_entity(e));        
        world.load_biome_tiles(biome_tiles);
        world.load_construction_tiles(constructions_tiles);
        Ok(world)
    }
}