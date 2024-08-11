use image::{GenericImageView, Pixel};
use rand::Rng;
use raylib::math::Rectangle;

use crate::{constants::{BG_TILE_SIZE, WORLD_MAP_PATH}, entities::background_tile::BackgroundTileType};

use super::game::Game;

impl Game {
    pub fn load_map(&mut self) {
        let (rows, columns, tiles) = parse_world_map(WORLD_MAP_PATH);
        self.bounds = Rectangle::new(0.0, 0.0, columns as f32 * BG_TILE_SIZE, rows as f32 * BG_TILE_SIZE);

        self.tiles = tiles
            .iter()
            .map(|(coords, tile_type)| {
                self.entity_factory.build_background_tile(
                    tile_type,
                    rand::thread_rng().gen_range(0..10),
                    coords.1,
                    coords.0,
                )
            })
            .collect();
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

fn parse_world_map(image_path: &str) -> (u32, u32, Vec<((u32, u32), BackgroundTileType)>) {
    let img = image::open(image_path).expect("Failed to open image");
    let (width, height) = img.dimensions();

    let mut tiles = Vec::new();

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y).to_rgb();
            let color_int = (pixel[0] as u32) << 16 | (pixel[1] as u32) << 8 | (pixel[2] as u32);

            if let Some(tile_type) = BackgroundTileType::from_color(color_int) {
                tiles.push(((x, y), tile_type));
            }
        }
    }

    (width, height, tiles)
}
