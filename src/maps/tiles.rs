use raylib::math::Rectangle;

use crate::{constants::TILE_SIZE, game_engine::{entity::Entity, entity_factory::EntityFactory}};

pub trait Tile: Clone {
    fn sprite_name(&self) -> String;
    fn into_obstacle_entity(&self, entity_factory: &EntityFactory) -> Box<dyn Entity>;
    fn row(&self) -> u32;
    fn column(&self) -> u32;
    fn is_same_tile_type(&self, other: &Self) -> bool;
    fn increment_width(&mut self, diff: u32);
}

pub struct TileSet<T: Tile> {
    pub tiles: Vec<Vec<T>>,
}

impl<T: Tile> TileSet<T> {
    pub fn empty() -> Self {
        Self {
            tiles: vec![],
        }
    }

    pub fn with_tiles(tiles: Vec<Vec<T>>) -> Self {
        Self { tiles }
    }

    pub fn visible_tiles(&self, viewport: &Rectangle) -> Vec<&T> {
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

pub fn joined_tiles<T: Tile>(tiles: &Vec<T>) -> Vec<T> {
    let mut joined: Vec<T> = vec![];
    let mut previous = tiles[0].clone();
    
    for i in 1..tiles.len() {
        let current = tiles[i].clone();
        
        if current.is_same_tile_type(&previous) {
            previous.increment_width(1);
        } else {
            joined.push(previous.clone());
            previous = current;
        }
    }
    joined.push(previous.clone());

    joined
}

#[macro_export]
macro_rules! impl_tile_defaults {
    () => {
        fn into_obstacle_entity(
            &self, entity_factory: &$crate::game_engine::entity_factory::EntityFactory
        ) -> Box<dyn $crate::game_engine::entity::Entity> {
            let entity = entity_factory.build_invisible_obstacle(
                raylib::math::Rectangle::new(
                    self.column as f32 * TILE_SIZE, 
                    self.row as f32 * TILE_SIZE, 
                    self.width as f32 * TILE_SIZE, 
                    self.height as f32 * TILE_SIZE
                )
            );
            Box::new(entity)
        }
        
        fn row(&self) -> u32 {
            self.row
        }
        
        fn column(&self) -> u32 {
            self.column
        }
    
        fn is_same_tile_type(&self, other: &Self) -> bool {
            self.tile_type == other.tile_type
        }
    
        fn increment_width(&mut self, diff: u32) {
            self.width += diff;
        }
    }
}