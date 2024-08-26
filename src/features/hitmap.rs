use crate::{constants::HERO_ENTITY_ID, game_engine::world::World};

pub type Hitmap = Vec<Vec<bool>>;

impl World {
    #[allow(clippy::needless_range_loop)]
    pub fn compute_hitmap(&self) -> Hitmap {
        let mut hitmap = vec![vec![false; self.bounds.w as usize]; self.bounds.h as usize];
        let entities = self.entities.borrow();

        for id in &self.visible_entities {
            if *id == HERO_ENTITY_ID {
                continue;
            }
            if let Some(entity) = entities.get(id) {
                if entity.body().is_rigid {                
                    let frame = entity.body().frame;
                    let col = frame.x as usize;
                    
                    let (row, height) = if frame.h == 1 { 
                        (frame.y as usize, 1) 
                    } else { 
                        (frame.y  as usize + 1, frame.h as usize - 1) 
                    };

                    for offset_x in 0..frame.w as usize {
                        for offset_y in 0..height as usize {
                            hitmap[row + offset_y][col + offset_x] = true;
                        }                    
                    }
                }
            }
        }

        if !self.biome_tiles.tiles.is_empty() {
            let rows_count = self.bounds.h as i32;
            let columns_count = self.bounds.w as i32;

            let min_y = self.cached_hero_props.frame.y as i32 - 2;
            let max_y = (self.cached_hero_props.frame.y + self.cached_hero_props.frame.h) as i32 + 4;
            let min_x = self.cached_hero_props.frame.x as i32 - 2;
            let max_x = (self.cached_hero_props.frame.x + self.cached_hero_props.frame.w) as i32 + 4;

            let min_row = min_y.max(0).min(rows_count) as usize;
            let max_row = max_y.max(0).min(rows_count) as usize;
            let min_col = min_x.max(0).min(columns_count) as usize;
            let max_col = max_x.max(0).min(columns_count) as usize;

            for row in min_row..max_row {
                for col in min_col..max_col {
                    let is_water = self.biome_tiles.tiles[row][col].is_water();
                    let is_construction = self.constructions_tiles.tiles[row][col].is_something();
                    hitmap[row][col] = hitmap[row][col] || is_water || is_construction;
                }
            }
        }

        hitmap
    }
}
