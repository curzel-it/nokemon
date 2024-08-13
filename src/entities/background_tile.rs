
use raylib::math::Rectangle;

use crate::{constants::{ASSETS_PATH, TILE_SIZE}, game_engine::{entity::Entity, entity_factory::EntityFactory}};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BackgroundTileType {
    Grass,
    Water,
    Rock,
    Desert, 
    Snow
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackgroundTileInfo {
    pub tile_type: BackgroundTileType,
    pub variant: i32,
    pub column: u32, 
    pub row: u32,
    pub width: u32,
    pub height: u32,
    pub tile_up_type: BackgroundTileType,
    pub tile_right_type: BackgroundTileType,
    pub tile_down_type: BackgroundTileType,
    pub tile_left_type: BackgroundTileType,
}

impl Default for BackgroundTileInfo {
    fn default() -> Self {
        BackgroundTileInfo {
            tile_type: BackgroundTileType::Grass,
            variant: 0,
            column: 0,
            row: 0,
            width: 0,
            height: 0,
            tile_up_type: BackgroundTileType::Grass,
            tile_right_type: BackgroundTileType::Grass,
            tile_down_type: BackgroundTileType::Grass,
            tile_left_type: BackgroundTileType::Grass,
        }
    }
}

impl BackgroundTileInfo {
    pub fn with_color_indeces(color: u32, column: u32, row: u32) -> Self {
        Self::with_color_indeces_size(color, column, row, 1, 1)
    }

    pub fn with_color_indeces_size(color: u32, column: u32, row: u32, width: u32, height: u32) -> Self {
        let tile_type = BackgroundTileType::from_color(color).unwrap_or(BackgroundTileType::Desert);            
        // let variant = rand::thread_rng().gen_range(0..10);
        
        Self {
            tile_type,
            variant: 1,
            column, 
            row,
            width,
            height,
            tile_up_type: BackgroundTileType::Grass,
            tile_right_type: BackgroundTileType::Grass,
            tile_down_type: BackgroundTileType::Grass,
            tile_left_type: BackgroundTileType::Grass,
        }
    }

    pub fn is_water(&self) -> bool {
        match &self.tile_type {
            BackgroundTileType::Water => true,
            _ => false
        }
    }

    pub fn sprite_name(&self) -> String {
        format!("{}/bg_tile_{}-{}.png", ASSETS_PATH, self.tile_type.animation_name(), self.variant)
    }

    pub fn into_obstacle_entity(&self, entity_factory: &EntityFactory) -> Box<dyn Entity> {
        let entity = entity_factory.build_invisible_obstacle(
            Rectangle::new(
                self.column as f32 * TILE_SIZE, 
                self.row as f32 * TILE_SIZE, 
                self.width as f32 * TILE_SIZE, 
                self.height as f32 * TILE_SIZE
            )
        );
        Box::new(entity)
    }
}

pub const COLOR_GRASS: u32 = 0x00FF00;
pub const COLOR_WATER: u32 = 0x0000FF;
pub const COLOR_ROCK: u32 = 0x7F7F7F;
pub const COLOR_DESERT: u32 = 0xFFFF00;
pub const COLOR_SNOW: u32 = 0xFFFFFF;

impl BackgroundTileType {
    fn animation_name(&self) -> &str {
        match self {
            BackgroundTileType::Grass => "grass",
            BackgroundTileType::Water => "water",
            BackgroundTileType::Rock => "rock",
            BackgroundTileType::Desert => "desert",
            BackgroundTileType::Snow => "snow",
        }
    }
    
    fn from_color(color: u32) -> Option<BackgroundTileType> {
        match color {
            COLOR_GRASS => Some(BackgroundTileType::Grass),
            COLOR_WATER => Some(BackgroundTileType::Water),
            COLOR_ROCK => Some(BackgroundTileType::Rock),
            COLOR_DESERT => Some(BackgroundTileType::Desert),
            COLOR_SNOW => Some(BackgroundTileType::Snow),
            _ => None,
        }
    }
}