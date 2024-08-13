use image::{GenericImageView, Pixel};
use rand::Rng;
use raylib::math::Rectangle;

use crate::{constants::{TILE_SIZE, WORLD_BIOME_PATH}, game_engine::{entity::Entity, world::World}};

use super::{biome_tiles::{Biome, BiomeTile, BiomeTileSet}, tiles::{joined_tiles, Tile}};

impl World {
    pub fn load_biome_tiles(&mut self) {
        let (rows, columns, mut tiles) = parse_biome_map(WORLD_BIOME_PATH);
        integrate_borders_info(&mut tiles);
        make_water_obstacles(self, &tiles);        
        make_variations(&mut tiles);
        self.bounds = Rectangle::new(0.0, 0.0, columns as f32 * TILE_SIZE, rows as f32 * TILE_SIZE);        
        self.tiles = BiomeTileSet::with_tiles(tiles);
    }
}

fn make_variations(tiles: &mut Vec<Vec<BiomeTile>>) {
    for row in 0..tiles.len() {
        for col in 0..tiles[row].len() {
            let variant = rand::thread_rng().gen_range(0..10);
            tiles[row][col].variant = variant;
        }
    }
}

fn make_water_obstacles(world: &mut World, tiles: &Vec<Vec<BiomeTile>>) {
    for row in tiles {
        let obstacles: Vec<Box<dyn Entity>> = joined_water_tiles(row)
            .iter()
            .filter(|tile| tile.is_water())
            .map(|tile| tile.into_obstacle_entity(&world.entity_factory))
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
            let mut tile_up_type = Biome::Grass;
            let mut tile_right_type = Biome::Grass;
            let mut tile_down_type = Biome::Grass;
            let mut tile_left_type = Biome::Grass;

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
            current.tile_up_type = tile_up_type;
            current.tile_right_type = tile_right_type;
            current.tile_down_type = tile_down_type;
            current.tile_left_type = tile_left_type;
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
    joined_tiles(tiles).into_iter().filter(|t| t.is_water()).collect()
} 

#[cfg(test)]
mod tests {
    use crate::maps::biome_tiles::{Biome, COLOR_DESERT, COLOR_GRASS, COLOR_ROCK, COLOR_WATER};

    use super::*;

    #[test]
    fn test_single_water_tile() {
        let tiles = vec![
            BiomeTile::with_color_indeces(COLOR_WATER, 0, 0)
        ];

        let joined_tiles = joined_tiles(&tiles);

        assert_eq!(joined_tiles.len(), 1);
        assert_eq!(joined_tiles[0].tile_type, Biome::Water);
        assert_eq!(joined_tiles[0].width, 1);
    }

    #[test]
    fn test_multiple_non_contiguous_water_tiles() {
        let tiles = vec![
            BiomeTile::with_color_indeces(COLOR_WATER, 0, 0),
            BiomeTile::with_color_indeces(COLOR_GRASS, 1, 0),
            BiomeTile::with_color_indeces(COLOR_WATER, 2, 0),
        ];

        let joined_tiles = joined_tiles(&tiles);

        assert_eq!(joined_tiles.len(), 3);
        assert_eq!(joined_tiles[0].tile_type, Biome::Water);
        assert_eq!(joined_tiles[0].width, 1);
        assert_eq!(joined_tiles[1].tile_type, Biome::Grass);
        assert_eq!(joined_tiles[1].width, 1);
        assert_eq!(joined_tiles[2].tile_type, Biome::Water);
        assert_eq!(joined_tiles[2].width, 1);
    }

    #[test]
    fn test_contiguous_water_tiles() {
        let tiles = vec![
            BiomeTile::with_color_indeces(COLOR_WATER, 0, 0),
            BiomeTile::with_color_indeces(COLOR_WATER, 1, 0),
            BiomeTile::with_color_indeces(COLOR_WATER, 2, 0),
        ];

        let joined_tiles = joined_tiles(&tiles);

        assert_eq!(joined_tiles.len(), 1);
        assert_eq!(joined_tiles[0].tile_type, Biome::Water);
        assert_eq!(joined_tiles[0].width, 3);
    }

    #[test]
    fn test_mixed_tiles() {
        let tiles = vec![
            BiomeTile::with_color_indeces(COLOR_WATER, 0, 0),
            BiomeTile::with_color_indeces(COLOR_WATER, 1, 0),
            BiomeTile::with_color_indeces(COLOR_GRASS, 2, 0),
            BiomeTile::with_color_indeces(COLOR_WATER, 3, 0),
            BiomeTile::with_color_indeces(COLOR_WATER, 4, 0),
        ];

        let joined_tiles = joined_water_tiles(&tiles);

        assert_eq!(joined_tiles.len(), 2);
        assert_eq!(joined_tiles[0].tile_type, Biome::Water);
        assert_eq!(joined_tiles[0].width, 2);
        assert_eq!(joined_tiles[1].tile_type, Biome::Water);
        assert_eq!(joined_tiles[1].width, 2);
    }

    #[test]
    fn test_no_water_tiles() {
        let tiles = vec![
            BiomeTile::with_color_indeces(COLOR_GRASS, 0, 0),
            BiomeTile::with_color_indeces(COLOR_ROCK, 1, 0),
            BiomeTile::with_color_indeces(COLOR_DESERT, 2, 0),
        ];
        assert_eq!(joined_tiles(&tiles).len(), 3);
        assert_eq!(joined_water_tiles(&tiles).len(), 0);
    }
}
