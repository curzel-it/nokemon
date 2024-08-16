use std::collections::HashSet;

use common_macros::hash_set;
use raylib::math::Rectangle;

use super::world::World;

pub fn compute_visible_entities_and_collision_candidates(world: &World) -> (HashSet<u32>, HashSet<u32>) {
    let mut collidable_ids: HashSet<u32> = hash_set!();
    let mut visible_ids: HashSet<u32> = hash_set!();

    world.entities.borrow()
        .values()
        .filter(|e| is_visible(&e.body().frame, world))
        .for_each(|entity| {
            let id = entity.id();
            let body = entity.body();
    
            visible_ids.insert(id);
    
            if body.requires_collision_detection {
                collidable_ids.insert(id);
            }
        });    

    (visible_ids, collidable_ids)
}

pub fn is_visible(rect: &Rectangle, world: &World) -> bool {
    world.camera_viewport.check_collision_recs(rect)
}