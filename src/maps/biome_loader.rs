use crate::{constants::{SPRITE_SHEET_BLANK, TILE_SIZE}, game_engine::{entity::Entity, obstacles::StaticObstacle, world::World}, utils::rect::Rect};

use super::{biome_tiles::BiomeTile, tiles::TileSet};

impl World {    
    pub fn load_biome_tiles(&mut self, tiles: TileSet<BiomeTile>) {
        self.make_water_obstacles(&tiles);        
        self.bounds = Rect::new(0, 0, tiles.tiles[0].len() as u32, tiles.tiles.len() as u32);
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
                self.column,
                self.row, 
                self.width, 
                self.height
            )
        );
        Box::new(entity)
    }
}