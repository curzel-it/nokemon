use std::collections::HashSet;

use crate::{constants::HERO_ENTITY_ID, utils::rect::Rect};

use super::world::World;

impl World {
    pub fn compute_visible_entities(&self, viewport: &Rect) -> HashSet<u32> {
        let min_row = viewport.y;
        let max_row = viewport.y + viewport.h;
        let min_col = viewport.x;
        let max_col = viewport.x + viewport.w;

        let mut ids: HashSet<u32> = self.entities.borrow().values()
            .filter(|e| {
                let frame = e.body().frame;
                frame.y + frame.h >= min_row &&
                frame.y <= max_row &&
                frame.x + frame.w >= min_col &&
                frame.x <= max_col
            })
            .map(|e| e.id())
            .collect();

        ids.insert(HERO_ENTITY_ID);
        ids
    }
}