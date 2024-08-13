use std::{fs::File, io::BufReader};

use image::{GenericImageView, Pixel};
use rand::Rng;
use raylib::math::Rectangle;
use serde_json::from_reader;

use crate::{constants::{TILE_SIZE, WORLD_MAP_PATH}, entities::background_tile::{BackgroundTileInfo, BackgroundTileType}, game_engine::entity::Entity};

use super::{tile_set::TileSet, world::World};

impl World {
    pub fn load_map(&mut self) {
        let mut tiles = parse_world_map(WORLD_MAP_PATH);
        let rows = tiles.len();
        let columns = tiles[0].len();
        integrate_borders_info(&mut tiles);
        make_water_obstacles(self, &tiles);        
        make_variations(&mut tiles);
        self.bounds = Rectangle::new(0.0, 0.0, columns as f32 * TILE_SIZE, rows as f32 * TILE_SIZE);        
        self.tiles = TileSet::with_tiles(tiles);
    }
}

fn make_variations(tiles: &mut Vec<Vec<BackgroundTileInfo>>) {
    for row in 0..tiles.len() {
        for col in 0..tiles[row].len() {
            let variant = rand::thread_rng().gen_range(0..10);
            tiles[row][col].variant = variant;
        }
    }
}

fn make_water_obstacles(world: &mut World, tiles: &Vec<Vec<BackgroundTileInfo>>) {
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

fn integrate_borders_info(tiles: &mut Vec<Vec<BackgroundTileInfo>>) {
    let rows = tiles.len();
    let columns = tiles[0].len();

    for row in 0..rows {
        for col in 0..columns {
            let mut tile_up_type = BackgroundTileType::Grass;
            let mut tile_right_type = BackgroundTileType::Grass;
            let mut tile_down_type = BackgroundTileType::Grass;
            let mut tile_left_type = BackgroundTileType::Grass;

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

fn parse_world_map(json_path: &str) -> Vec<Vec<BackgroundTileInfo>> {
    let file = File::open(json_path).expect("Failed to open JSON file");
    let reader = BufReader::new(file);
    
    let tiles: Vec<BackgroundTileInfo> = from_reader(reader).expect("Failed to parse JSON");

    let columns = tiles.iter().max_by_key(|tile| tile.column).map(|tile| tile.column + 1).unwrap_or(0);
    let rows = tiles.iter().max_by_key(|tile| tile.row).map(|tile| tile.row + 1).unwrap_or(0);

    let mut tile_matrix: Vec<Vec<BackgroundTileInfo>> = vec![vec![BackgroundTileInfo::default(); columns as usize]; rows as usize];

    for tile in tiles {
        tile_matrix[tile.row as usize][tile.column as usize] = tile;
    }

    tile_matrix
}

fn joined_water_tiles(tiles: &Vec<BackgroundTileInfo>) -> Vec<BackgroundTileInfo> {
    joined_tiles(tiles).into_iter().filter(|t| t.is_water()).collect()
} 

fn joined_tiles(tiles: &Vec<BackgroundTileInfo>) -> Vec<BackgroundTileInfo> {
    let mut joined: Vec<BackgroundTileInfo> = vec![];    
    let mut previous = tiles[0];
    
    for i in 1..tiles.len() {
        let current = tiles[i];
        
        if current.tile_type == previous.tile_type {
            previous.width += 1;
        } else {
            joined.push(previous);
            previous = current;
        }
    }
    joined.push(previous);

    joined
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::background_tile::{BackgroundTileInfo, BackgroundTileType};

    #[test]
    fn test_single_water_tile() {
        let tiles = vec![
            BackgroundTileInfo::with_type_indeces(BackgroundTileType::Water, 0, 0)
        ];

        let joined_tiles = joined_tiles(&tiles);

        assert_eq!(joined_tiles.len(), 1);
        assert_eq!(joined_tiles[0].tile_type, BackgroundTileType::Water);
        assert_eq!(joined_tiles[0].width, 1);
    }

    #[test]
    fn test_multiple_non_contiguous_water_tiles() {
        let tiles = vec![
            BackgroundTileInfo::with_type_indeces(BackgroundTileType::Water, 0, 0),
            BackgroundTileInfo::with_type_indeces(BackgroundTileType::Grass, 1, 0),
            BackgroundTileInfo::with_type_indeces(BackgroundTileType::Water, 2, 0),
        ];

        let joined_tiles = joined_tiles(&tiles);

        assert_eq!(joined_tiles.len(), 3);
        assert_eq!(joined_tiles[0].tile_type, BackgroundTileType::Water);
        assert_eq!(joined_tiles[0].width, 1);
        assert_eq!(joined_tiles[1].tile_type, BackgroundTileType::Grass);
        assert_eq!(joined_tiles[1].width, 1);
        assert_eq!(joined_tiles[2].tile_type, BackgroundTileType::Water);
        assert_eq!(joined_tiles[2].width, 1);
    }

    #[test]
    fn test_contiguous_water_tiles() {
        let tiles = vec![
            BackgroundTileInfo::with_type_indeces(BackgroundTileType::Water, 0, 0),
            BackgroundTileInfo::with_type_indeces(BackgroundTileType::Water, 1, 0),
            BackgroundTileInfo::with_type_indeces(BackgroundTileType::Water, 2, 0),
        ];

        let joined_tiles = joined_tiles(&tiles);

        assert_eq!(joined_tiles.len(), 1);
        assert_eq!(joined_tiles[0].tile_type, BackgroundTileType::Water);
        assert_eq!(joined_tiles[0].width, 3);
    }

    #[test]
    fn test_mixed_tiles() {
        let tiles = vec![
            BackgroundTileInfo::with_type_indeces(BackgroundTileType::Water, 0, 0),
            BackgroundTileInfo::with_type_indeces(BackgroundTileType::Water, 1, 0),
            BackgroundTileInfo::with_type_indeces(BackgroundTileType::Grass, 2, 0),
            BackgroundTileInfo::with_type_indeces(BackgroundTileType::Water, 3, 0),
            BackgroundTileInfo::with_type_indeces(BackgroundTileType::Water, 4, 0),
        ];

        let joined_tiles = joined_water_tiles(&tiles);

        assert_eq!(joined_tiles.len(), 2);
        assert_eq!(joined_tiles[0].tile_type, BackgroundTileType::Water);
        assert_eq!(joined_tiles[0].width, 2);
        assert_eq!(joined_tiles[1].tile_type, BackgroundTileType::Water);
        assert_eq!(joined_tiles[1].width, 2);
    }

    #[test]
    fn test_no_water_tiles() {
        let tiles = vec![
            BackgroundTileInfo::with_type_indeces(BackgroundTileType::Grass, 0, 0),
            BackgroundTileInfo::with_type_indeces(BackgroundTileType::Rock, 1, 0),
            BackgroundTileInfo::with_type_indeces(BackgroundTileType::Desert, 2, 0),
        ];
        assert_eq!(joined_tiles(&tiles).len(), 3);
        assert_eq!(joined_water_tiles(&tiles).len(), 0);
    }
}
