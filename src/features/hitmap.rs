use crate::{constants::HERO_ENTITY_ID, game_engine::world::World};

pub type Hitmap = Vec<Vec<bool>>;

impl World {
    #[allow(clippy::needless_range_loop)]
    pub fn compute_hitmap(&self) -> Hitmap {
        let mut hitmap = vec![vec![false; self.bounds.w as usize]; self.bounds.h as usize];
        let entities = self.entities.borrow();

        for (index, id) in &self.visible_entities {
            if *id == HERO_ENTITY_ID {
                continue;
            }
            let entity = &entities[*index];

            if entity.is_rigid {                
                let col = entity.frame.x as usize;
                
                let (row, height) = if entity.frame.h == 1 { 
                    (entity.frame.y as usize, 1) 
                } else { 
                    (entity.frame.y  as usize + 1, entity.frame.h as usize - 1) 
                };

                for offset_x in 0..entity.frame.w as usize {
                    for offset_y in 0..height {
                        hitmap[row + offset_y][col + offset_x] = true;
                    }                    
                }
            }
        }

        if !self.biome_tiles.tiles.is_empty() {
            let rows_count = self.bounds.h;
            let columns_count = self.bounds.w;

            let min_y = self.cached_hero_props.frame.y - 2;
            let max_y = (self.cached_hero_props.frame.y + self.cached_hero_props.frame.h) + 4;
            let min_x = self.cached_hero_props.frame.x - 2;
            let max_x = (self.cached_hero_props.frame.x + self.cached_hero_props.frame.w) + 4;

            let min_row = min_y.max(0).min(rows_count) as usize;
            let max_row = max_y.max(0).min(rows_count) as usize;
            let min_col = min_x.max(0).min(columns_count) as usize;
            let max_col = max_x.max(0).min(columns_count) as usize;

            for row in min_row..max_row {
                for col in min_col..max_col {
                    let is_biome_obstacle = self.biome_tiles.tiles[row][col].is_obstacle();
                    let is_construction_obstacle = self.constructions_tiles.tiles[row][col].is_obstacle();
                    hitmap[row][col] = hitmap[row][col] || is_biome_obstacle || is_construction_obstacle;
                }
            }
        }

        hitmap
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{entities::{npcs::NpcType, species::Species}, maps::{biome_tiles::{Biome, BiomeTile}, constructions_tiles::ConstructionTile}, utils::{ids::get_next_id, rect::Rect}};
    
    #[test]
    fn test_hitmap_with_rigid_entity_excludes_top_row() {
        let mut world = World::new(get_next_id());        
        let mut npc = Species::Npc(NpcType::OldMan).make_entity();
        npc.frame.x = 5;
        npc.frame.y = 5;
        world.add_entity(npc);
        world.visible_entities = world.compute_visible_entities(&Rect::square_from_origin(20));
        
        let hitmap = world.compute_hitmap();
        println!("{:#?}", world.visible_entities);
        assert!(!hitmap[5][5]);
        assert!(hitmap[6][5]);
    }

    #[test]
    fn test_hitmap_ignores_non_rigid_entity() {
        let mut world = World::new(get_next_id());
        let mut npc = Species::Npc(NpcType::OldMan).make_entity();
        npc.frame = Rect::new(5, 5, 2, 2);
        npc.is_rigid = false;
        
        world.add_entity(npc);
        world.compute_visible_entities(&Rect::square_from_origin(20));
        
        let hitmap = world.compute_hitmap();
        assert!(!hitmap[6][5]);
        assert!(!hitmap[6][6]);
        assert!(!hitmap[5][5]);
        assert!(!hitmap[5][6]);
    }

    #[test]
    fn test_hitmap_with_biome_tiles_nothing_still_hits() {
        let mut world = World::new(get_next_id());
        world.bounds = Rect::new(0, 0, 10, 10);
        world.cached_hero_props.frame = Rect::new(4, 4, 2, 2);
        
        world.constructions_tiles.tiles = vec![vec![ConstructionTile::from_data(0, 0, '0'); 10]; 10];
        world.biome_tiles.tiles = vec![vec![BiomeTile::from_data(0, 0, '0'); 10]; 10];
        
        let hitmap = world.compute_hitmap();

        assert!(hitmap[4][4]);
        assert!(hitmap[5][5]);
        assert!(hitmap[6][6]);
    }

    #[test]
    fn test_hitmap_with_biome_tiles() {
        let mut world = World::new(get_next_id());
        world.bounds = Rect::new(0, 0, 10, 10);
        world.cached_hero_props.frame = Rect::new(4, 4, 2, 2);
        
        world.constructions_tiles.tiles = vec![vec![ConstructionTile::from_data(0, 0, '0'); 10]; 10];
        world.biome_tiles.tiles = vec![vec![BiomeTile::from_data(0, 0, '1'); 10]; 10];
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
