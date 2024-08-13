
use raylib::math::Rectangle;

use crate::{constants::{ASSETS_PATH, TILE_SIZE}, impl_tile_defaults};

use super::tiles::Tile;

pub const COLOR_WOODEN_FENCE: u32 = 0x391f21;
pub const COLOR_NOTHING: u32 = 0x000000;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Construction {
    WoodenFence,
    Nothing
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConstructionTile {
    pub tile_type: Construction,
    pub column: u32, 
    pub row: u32,
    pub width: u32,
    pub height: u32,
    pub tile_up_type: Construction,
    pub tile_right_type: Construction,
    pub tile_down_type: Construction,
    pub tile_left_type: Construction,
}

impl Default for ConstructionTile {
    fn default() -> Self {
        ConstructionTile {
            tile_type: Construction::Nothing,
            column: 0,
            row: 0,
            width: 1,
            height: 1,
            tile_up_type: Construction::Nothing,
            tile_right_type: Construction::Nothing,
            tile_down_type: Construction::Nothing,
            tile_left_type: Construction::Nothing,
        }
    }
}

impl ConstructionTile { 
    pub fn with_color_indeces(color: u32, column: u32, row: u32) -> Self {
        Self::with_color_indeces_size(color, column, row, 1, 1)
    }

    pub fn with_color_indeces_size(color: u32, column: u32, row: u32, width: u32, height: u32) -> Self {
        let tile_type = Construction::from_color(color).unwrap_or(Construction::Nothing);            
        
        Self {
            tile_type,
            column, 
            row,
            width,
            height,
            tile_up_type: Construction::Nothing,
            tile_right_type: Construction::Nothing,
            tile_down_type: Construction::Nothing,
            tile_left_type: Construction::Nothing,
        }
    }
}

impl Tile for ConstructionTile {    
    fn sprite_name(&self) -> String {
        format!("{}/{}-0.png", ASSETS_PATH, self.tile_type.animation_name())
    }

    impl_tile_defaults!();
}

impl Construction {
    fn animation_name(&self) -> &str {
        match self {
            Construction::Nothing => "nothing",
            Construction::WoodenFence => "wooden_fence_stills",
        }
    }
    
    fn from_color(color: u32) -> Option<Construction> {
        match color {
            COLOR_WOODEN_FENCE => Some(Construction::WoodenFence),
            _ => None,
        }
    }
}