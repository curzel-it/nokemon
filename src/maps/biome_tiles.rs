

use crate::{constants::{ASSETS_PATH, TILE_SIZE}, impl_tile_defaults};

use super::tiles::Tile;

pub const COLOR_GRASS: u32 = 0x00FF00;
pub const COLOR_WATER: u32 = 0x0000FF;
pub const COLOR_ROCK: u32 = 0x7F7F7F;
pub const COLOR_DESERT: u32 = 0xFFFF00;
pub const COLOR_SNOW: u32 = 0xFFFFFF;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Biome {
    Grass,
    Water,
    Rock,
    Desert, 
    Snow
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BiomeTile {
    pub tile_type: Biome,
    pub column: u32, 
    pub row: u32,
    pub width: u32,
    pub height: u32,
    pub tile_up_type: Biome,
    pub tile_right_type: Biome,
    pub tile_down_type: Biome,
    pub tile_left_type: Biome,
    pub neighbor_sprite_name_suffix: String
}

impl Default for BiomeTile {
    fn default() -> Self {
        BiomeTile {
            tile_type: Biome::Grass,
            column: 0,
            row: 0,
            width: 1,
            height: 1,
            tile_up_type: Biome::Grass,
            tile_right_type: Biome::Grass,
            tile_down_type: Biome::Grass,
            tile_left_type: Biome::Grass,
            neighbor_sprite_name_suffix: "".to_owned()
        }
    }
}

impl Tile for BiomeTile {
    fn sprite_name(&self, variant: u32) -> String {
        format!("{}/bg_tile_{}{}-{}.png", ASSETS_PATH, self.tile_type.animation_name(), self.neighbor_sprite_name_suffix, variant)
    }
    impl_tile_defaults!();
}

impl BiomeTile {
    pub fn setup_neighbors(&mut self, up: Biome, right: Biome, bottom: Biome, left: Biome) {
        self.tile_up_type = up;
        self.tile_right_type = right;
        self.tile_down_type = bottom;
        self.tile_left_type = left;        
        self.setup_mixed_biomes();    
    }

    fn setup_mixed_biomes(&mut self) {
        match self.tile_type {
            Biome::Desert => {
                self.setup_mixed_biome(Biome::Snow);
                self.setup_mixed_biome(Biome::Grass);
                self.setup_mixed_biome(Biome::Rock);
                self.setup_mixed_biome(Biome::Water);
            },
            Biome::Grass => {
                self.setup_mixed_biome(Biome::Snow);
                self.setup_mixed_biome(Biome::Desert);
                self.setup_mixed_biome(Biome::Rock);
            },
            Biome::Snow => {
                self.setup_mixed_biome(Biome::Grass);
                self.setup_mixed_biome(Biome::Rock);
            },
            Biome::Water => {
                self.setup_mixed_biome(Biome::Grass);
            }
            Biome::Rock => {}
        }
    }

    fn setup_mixed_biome(&mut self, biome: Biome) {
        if self.tile_type == biome { 
            return;
        }

        let mut directions: String = "".to_owned();

        if self.tile_up_type == biome { directions += "n"; }
        if self.tile_right_type == biome { directions += "e"; }
        if self.tile_down_type == biome { directions += "s"; }
        if self.tile_left_type == biome { directions += "w"; }

        if !directions.is_empty() {
            self.neighbor_sprite_name_suffix = format!("_{}_{}", biome.animation_name(), directions);            
        }
    }
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
        
        Self {
            tile_type,
            column, 
            row,
            width,
            height,
            tile_up_type: Biome::Grass,
            tile_right_type: Biome::Grass,
            tile_down_type: Biome::Grass,
            tile_left_type: Biome::Grass,
            neighbor_sprite_name_suffix: "".to_owned()
        }
    }

    pub fn is_water(&self) -> bool {
        match &self.tile_type {
            Biome::Water => true,
            _ => false
        }
    }
}