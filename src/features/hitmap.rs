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

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{entities::npc::{Npc, NpcType}, game_engine::entity_body::EmbodiedEntity, maps::{biome_tiles::{Biome, BiomeTile}, constructions_tiles::ConstructionTile}, utils::{ids::get_next_id, rect::Rect}};
    
    
    #[test]
    fn test_hitmap_with_rigid_entity_excludes_top_row() {
        let mut world = World::new(get_next_id());        
        let mut npc = Npc::new(NpcType::OldMan);
        npc.body_mut().frame = Rect::new(5, 5, 2, 2);         
        let id = world.add_entity(Box::new(npc));
        world.visible_entities.insert(id);
        
        let hitmap = world.compute_hitmap();
        assert!(hitmap[6][5]);
        assert!(hitmap[6][6]);
        assert!(!hitmap[5][5]);
        assert!(!hitmap[5][6]);
    }

    #[test]
    fn test_hitmap_ignores_non_rigid_entity() {
        let mut world = World::new(get_next_id());
        let mut npc = Npc::new(NpcType::OldMan);
        npc.body_mut().frame = Rect::new(5, 5, 2, 2);
        npc.body_mut().is_rigid = false;
        
        let id = world.add_entity(Box::new(npc));
        world.visible_entities.insert(id);
        
        let hitmap = world.compute_hitmap();
        assert!(!hitmap[6][5]);
        assert!(!hitmap[6][6]);
        assert!(!hitmap[5][5]);
        assert!(!hitmap[5][6]);
    }

    #[test]
    fn test_hitmap_with_biome_tiles() {
        let mut world = World::new(get_next_id());
        world.bounds = Rect::new(0, 0, 10, 10);
        world.cached_hero_props.frame = Rect::new(4, 4, 2, 2);
        
        world.constructions_tiles.tiles = vec![vec![ConstructionTile::from_data(0, 0, 0); 10]; 10];
        world.biome_tiles.tiles = vec![vec![BiomeTile::from_data(0, 0, 0); 10]; 10];
        world.biome_tiles.tiles[5][5].tile_type = Biome::Water;
        
        let hitmap = world.compute_hitmap();

        assert!(!hitmap[4][4]);
        assert!(!hitmap[4][5]);
        assert!(!hitmap[4][6]);

        assert!(!hitmap[5][4]);
        assert!(hitmap[5][5]);
        assert!(!hitmap[5][6]);

        assert!(!hitmap[6][4]);
        assert!(!hitmap[6][5]);
        assert!(!hitmap[6][6]);
    }
}
