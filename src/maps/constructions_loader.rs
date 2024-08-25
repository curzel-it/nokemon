use crate::game_engine::world::World;

use super::{constructions_tiles::ConstructionTile, tiles::TileSet};

impl World {    
    pub fn load_construction_tiles(&mut self, tiles: TileSet<ConstructionTile>) {
        self.constructions_tiles = tiles;
    }
}