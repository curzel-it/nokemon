use raylib::math::Vector2;

use crate::{constants::{BG_TILE_SIZE, BG_TILE_Z_INDEX}, game_engine::{entity::Entity, entity_body::EntityBody, entity_factory::EntityFactory, world::World, game_state_update::GameStateUpdate}, impl_embodied_entity};

#[derive(Debug)]
pub enum BackgroundTileType {
    Grass,
    Water,
    Rock,
    Desert, 
    Snow
}

#[derive(Debug)]
pub struct BackgroundTile {
    body: EntityBody
}

impl BackgroundTile {
    pub fn new(body: EntityBody) -> Self {
        Self { 
            body
        }
    }
}

impl_embodied_entity!(BackgroundTile);

impl Entity for BackgroundTile {
    fn update(&mut self, _: &World, _: f32) -> Vec<GameStateUpdate> {
        vec![]
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

impl EntityFactory {
    pub fn build_background_tile(&self, tile_type: &BackgroundTileType, index: usize, row: u32, column: u32) -> BackgroundTile {
        let mut body = self.build("bg_tile");
        body.set_animation(tile_type.animation_name());
        body.current_sprite.jump_to_frame(index);
        body.resize(BG_TILE_SIZE, BG_TILE_SIZE);
        body.frame.x = column as f32 * BG_TILE_SIZE;
        body.frame.y = row as f32 * BG_TILE_SIZE;
        body.z_index = BG_TILE_Z_INDEX;
        body.base_speed = 0.0;
        body.reset_speed();
        body.direction = Vector2::new(0.0, 0.0);    
        BackgroundTile::new(body)
    }
}