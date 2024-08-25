use std::collections::HashSet;

use uuid::Uuid;

use crate::utils::rect::Rect;

use super::world::World;

pub fn compute_visible_entities(world: &World, viewport: &Rect) -> HashSet<Uuid> {
    let min_row = viewport.y;
    let max_row = viewport.y + viewport.h;
    let min_col = viewport.x;
    let max_col = viewport.x + viewport.w;

    world.entities.borrow().values()
        .filter(|e| {
            e.body().frame.y >= min_row &&
            e.body().frame.y <= max_row &&
            e.body().frame.x >= min_col &&
            e.body().frame.x <= max_col
        })
        .map(|e| e.id())
        .collect()
}