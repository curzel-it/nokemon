use image::{GenericImageView, Pixel};
use rand::Rng;
use raylib::math::Rectangle;

use crate::{constants::{BG_TILE_SIZE, WORLD_MAP_PATH}, entities::background_tile::{BackgroundTile, BackgroundTileType}};

use super::{entity_factory::EntityFactory, world::World, tile_set::TileSet};

impl World {
    pub fn load_map(&mut self) {
        let (rows, columns, tiles) = parse_world_map(WORLD_MAP_PATH, &self.entity_factory);
        self.bounds = Rectangle::new(0.0, 0.0, columns as f32 * BG_TILE_SIZE, rows as f32 * BG_TILE_SIZE);
        self.tiles = TileSet::with_tiles(tiles);
    }
}

impl BackgroundTileType {
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

fn parse_world_map(image_path: &str, entity_factory: &EntityFactory) -> (u32, u32, Vec<Vec<BackgroundTile>>) {
    let img = image::open(image_path).expect("Failed to open image");
    let (width, height) = img.dimensions();

    let mut tiles: Vec<Vec<BackgroundTile>> = Vec::new();

    for y in 0..height {
        let mut row: Vec<BackgroundTile> = Vec::new();

        for x in 0..width {
            let pixel = img.get_pixel(x, y).to_rgb();
            let color_int = (pixel[0] as u32) << 16 | (pixel[1] as u32) << 8 | (pixel[2] as u32);
            let tile_type = BackgroundTileType::from_color(color_int).unwrap_or(BackgroundTileType::Desert);
            let index = rand::thread_rng().gen_range(0..10);
            let tile = entity_factory.build_background_tile(&tile_type, index, y, x);
            row.push(tile);
        }
        tiles.push(row);
    }

    (width, height, tiles)
}
