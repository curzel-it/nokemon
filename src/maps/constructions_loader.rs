use crate::{constants::{SPRITE_SHEET_CONSTRUCTION_TILES, WORLD_SIZE_COLUMNS, WORLD_SIZE_ROWS}, game_engine::world::World};

use super::{constructions_tiles::ConstructionTile, tiles::TileSet};

impl World {    
    pub fn load_construction_tiles(&mut self, tiles: TileSet<ConstructionTile>) {
        let nothing = ConstructionTile::from_data('0');
        let tiles = if tiles.tiles.is_empty() {
            TileSet::<ConstructionTile>::with_tiles(
                SPRITE_SHEET_CONSTRUCTION_TILES,
                vec![vec![nothing; WORLD_SIZE_COLUMNS]; WORLD_SIZE_ROWS]
            )
        } else {
            tiles
        };
        self.constructions_tiles = tiles;     
    }
}