use crate::game_engine::{entity::EntityId, world::World};

pub type EntityIdsMap = Vec<Vec<EntityId>>;

impl World {
    #[allow(clippy::needless_range_loop)]
    pub fn compute_ids_map(&self) -> EntityIdsMap {
        vec![vec![0; self.bounds.w as usize]; self.bounds.h as usize]
    }
}
