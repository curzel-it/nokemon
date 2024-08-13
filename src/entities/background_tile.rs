use serde::{Deserialize, Serialize};

use raylib::math::Rectangle;

use crate::{constants::{ASSETS_PATH, TILE_SIZE}, game_engine::{entity::Entity, entity_factory::EntityFactory}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BackgroundTileType {
    Grass,
    Water,
    Rock,
    Desert, 
    Snow
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct BackgroundTileInfo {
    pub tile_type: BackgroundTileType,
    pub column: u32, 
    pub row: u32,

    #[serde(default="df_one")]
    pub width: u32,

    #[serde(default="df_one")]
    pub height: u32,

    #[serde(default="df_zero")]
    pub variant: i32,

    #[serde(default="df_grass")]
    pub tile_up_type: BackgroundTileType,

    #[serde(default="df_grass")]
    pub tile_right_type: BackgroundTileType,

    #[serde(default="df_grass")]
    pub tile_down_type: BackgroundTileType,

    #[serde(default="df_grass")]
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
}

fn df_grass() -> BackgroundTileType {
    BackgroundTileType::Grass
}

fn df_one() -> u32 {
    1
}

fn df_zero() -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    impl BackgroundTileInfo {
        pub fn with_type_indeces(tile_type: BackgroundTileType, column: u32, row: u32) -> Self {
            Self::with_type_indeces_size(tile_type, column, row, 1, 1)
        }

        pub fn with_type_indeces_size(tile_type: BackgroundTileType, column: u32, row: u32, width: u32, height: u32) -> Self {
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
    }
}