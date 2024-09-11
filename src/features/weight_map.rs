use crate::{entities::species::EntityType, game_engine::world::World};

pub type WeightMap = Vec<Vec<u32>>;

impl World {
    #[allow(clippy::needless_range_loop)]
    pub fn compute_weight_map(&self) -> WeightMap {
        let mut weight_map = vec![vec![0; self.bounds.w as usize]; self.bounds.h as usize];
        let entities = self.entities.borrow();

        for (index, _) in &self.visible_entities {
            let entity = &entities[*index];
            
            if !entity.entity_type.has_weight() {
                continue
            }

            let col = entity.frame.x as usize;

            let (row, height) = if entity.frame.h == 1 { 
                (entity.frame.y as usize, 1) 
            } else { 
                (entity.frame.y  as usize + 1, entity.frame.h as usize - 1) 
            };

            for offset_x in 0..entity.frame.w as usize {
                for offset_y in 0..height {
                    weight_map[row + offset_y][col + offset_x] += 1;
                }                    
            }
        }
        weight_map
    }
}

impl EntityType {
    fn has_weight(&self) -> bool {
        match self {
            EntityType::PressurePlate => false,
            _ => true
        }
    }
}