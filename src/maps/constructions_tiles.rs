use raylib::math::Rectangle;

use crate::{constants::{TILE_SIZE, TILE_TEXTURE_SIZE}, impl_tile};

use super::tiles::SpriteTile;

pub const COLOR_WOODEN_FENCE: u32 = 0x391f21;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Construction {
    WoodenFence,
    Nothing
}

#[derive(Debug, Clone, Copy)]
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
    pub texture_offset_x: f32,
    pub texture_offset_y: f32,
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
            texture_offset_x: 0.0,
            texture_offset_y: 0.0,
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
            texture_offset_x: tile_type.texture_offset_x() as f32,
            texture_offset_y: 0.0,
        }
    }

    pub fn is_something(&self) -> bool {
        self.tile_type != Construction::Nothing
    }
}

impl_tile!(ConstructionTile);

impl SpriteTile for ConstructionTile {    
    fn texture_source_rect(&self, _: u32) -> Rectangle {
        Rectangle::new(
            self.texture_offset_x,
            self.texture_offset_y,
            TILE_TEXTURE_SIZE, 
            TILE_TEXTURE_SIZE
        )
    }
}

impl ConstructionTile {
    pub fn setup_neighbors(&mut self, up: Construction, right: Construction, bottom: Construction, left: Construction) {
        self.tile_up_type = up;
        self.tile_right_type = right;
        self.tile_down_type = bottom;
        self.tile_left_type = left;    
        self.setup_tile();     
    }

    fn setup_tile(&mut self) {
        let same_up = self.tile_up_type == self.tile_type;
        let same_right = self.tile_right_type == self.tile_type;
        let same_down = self.tile_down_type == self.tile_type;
        let same_left = self.tile_left_type == self.tile_type;

        let x = self.tile_type.texture_offset_x();
        let y = match (same_up, same_right, same_down, same_left) {
            (false, true, false, true) => 0,
            (false, false, false, false) => 1,
            (false, false, false, true) => 2,
            (false, true, false, false) => 3,
            (true, false, true, false) => 4,
            (true, false, false, false) => 5,
            (false, false, true, false) => 6,
            (true, true, false, false) => 7,
            (true, false, false, true) => 8,
            (false, true, true, false) => 9,
            (false, false, true, true) => 10,
            _ => 0
        };
        self.texture_offset_x = TILE_TEXTURE_SIZE * x as f32;
        self.texture_offset_y = TILE_TEXTURE_SIZE * y as f32;
    }
}

impl Construction {    
    fn texture_offset_x(&self) -> u32 {
        match self {
            Construction::Nothing => 0,
            Construction::WoodenFence => 1
        }
    }

    fn from_color(color: u32) -> Option<Construction> {
        match color {
            COLOR_WOODEN_FENCE => Some(Construction::WoodenFence),
            _ => None,
        }
    }
}