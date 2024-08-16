use raylib::math::Rectangle;

use crate::{constants::{ASSETS_PATH, TILE_SIZE, TILE_VARIATIONS_COUNT, TILE_VARIATIONS_FPS}, game_engine::entity::Entity, utils::timed_content_provider::TimedContentProvider};

pub trait Tile: Clone {
    fn into_obstacle_entity(&self) -> Box<dyn Entity>;
    fn row(&self) -> u32;
    fn column(&self) -> u32;
    fn is_same_tile_type(&self, other: &Self) -> bool;
    fn increment_width(&mut self, diff: u32);
}

pub trait SpriteTile: Tile {
    fn texture_source_rect(&self, variant: u32) -> Rectangle;
}

pub struct TileSet<T: Tile> {
    pub tiles: Vec<Vec<T>>,
    pub sheet_path: String,
    sprite_counter: TimedContentProvider<u32>,
}

impl<T: Tile> TileSet<T> {
    pub fn empty() -> Self {
        Self::with_tiles("".to_owned(), vec![])
    }

    pub fn with_tiles(sheet_path: String, tiles: Vec<Vec<T>>) -> Self {
        Self { 
            tiles,
            sheet_path: format!("{}/{}.png", ASSETS_PATH, sheet_path),
            sprite_counter: Self::content_provider()
        }
    }

    pub fn content_provider() -> TimedContentProvider<u32> {
        TimedContentProvider::new(Vec::from_iter(0..TILE_VARIATIONS_COUNT), TILE_VARIATIONS_FPS)
    }

    pub fn update(&mut self, time_since_last_update: f32) {
        self.sprite_counter.update(time_since_last_update);
    }

    pub fn current_variant(&self, row: u32, col: u32) -> u32 {
        (*self.sprite_counter.current_frame() + row + col) % TILE_VARIATIONS_COUNT
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

pub fn entity_is_on_tile(entity: &dyn Entity) -> bool {
    rect_is_on_tile(&entity.body().frame)
}

pub fn rect_is_on_tile(frame: &Rectangle) -> bool {
    frame.x % TILE_SIZE == 0.0 && frame.y % TILE_SIZE == 0.0
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
macro_rules! impl_tile {
    ($struct_name:ident) => {
        impl $crate::maps::tiles::Tile for $struct_name {
            fn into_obstacle_entity(&self) -> Box<dyn $crate::game_engine::entity::Entity> {
                let entity = $crate::game_engine::obstacles::StaticObstacle::new(
                    "invisible",
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
}