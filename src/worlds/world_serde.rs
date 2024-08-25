use std::{fs::File, io::{BufReader, Write}};

use serde::{ser::SerializeStruct, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Error;
use uuid::Uuid;
use crate::{constants::{SPRITE_SHEET_BIOME_TILES, SPRITE_SHEET_CONSTRUCTION_TILES}, entities::{building::Building, teleporter::Teleporter}, game_engine::world::World, maps::{biome_tiles::BiomeTile, constructions_tiles::ConstructionTile, tiles::TileSet}};

use super::utils::world_path;

impl World {
    pub fn load(id: Uuid) -> Option<Self> {
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

    pub fn load_or_create(id: Uuid) -> Self {
        Self::load(id).unwrap_or_else(|| {
            let new = Self::new_with_default_tiles(id);
            new.save();
            new
        })
    }

    pub fn save(&self) {
        let path = world_path(self.id);

        if let Ok(serialized_world) = serde_json::to_string(self) {
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

    fn new_with_default_tiles(id: Uuid) -> Self {
        let mut world = World::new(id);

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
            (0..150).map(|row| {
                (0..200).map(|column| {
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
    id: Uuid,
    biome_tiles: TileSet<BiomeTile>,
    constructions_tiles: TileSet<ConstructionTile>,
    buildings: Vec<Building>,
    teleporters: Vec<Teleporter>,
}

impl Serialize for World {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        let entities = self.entities.borrow();

        let buildings: Vec<&Building> = entities.values()
            .filter_map(|e| e.as_ref().as_any().downcast_ref::<Building>())
            .collect();

        let teleporters: Vec<&Teleporter> = entities.values()
            .filter_map(|e| e.as_ref().as_any().downcast_ref::<Teleporter>())
            .collect();

        let mut state = serializer.serialize_struct("World", 4)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("biome_tiles", &self.biome_tiles)?;
        state.serialize_field("constructions_tiles", &self.constructions_tiles)?;
        state.serialize_field("buildings", &buildings)?;
        state.serialize_field("teleporters", &teleporters)?;
        state.end()
    }
}

impl<'de> Deserialize<'de> for World {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error> where D: Deserializer<'de> {
        let WorldData {
            id,
            biome_tiles,
            constructions_tiles,
            buildings,
            teleporters,
        } = WorldData::deserialize(deserializer)?;

        let mut world = World::new(id);
        buildings.into_iter().for_each(|b| { world.add_entity(Box::new(b)); });
        teleporters.into_iter().for_each(|b| { world.add_entity(Box::new(b)); });
        world.load_biome_tiles(biome_tiles);
        world.load_construction_tiles(constructions_tiles);
        Ok(world)
    }
}