use image::{GenericImageView, Pixel};
use raylib::math::Rectangle;

use crate::{constants::{TILE_SIZE, WORLD_MAP_BIOME}, game_engine::{entity::Entity, obstacles::StaticObstacle, world::World}};

use super::{biome_tiles::{group_biome_tiles, BiomeTile}, tiles::{Tile, TileSet}};

impl World {
    pub fn load_biome_tiles(&mut self) {
        let (rows, columns, mut tiles) = parse_biome_map(WORLD_MAP_BIOME);
        integrate_borders_info(&mut tiles);
        make_water_obstacles(self, &tiles);        
        self.bounds = Rectangle::new(0.0, 0.0, columns as f32 * TILE_SIZE, rows as f32 * TILE_SIZE);        
        self.biome_tiles = TileSet::with_tiles("tiles_biome".to_owned(), tiles);
    }
}

fn make_water_obstacles(world: &mut World, tiles: &Vec<Vec<BiomeTile>>) {
    for row in tiles {
        let obstacles: Vec<Box<dyn Entity>> = joined_water_tiles(row)
            .iter()
            .filter(|tile| tile.is_water())
            .map(|tile| tile.into_obstacle_entity())
            .collect();

        for obstacle in obstacles {
            world.add_entity(obstacle);
        };
    }
}

fn integrate_borders_info(tiles: &mut Vec<Vec<BiomeTile>>) {
    let rows = tiles.len();
    let columns = tiles[0].len();

    for row in 0..rows {
        for col in 0..columns {
            let current_biome = tiles[row][col].tile_type;
            let mut tile_up_type = current_biome;
            let mut tile_right_type = current_biome;
            let mut tile_down_type = current_biome;
            let mut tile_left_type = current_biome;

            if row > 0 {
                tile_up_type = tiles[row-1][col].tile_type;
            }
            if col < columns - 1 {
                tile_right_type = tiles[row][col+1].tile_type;
            }
            if row < rows - 1 {
                tile_down_type = tiles[row+1][col].tile_type;
            }
            if col > 0 {
                tile_left_type = tiles[row][col-1].tile_type;
            }

            let current = &mut tiles[row][col];
            current.setup_neighbors(
                tile_up_type,
                tile_right_type,
                tile_down_type,
                tile_left_type
            );
        }
    }
}

fn parse_biome_map(image_path: &str) -> (u32, u32, Vec<Vec<BiomeTile>>) {
    let img = image::open(image_path).expect("Failed to open image");
    let (width, height) = img.dimensions();

    let mut tiles: Vec<Vec<BiomeTile>> = Vec::new();

    for y in 0..height {
        let mut row: Vec<BiomeTile> = Vec::new();

        for x in 0..width {
            let pixel = img.get_pixel(x, y).to_rgb();
            let color_int = (pixel[0] as u32) << 16 | (pixel[1] as u32) << 8 | (pixel[2] as u32);            
            let tile = BiomeTile::with_color_indeces(color_int, x, y);
            row.push(tile);
        }
        tiles.push(row);
    }

    (width, height, tiles)
}

fn joined_water_tiles(tiles: &Vec<BiomeTile>) -> Vec<BiomeTile> {
    group_biome_tiles(tiles).into_iter().filter(|t| t.is_water()).collect()
} 

impl BiomeTile {
    fn into_obstacle_entity(&self) -> Box<dyn Entity> {
        let entity = StaticObstacle::new(
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
}