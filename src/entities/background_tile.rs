use rand::Rng;

use raylib::math::Rectangle;

use crate::{constants::{ASSETS_PATH, BG_TILE_SIZE, INVISIBLE_OBSTACLE_Z_INDEX}, game_engine::{entity::Entity, entity_body::EntityBody, entity_factory::{self, EntityFactory}, obstacles::StaticObstacle, simple_entity::SimpleEntity, world::World, world_state_update::WorldStateUpdate}, impl_embodied_entity};

#[derive(Debug)]
pub enum BackgroundTileType {
    Grass,
    Water,
    Rock,
    Desert, 
    Snow
}

#[derive(Debug)]
pub struct BackgroundTileInfo {
    pub tile_type: BackgroundTileType,
    pub variant: i32,
    pub column: u32, 
    pub row: u32,
    pub has_ground_contact: bool,
    pub has_water_contact: bool,
}

impl BackgroundTileInfo {
    pub fn new(color: u32, column: u32, row: u32) -> Self {
        let tile_type = BackgroundTileType::from_color(color).unwrap_or(BackgroundTileType::Desert);            
        let variant = rand::thread_rng().gen_range(0..10);
        
        Self {
            tile_type,
            variant,
            column, row,
            has_ground_contact: false,
            has_water_contact: false,
        }
    }

    pub fn is_water(&self) -> bool {
        match &self.tile_type {
            BackgroundTileType::Water => true,
            _ => false
        }
    }

    pub fn is_not_water(&self) -> bool {
        !self.is_water()
    }

    pub fn sprite_name(&self) -> String {
        format!("{}/bg_tile_{}-{}.png", ASSETS_PATH, self.tile_type.animation_name(), self.variant)
    }

    pub fn into_obstacle_entity(&self, entity_factory: &EntityFactory) -> Box<dyn Entity> {
        let entity = entity_factory.build_invisible_obstacle(
            Rectangle::new(
                self.column as f32 * BG_TILE_SIZE, 
                self.row as f32 * BG_TILE_SIZE, 
                BG_TILE_SIZE, 
                BG_TILE_SIZE
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
    
    fn from_color(color: u32) -> Option<BackgroundTileType> {
        match color {
            0x00FF00 => Some(BackgroundTileType::Grass),
            0x0000FF => Some(BackgroundTileType::Water),
            0x7F7F7F => Some(BackgroundTileType::Rock),
            0xFFFF00 => Some(BackgroundTileType::Desert),
            0xFFFFFF => Some(BackgroundTileType::Snow),
            _ => None,
        }
    }
}