
use crate::{constants::{SPRITE_SHEET_BLANK, BIOME_NUMBER_OF_FRAMES, TILE_VARIATIONS_FPS}, utils::{rect::Rect, timed_content_provider::TimedContentProvider}};

pub trait Tile: Clone {
    fn row(&self) -> u32;
    fn column(&self) -> u32;
}

pub trait SpriteTile: Tile {
    fn texture_source_rect(&self, variant: i32) -> Rect;
}

#[derive(Default)]
pub struct TileSet<T: Tile> {
    pub tiles: Vec<Vec<T>>,
    pub sheet_id: u32,
    sprite_counter: TimedContentProvider<i32>,
}

impl<T: Tile> TileSet<T> {
    pub fn empty() -> Self {
        Self::with_tiles(SPRITE_SHEET_BLANK, vec![])
    }

    pub fn with_tiles(sheet_id: u32, tiles: Vec<Vec<T>>) -> Self {
        Self { 
            tiles,
            sheet_id,
            sprite_counter: Self::content_provider()
        }
    }

    pub fn content_provider() -> TimedContentProvider<i32> {
        TimedContentProvider::new(Vec::from_iter(0..BIOME_NUMBER_OF_FRAMES), TILE_VARIATIONS_FPS)
    }

    pub fn update(&mut self, time_since_last_update: f32) {
        self.sprite_counter.update(time_since_last_update);
    }

    pub fn current_variant(&self, row: usize, col: usize) -> i32 {
        (*self.sprite_counter.current_frame() + row as i32 + col as i32) % BIOME_NUMBER_OF_FRAMES
    }
}

#[macro_export]
macro_rules! impl_tile {
    ($struct_name:ident) => {
        impl $crate::maps::tiles::Tile for $struct_name {            
            fn row(&self) -> u32 {
                self.row
            }
            
            fn column(&self) -> u32 {
                self.column
            }
        }
    }
}