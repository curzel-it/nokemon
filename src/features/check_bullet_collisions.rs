use crate::game_engine::{entity::Entity, world::World, world_state_update::WorldStateUpdate};

pub fn handle_collisions_for_bullet(bullet: &dyn Entity, world: &World) -> Vec<WorldStateUpdate> {
    let damage = -bullet.body().dp;

    return world.collisions
        .get(&bullet.id())
        .unwrap_or(&vec![])
        .iter()
        .map(|victim_id| WorldStateUpdate::IncreaseHp(*victim_id, damage))
        .collect();
}