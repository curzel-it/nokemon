use image::{GenericImageView, Pixel};
use raylib::math::Rectangle;

use crate::{constants::{BG_TILE_SIZE, WORLD_MAP_PATH}, entities::background_tile::BackgroundTileInfo};

use super::{entity::Entity, obstacles, tile_set::TileSet, world::World};

impl World {
    pub fn load_map(&mut self) {
        let (rows, columns, mut tiles) = parse_world_map(WORLD_MAP_PATH);
        integrate_borders_info(&mut tiles);
        make_obstacles(self, &tiles);
        self.bounds = Rectangle::new(0.0, 0.0, columns as f32 * BG_TILE_SIZE, rows as f32 * BG_TILE_SIZE);        
        self.tiles = TileSet::with_tiles(tiles);
    }
}

fn make_obstacles(world: &mut World, tiles: &Vec<Vec<BackgroundTileInfo>>) {
    let rows = tiles.len();
    let columns = tiles[0].len();
    let mut counter = 0;

    for row in 0..rows {
        for col in 0..columns {
            let tile = &tiles[row][col];
            
            if tile.is_water() && tile.has_ground_contact {
                let entity = tile.into_obstacle_entity(&world.entity_factory);
                world.add_entity(entity);
                counter += 1;
            }
        }
    }

    println!("Created {} borders", counter);
}

fn integrate_borders_info(tiles: &mut Vec<Vec<BackgroundTileInfo>>) {
    let rows = tiles.len();
    let columns = tiles[0].len();

    for row in 0..rows {
        for col in 0..columns {
            let ground_above = row != 0 && tiles[row-1][col].is_not_water();
            let ground_right = col != columns-1 && tiles[row][col+1].is_not_water();
            let ground_below = row != rows-1 && tiles[row+1][col].is_not_water();
            let ground_left = col != 0 && tiles[row][col-1].is_not_water();            
            let has_ground_contact = ground_above || ground_right || ground_below || ground_left;

            let water_above = row != 0 && !tiles[row-1][col].is_water();
            let water_right = col != columns-1 && !tiles[row][col+1].is_water();
            let water_below = row != rows-1 && !tiles[row+1][col].is_water();
            let water_left = col != 0 && !tiles[row][col-1].is_water();            
            let has_water_contact = water_above || water_right || water_below || water_left;
            
            let tile = &mut tiles[row][col];
            tile.has_ground_contact = has_ground_contact;
            tile.has_water_contact = has_water_contact;
        }
    }
}

fn parse_world_map(image_path: &str) -> (u32, u32, Vec<Vec<BackgroundTileInfo>>) {
    let img = image::open(image_path).expect("Failed to open image");
    let (width, height) = img.dimensions();

    let mut tiles: Vec<Vec<BackgroundTileInfo>> = Vec::new();

    for y in 0..height {
        let mut row: Vec<BackgroundTileInfo> = Vec::new();

        for x in 0..width {
            let pixel = img.get_pixel(x, y).to_rgb();
            let color_int = (pixel[0] as u32) << 16 | (pixel[1] as u32) << 8 | (pixel[2] as u32);            
            let tile = BackgroundTileInfo::new(color_int, x, y);
            row.push(tile);
        }
        tiles.push(row);
    }

    (width, height, tiles)
}
