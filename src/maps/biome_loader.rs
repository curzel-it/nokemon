use crate::{constants::{SPRITE_SHEET_BLANK, TILE_SIZE}, game_engine::{entity::Entity, obstacles::StaticObstacle, world::World}, utils::rect::Rect};

use super::{biome_tiles::BiomeTile, tiles::TileSet};

impl World {    
    pub fn load_biome_tiles(&mut self, tiles: TileSet<BiomeTile>) {
        self.make_water_obstacles(&tiles);        
        let width = tiles.tiles[0].len() as f32 * TILE_SIZE;
        let height = tiles.tiles.len() as f32 * TILE_SIZE;
        self.bounds = Rect::new(0.0, 0.0, width, height);
        self.biome_tiles = tiles;
    }

    fn make_water_obstacles(&mut self, tiles: &TileSet<BiomeTile>) {
        let obstacles: Vec<Box<dyn Entity>> = tiles.tiles.iter().flatten()
            .filter(|tile| tile.is_water_obstacle())
            .map(|tile| tile.into_obstacle_entity())
            .collect();
    
        for obstacle in obstacles {
            self.add_entity(obstacle);
        };
    }
}

impl BiomeTile {
    fn is_water_obstacle(&self) -> bool {
        self.is_water() && !self.is_surrounded_by_water()
    }

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