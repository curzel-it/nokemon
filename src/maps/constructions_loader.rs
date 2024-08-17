use crate::{constants::TILE_SIZE, game_engine::{entity::Entity, obstacles::StaticObstacle, world::World}};

use super::{constructions_tiles::{Construction, ConstructionTile}, tiles::TileSet, worldgen::{deserialize_tiled_map, TileItem, TiledMap, WORLD_MAP_CONSTRUCTIONS_BIN}};

impl World {    
    pub fn load_construction_tiles(&mut self) {
        let map = deserialize_tiled_map(WORLD_MAP_CONSTRUCTIONS_BIN);
        let matrix = to_construction_tile_matrix(&map);
        let groups = to_construction_tile_list(&map);
        make_obstacles(self, &groups);        
        self.constructions_tiles = TileSet::with_tiles("tiles_constructions".to_owned(), matrix);
    }
}

fn make_obstacles(world: &mut World, groups: &Vec<ConstructionTile>) {
    let obstacles: Vec<Box<dyn Entity>> = groups
        .iter()
        .filter(|tile| tile.is_something())
        .map(|tile| tile.into_obstacle_entity())
        .collect();

    for obstacle in obstacles {
        world.add_entity(obstacle);
    };
}

impl ConstructionTile {
    fn from_tile_item(item: &TileItem) -> Self {
        let tile_type = Construction::from_color(item.tile_type);
        let tile_up_type = Construction::from_color(item.tile_up_type);
        let tile_right_type = Construction::from_color(item.tile_right_type);
        let tile_down_type = Construction::from_color(item.tile_down_type);
        let tile_left_type = Construction::from_color(item.tile_left_type);
    
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

fn to_construction_tile_matrix(tiled_map: &TiledMap) -> Vec<Vec<ConstructionTile>> {
    tiled_map.tiles_matrix
        .iter()
        .map(|row| {
            row.iter()
                .map(ConstructionTile::from_tile_item)
                .collect::<Vec<ConstructionTile>>()
        })
        .collect()
}

fn to_construction_tile_list(tiled_map: &TiledMap) -> Vec<ConstructionTile> {
    tiled_map.grouped_tiles
        .iter()
        .map(ConstructionTile::from_tile_item)
        .collect()
}

impl ConstructionTile {
    fn into_obstacle_entity(&self) -> Box<dyn Entity> {
        let entity = StaticObstacle::new(
            self.tile_type.sprite(),
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