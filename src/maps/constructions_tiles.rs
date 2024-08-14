use crate::{constants::{ASSETS_PATH, TILE_SIZE}, impl_tile_defaults};

use super::tiles::Tile;

pub const COLOR_WOODEN_FENCE: u32 = 0x391f21;

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

    pub fn is_something(&self) -> bool {
        self.tile_type != Construction::Nothing
    }
}

impl Tile for ConstructionTile {    
    fn sprite_name(&self, _: u32) -> String {
        let sprite_name = match self.tile_type {
            Construction::Nothing => "nothing".to_owned(),
            Construction::WoodenFence => self.wooden_fence_sprite_name()
        };
        let s = format!("{}/{}-0.png", ASSETS_PATH, sprite_name);
        println!("Fence: {:#?}", s);
        format!("{}/{}-0.png", ASSETS_PATH, sprite_name)
    }

    impl_tile_defaults!();
}

impl ConstructionTile {
    fn wooden_fence_sprite_name(&self) -> String {
        let mut names: Vec<String> = vec!["wooden_fence".to_owned()];

        if self.tile_up_type == self.tile_type {
            names.push("up".to_owned());
        }
        if self.tile_right_type == self.tile_type {
            names.push("right".to_owned());
        }
        if self.tile_down_type == self.tile_type {
            names.push("down".to_owned());
        }
        if self.tile_left_type == self.tile_type {
            names.push("left".to_owned());
        }
        if names.len() == 1 {
            names.push("alone".to_owned());
        }

        names.join("_")
    }
}

impl Construction {    
    fn from_color(color: u32) -> Option<Construction> {
        match color {
            COLOR_WOODEN_FENCE => Some(Construction::WoodenFence),
            _ => None,
        }
    }
}