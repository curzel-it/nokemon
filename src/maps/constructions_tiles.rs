use crate::{constants::{SPRITE_SHEET_BLANK, TILE_TEXTURE_SIZE}, impl_tile, utils::rect::Rect};

use super::tiles::{SpriteTile, TileSet};

pub const COLOR_WOODEN_FENCE: u32 = 0x391F21ff;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum Construction {
    WoodenFence,
    Nothing,
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

impl_tile!(ConstructionTile);

impl SpriteTile for ConstructionTile {
    fn texture_source_rect(&self, _: u32) -> Rect {
        Rect::new(
            self.texture_offset_x,
            self.texture_offset_y,
            TILE_TEXTURE_SIZE,
            TILE_TEXTURE_SIZE,
        )
    }
}

impl ConstructionTile {
    pub fn is_something(&self) -> bool {
        self.tile_type != Construction::Nothing
    }

    pub fn setup_textures(&mut self) {
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
            (true, true, true, false) => 11,
            (true, false, true, true) => 12,
            (true, true, false, true) => 13,
            (false, true, true, true) => 14,
            (true, true, true, true) => 15,
            _ => 0,
        };
        self.texture_offset_x = TILE_TEXTURE_SIZE * x as f32;
        self.texture_offset_y = TILE_TEXTURE_SIZE * y as f32;
    }
}

impl Construction {
    pub fn sprite(&self) -> u32 {
        match self {
            Construction::Nothing => SPRITE_SHEET_BLANK,
            Construction::WoodenFence => SPRITE_SHEET_BLANK,
        }
    }

    fn texture_offset_x(&self) -> u32 {
        match self {
            Construction::Nothing => 0,
            Construction::WoodenFence => 1,
        }
    }

    pub fn from_color(color: u32) -> Construction {
        match color {
            COLOR_WOODEN_FENCE => Construction::WoodenFence,
            _ => Construction::Nothing
        }
    }
}

impl TileSet<ConstructionTile> {
    pub fn update_tile(&mut self, row: usize, col: usize, new_biome: Construction) {
        self.tiles[row][col].tile_type = new_biome;
        self.tiles[row][col].setup_textures();

        if row > 0 {
            self.tiles[row-1][col].tile_down_type = new_biome;
            self.tiles[row-1][col].setup_textures();
        }
        if row < self.tiles.len() - 1 {
            self.tiles[row+1][col].tile_up_type = new_biome;
            self.tiles[row+1][col].setup_textures();
        }
        if col > 0 {
            self.tiles[row][col-1].tile_right_type = new_biome;
            self.tiles[row][col-1].setup_textures();
        }
        if col < self.tiles[0].len() - 1 {
            self.tiles[row][col+1].tile_left_type = new_biome;
            self.tiles[row][col+1].setup_textures();
        }
    }
}