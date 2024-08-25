use crate::{constants::SPRITE_SHEET_BLANK, game_engine::{entity::Entity, obstacles::StaticObstacle, world::World}, utils::rect::Rect};

use super::{constructions_tiles::ConstructionTile, tiles::TileSet};

impl World {    
    pub fn load_construction_tiles(&mut self, tiles: TileSet<ConstructionTile>) {
        self.make_obstacles(&tiles);        
        self.constructions_tiles = tiles;
    }

    fn make_obstacles(&mut self, tiles: &TileSet<ConstructionTile>) {
        let obstacles: Vec<Box<dyn Entity>> = tiles.tiles.iter().flatten()
            .filter(|tile| tile.is_something())
            .map(|tile| tile.into_obstacle_entity())
            .collect();
    
        for obstacle in obstacles {
            self.add_entity(obstacle);
        };
    }
}

impl ConstructionTile {
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