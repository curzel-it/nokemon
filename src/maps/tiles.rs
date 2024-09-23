
use crate::{constants::{SPRITE_SHEET_BLANK, TILE_VARIATIONS_COUNT, TILE_VARIATIONS_FPS}, utils::{rect::Rect, timed_content_provider::TimedContentProvider}};

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
        TimedContentProvider::new(Vec::from_iter(0..TILE_VARIATIONS_COUNT), TILE_VARIATIONS_FPS)
    }

    pub fn update(&mut self, time_since_last_update: f32) {
        self.sprite_counter.update(time_since_last_update);
    }

    pub fn current_variant(&self, row: usize, col: usize) -> i32 {
        (*self.sprite_counter.current_frame() + row as i32 + col as i32) % TILE_VARIATIONS_COUNT
    }

    pub fn visible_tiles(&self, viewport: &Rect) -> Vec<&T> {
        let rows_count = self.tiles.len() as i32;
        let columns_count = self.tiles[0].len() as i32;

        let min_y = viewport.y - 2;
        let max_y = (viewport.y + viewport.h) + 4;
        let min_x = viewport.x - 2;
        let max_x = (viewport.x + viewport.w) + 4;

        let min_row = min_y.max(0).min(rows_count) as usize;
        let max_row = max_y.max(0).min(rows_count) as usize;
        let min_col = min_x.max(0).min(columns_count) as usize;
        let max_col = max_x.max(0).min(columns_count) as usize;

        let mut visible_tiles = Vec::new();

        for row in min_row..max_row {
            for col in min_col..max_col {
                let tile = &self.tiles[row][col];
                visible_tiles.push(tile);
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