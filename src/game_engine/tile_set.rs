use raylib::math::Rectangle;

use crate::{constants::BG_TILE_SIZE, entities::background_tile::BackgroundTile};

pub struct TileSet {
    pub tiles: Vec<Vec<BackgroundTile>>,
}

impl TileSet {
    pub fn empty() -> Self {
        Self {
            tiles: vec![]
        }
    }

    pub fn with_tiles(tiles: Vec<Vec<BackgroundTile>>) -> Self {
        Self {
            tiles
        }
    }

    pub fn visible_tiles(&self, viewport: &Rectangle) -> Vec<&BackgroundTile> {
        let first_row = (viewport.y / BG_TILE_SIZE).floor() as usize;
        let rows = (viewport.height / BG_TILE_SIZE).ceil() as usize;
        let first_col = (viewport.x / BG_TILE_SIZE).floor() as usize;
        let cols = (viewport.width / BG_TILE_SIZE).ceil() as usize;

        let mut visible_tiles = Vec::new();

        for row in first_row..(first_row + rows) {
            for col in first_col..(first_col + cols) {
                if row < self.tiles.len() && col < self.tiles[row].len() {
                    let tile = &self.tiles[row][col];
                    visible_tiles.push(tile);
                }
            }
        }

        visible_tiles    
    }
}