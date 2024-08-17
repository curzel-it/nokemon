use crate::game_engine::{entity::Entity, world::World, state_updates::WorldStateUpdate};

pub fn handle_collisions_for_bullet(bullet: &dyn Entity, world: &World) -> Vec<WorldStateUpdate> {
    let damage = -bullet.body().dp;

    return world.collisions
        .get(&bullet.id())
        .unwrap_or(&vec![])
        .iter()
        .filter(|c| !c.are_same_faction)
        .map(|c| WorldStateUpdate::IncreaseHp(c.other_id, damage))
        .collect();
}