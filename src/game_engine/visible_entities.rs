use std::collections::HashSet;

use crate::utils::rect::Rect;

use super::world::World;

pub fn compute_visible_entities(world: &World, viewport: &Rect) -> HashSet<u32> {
    world.entities.borrow()
        .values()
        .filter(|e| is_visible(&e.body().frame, viewport))
        .map(|e| e.id())
        .collect()
}

pub fn is_visible(rect: &Rect, viewport: &Rect) -> bool {
    rect.collides_with_rect(viewport)
}