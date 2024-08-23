use crate::{constants::{SPRITE_SHEET_BLANK, TILE_SIZE, TILE_VARIATIONS_COUNT, TILE_VARIATIONS_FPS}, utils::{rect::Rect, timed_content_provider::TimedContentProvider}};

pub trait Tile: Clone {
    fn row(&self) -> u32;
    fn column(&self) -> u32;
}

pub trait SpriteTile: Tile {
    fn texture_source_rect(&self, variant: u32) -> Rect;
}

pub struct TileSet<T: Tile> {
    pub tiles: Vec<Vec<T>>,
    pub sheet_id: u32,
    sprite_counter: TimedContentProvider<u32>,
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

    pub fn content_provider() -> TimedContentProvider<u32> {
        TimedContentProvider::new(Vec::from_iter(0..TILE_VARIATIONS_COUNT), TILE_VARIATIONS_FPS)
    }

    pub fn update(&mut self, time_since_last_update: f32) {
        self.sprite_counter.update(time_since_last_update);
    }

    pub fn current_variant(&self, row: u32, col: u32) -> u32 {
        (*self.sprite_counter.current_frame() + row + col) % TILE_VARIATIONS_COUNT
    }

    pub fn visible_tiles(&self, viewport: &Rect) -> Vec<&T> {
        let first_row = (viewport.y / TILE_SIZE).floor() as usize;
        let rows = (viewport.h / TILE_SIZE).ceil() as usize + 1;
        let first_col = (viewport.x / TILE_SIZE).floor() as usize;
        let cols = (viewport.w / TILE_SIZE).ceil() as usize + 1;

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