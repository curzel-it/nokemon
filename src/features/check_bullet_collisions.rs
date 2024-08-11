use crate::game_engine::{entity::Entity, world::World, game_state_update::GameStateUpdate};

pub fn handle_collisions_for_bullet(bullet: &dyn Entity, world: &World) -> Vec<GameStateUpdate> {
    let damage = -bullet.body().dp;

    return world.collisions
        .get(&bullet.id())
        .unwrap_or(&vec![])
        .iter()
        .map(|victim_id| GameStateUpdate::IncreaseHp(*victim_id, damage))
        .collect();
}