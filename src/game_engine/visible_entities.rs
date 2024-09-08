use std::collections::HashSet;

use crate::{constants::HERO_ENTITY_ID, utils::rect::Rect};

use super::world::World;

impl World {
    pub fn compute_visible_entities(&self, viewport: &Rect) -> HashSet<(usize, u32)> {
        let min_row = viewport.y;
        let max_row = viewport.y + viewport.h;
        let min_col = viewport.x;
        let max_col = viewport.x + viewport.w;

        self.entities.borrow().iter()
            .enumerate()
            .filter(|(_, e)| {
                let frame = e.body().frame;
                let is_inside_viewport = frame.y + frame.h >= min_row &&
                    frame.y <= max_row &&
                    frame.x + frame.w >= min_col &&
                    frame.x <= max_col;
                e.id() == HERO_ENTITY_ID || is_inside_viewport
            })
            .map(|(index, e)| (index, e.id()))
            .collect()
    }
}