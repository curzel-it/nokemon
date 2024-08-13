
use raylib::math::Rectangle;

use crate::{constants::{ASSETS_PATH, TILE_SIZE}, impl_tile_defaults};

use super::tiles::Tile;

pub const COLOR_GRASS: u32 = 0x00FF00;
pub const COLOR_WATER: u32 = 0x0000FF;
pub const COLOR_ROCK: u32 = 0x7F7F7F;
pub const COLOR_DESERT: u32 = 0xFFFF00;
pub const COLOR_SNOW: u32 = 0xFFFFFF;

pub struct BiomeTileSet {
    pub tiles: Vec<Vec<BiomeTile>>,
}

impl BiomeTileSet {
    pub fn empty() -> Self {
        Self {
            tiles: vec![]
        }
    }

    pub fn with_tiles(tiles: Vec<Vec<BiomeTile>>) -> Self {
        Self {
            tiles
        }
    }

    pub fn visible_tiles(&self, viewport: &Rectangle) -> Vec<&BiomeTile> {
        let first_row = (viewport.y / TILE_SIZE).floor() as usize;
        let rows = (viewport.height / TILE_SIZE).ceil() as usize + 1;
        let first_col = (viewport.x / TILE_SIZE).floor() as usize;
        let cols = (viewport.width / TILE_SIZE).ceil() as usize + 1;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Biome {
    Grass,
    Water,
    Rock,
    Desert, 
    Snow
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BiomeTile {
    pub tile_type: Biome,
    pub variant: i32,
    pub column: u32, 
    pub row: u32,
    pub width: u32,
    pub height: u32,
    pub tile_up_type: Biome,
    pub tile_right_type: Biome,
    pub tile_down_type: Biome,
    pub tile_left_type: Biome,
}

impl Default for BiomeTile {
    fn default() -> Self {
        BiomeTile {
            tile_type: Biome::Grass,
            variant: 0,
            column: 0,
            row: 0,
            width: 1,
            height: 1,
            tile_up_type: Biome::Grass,
            tile_right_type: Biome::Grass,
            tile_down_type: Biome::Grass,
            tile_left_type: Biome::Grass,
        }
    }
}

impl Tile for BiomeTile {
    fn sprite_name(&self) -> String {
        format!("{}/bg_tile_{}-{}.png", ASSETS_PATH, self.tile_type.animation_name(), self.variant)
    }

    impl_tile_defaults!();
}

impl Biome {
    fn animation_name(&self) -> &str {
        match self {
            Biome::Grass => "grass",
            Biome::Water => "water",
            Biome::Rock => "rock",
            Biome::Desert => "desert",
            Biome::Snow => "snow",
        }
    }
    
    fn from_color(color: u32) -> Option<Biome> {
        match color {
            COLOR_GRASS => Some(Biome::Grass),
            COLOR_WATER => Some(Biome::Water),
            COLOR_ROCK => Some(Biome::Rock),
            COLOR_DESERT => Some(Biome::Desert),
            COLOR_SNOW => Some(Biome::Snow),
            _ => None,
        }
    }
}

impl BiomeTile {
    pub fn with_color_indeces(color: u32, column: u32, row: u32) -> Self {
        Self::with_color_indeces_size(color, column, row, 1, 1)
    }

    pub fn with_color_indeces_size(color: u32, column: u32, row: u32, width: u32, height: u32) -> Self {
        let tile_type = Biome::from_color(color).unwrap_or(Biome::Desert);            
        // let variant = rand::thread_rng().gen_range(0..10);
        
        Self {
            tile_type,
            variant: 1,
            column, 
            row,
            width,
            height,
            tile_up_type: Biome::Grass,
            tile_right_type: Biome::Grass,
            tile_down_type: Biome::Grass,
            tile_left_type: Biome::Grass,
        }
    }

    pub fn is_water(&self) -> bool {
        match &self.tile_type {
            Biome::Water => true,
            _ => false
        }
    }
}