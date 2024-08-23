use crate::{constants::{SPRITE_SHEET_BIOME_TILES, SPRITE_SHEET_BLANK, TILE_SIZE}, game_engine::{entity::Entity, obstacles::StaticObstacle, world::World}, levels::utils::level_biome_binary_path, utils::rect::Rect};

use super::{biome_tiles::{Biome, BiomeTile}, tiles::TileSet, worldgen::{deserialize_tiled_map, TileItem, TiledMap}};

impl World {    
    pub fn load_biome_tiles(&mut self) {
        let file_path = level_biome_binary_path(self.level_id);
        let map = deserialize_tiled_map(file_path);
        let matrix = to_biome_tile_matrix(&map);
        let groups = to_biome_tile_list(&map);
        make_water_obstacles(self, &groups);        
        let width = matrix[0].len() as f32 * TILE_SIZE;
        let height = matrix.len() as f32 * TILE_SIZE;
        self.bounds = Rect::new(0.0, 0.0, width, height);
        self.biome_tiles = TileSet::with_tiles(SPRITE_SHEET_BIOME_TILES, matrix);
    }
}

fn make_water_obstacles(world: &mut World, groups: &Vec<BiomeTile>) {
    let obstacles: Vec<Box<dyn Entity>> = groups
        .iter()
        .filter(|tile| tile.is_water())
        .map(|tile| tile.into_obstacle_entity())
        .collect();

    for obstacle in obstacles {
        world.add_entity(obstacle);
    };
}

impl BiomeTile {
    fn from_tile_item(item: &TileItem) -> Self {
        let tile_type = Biome::from_color(item.tile_type);
        let tile_up_type = Biome::from_color(item.tile_up_type);
        let tile_right_type = Biome::from_color(item.tile_right_type);
        let tile_down_type = Biome::from_color(item.tile_down_type);
        let tile_left_type = Biome::from_color(item.tile_left_type);
    
        let mut tile = Self {
            tile_type,
            column: item.column,
            row: item.row,
            width: item.width,
            height: item.height,
            tile_up_type,
            tile_right_type,
            tile_down_type,
            tile_left_type,
            texture_offset_x: 0.0,
            texture_offset_y: 0.0,
        };
        tile.setup_textures();
        tile
    }
}

fn to_biome_tile_matrix(tiled_map: &TiledMap) -> Vec<Vec<BiomeTile>> {
    tiled_map.tiles_matrix
        .iter()
        .map(|row| {
            row.iter()
                .map(BiomeTile::from_tile_item)
                .collect::<Vec<BiomeTile>>()
        })
        .collect()
}

fn to_biome_tile_list(tiled_map: &TiledMap) -> Vec<BiomeTile> {
    tiled_map.grouped_tiles
        .iter()
        .map(BiomeTile::from_tile_item)
        .collect()
}

impl BiomeTile {
    fn into_obstacle_entity(&self) -> Box<dyn Entity> {
        let entity = StaticObstacle::new(
            SPRITE_SHEET_BLANK,
            Rect::new(
                self.column as f32 * TILE_SIZE, 
                self.row as f32 * TILE_SIZE, 
                self.width as f32 * TILE_SIZE, 
                self.height as f32 * TILE_SIZE
            )
        );
        Box::new(entity)
    }
}