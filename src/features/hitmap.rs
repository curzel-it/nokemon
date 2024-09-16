use crate::{constants::HERO_ENTITY_ID, entities::species::EntityType, game_engine::{entity::EntityId, world::World}};

pub type Hitmap = Vec<Vec<bool>>;
pub type EntityIdsMap = Vec<Vec<EntityId>>;
pub type WeightsMap = Vec<Vec<i32>>;

impl World {    
    pub fn update_hitmaps(&mut self) {
        (self.hitmap, self.entities_map, self.weights_map) = self.compute_hitmap();
    }    

    #[allow(clippy::needless_range_loop)]
    fn compute_hitmap(&self) -> (Hitmap, EntityIdsMap, WeightsMap) {
        let entities = self.entities.borrow();

        let mut hitmap = vec![vec![false; self.bounds.w as usize]; self.bounds.h as usize];
        let mut idsmap = vec![vec![0; self.bounds.w as usize]; self.bounds.h as usize];
        let mut weightsmap = vec![vec![0; self.bounds.w as usize]; self.bounds.h as usize];

        for (rindex, rid) in &self.visible_entities {
            let (index, id) = (*rindex, *rid);
            let entity = &entities[index];
            let col = entity.frame.x as usize;
            
            let (row, height) = if entity.frame.h == 1 { 
                (entity.frame.y as usize, 1) 
            } else { 
                (entity.frame.y  as usize + 1, (entity.frame.h as usize).saturating_sub(1)) 
            };

            for offset_x in 0..entity.frame.w as usize {
                for offset_y in 0..height {
                    if entity.is_rigid && id != HERO_ENTITY_ID {                
                        hitmap[row + offset_y][col + offset_x] = true;
                    }
                    if entity.entity_type.has_weight() {                
                        weightsmap[row + offset_y][col + offset_x] += 1;
                    }
                    idsmap[row + offset_y][col + offset_x] = id;
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

        (hitmap, idsmap, weightsmap)
    }
}

impl EntityType {
    fn has_weight(&self) -> bool {
        !matches!(self, EntityType::PressurePlate)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::{entities::{known_species::SPECIES_NPC_OLD_MAN, species::make_entity_by_species}, maps::{biome_tiles::{Biome, BiomeTile}, constructions_tiles::ConstructionTile}, utils::{ids::get_next_id, rect::Rect}};
    
    #[test]
    fn test_hitmap_with_rigid_entity_excludes_top_row() {
        let mut world = World::new(get_next_id());        
        let mut npc = make_entity_by_species(SPECIES_NPC_OLD_MAN);
        npc.frame.x = 5;
        npc.frame.y = 5;
        world.add_entity(npc);
        world.visible_entities = world.compute_visible_entities(&Rect::square_from_origin(20));
        
        let (hitmap, _, _) = world.compute_hitmap();
        println!("{:#?}", world.visible_entities);
        assert!(!hitmap[5][5]);
        assert!(hitmap[6][5]);
    }

    #[test]
    fn test_hitmap_ignores_non_rigid_entity() {
        let mut world = World::new(get_next_id());
        let mut npc = make_entity_by_species(SPECIES_NPC_OLD_MAN);
        npc.frame = Rect::new(5, 5, 2, 2);
        npc.is_rigid = false;
        
        world.add_entity(npc);
        world.compute_visible_entities(&Rect::square_from_origin(20));
        
        let (hitmap, _, _) = world.compute_hitmap();
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
        
        let (hitmap, _, _) = world.compute_hitmap();

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
        
        let (hitmap, _, _) = world.compute_hitmap();

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
