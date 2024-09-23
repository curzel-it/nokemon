use crate::{constants::{SPRITE_SHEET_BIOME_TILES, WORLD_SIZE_COLUMNS, WORLD_SIZE_ROWS}, game_engine::world::World, utils::rect::Rect};

use super::{biome_tiles::{Biome, BiomeTile}, tiles::TileSet};

impl World {    
    pub fn load_biome_tiles(&mut self, tiles: TileSet<BiomeTile>) {
        let mut grass = BiomeTile::from_data(1, 1, '1');
        grass.setup_neighbors(Biome::Grass, Biome::Grass, Biome::Grass, Biome::Grass);

        let tiles = if tiles.tiles.is_empty() {
            TileSet::<BiomeTile>::with_tiles(
                SPRITE_SHEET_BIOME_TILES,
                vec![vec![grass; WORLD_SIZE_COLUMNS]; WORLD_SIZE_ROWS]
            )
        } else {
            tiles
        };
        self.bounds = Rect::new(0, 0, tiles.tiles[0].len() as i32, tiles.tiles.len() as i32);
        self.biome_tiles = tiles;            
    }
}