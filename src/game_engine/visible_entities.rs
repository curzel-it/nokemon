use std::collections::HashSet;

use raylib::math::Rectangle;

use super::world::World;

pub fn compute_visible_entities(world: &World) -> HashSet<u32> {
    world.entities.borrow()
        .values()
        .filter(|e| is_visible(&e.body().frame, world))
        .map(|e| e.id())
        .collect()        
}

pub fn is_visible(rect: &Rectangle, world: &World) -> bool {
    world.camera_viewport.check_collision_recs(rect)
}