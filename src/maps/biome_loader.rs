use crate::{game_engine::world::World, utils::rect::Rect};

use super::{biome_tiles::BiomeTile, tiles::TileSet};

impl World {    
    pub fn load_biome_tiles(&mut self, tiles: TileSet<BiomeTile>) {
        self.bounds = Rect::new(0, 0, tiles.tiles[0].len() as i32, tiles.tiles.len() as i32);
        self.biome_tiles = tiles;
    }
}